use rayon::prelude::*;

use crate::Ray;

use super::aabb::*;
use super::mesh::*;
use super::{Geometry, RayHit};

#[derive(Debug)]
pub enum KdTree {
    Leaf {
        bounds: AABB,
        tris: Vec<Triangle>,
    },
    Node {
        bounds: AABB,
        left: Box<KdTree>,
        right: Box<KdTree>,
    },
}

impl KdTree {
    pub fn build(bounds: AABB, tris: Vec<Triangle>) -> Self {
        let cost = cost(&bounds, tris.len());
        let xs = tris
            .par_iter()
            .flat_map(Triangle::par_iter)
            .map(|v| bounds.split_x(v.pos.x));
        let ys = tris
            .par_iter()
            .flat_map(Triangle::par_iter)
            .map(|v| bounds.split_y(v.pos.y));
        let zs = tris
            .par_iter()
            .flat_map(Triangle::par_iter)
            .map(|v| bounds.split_z(v.pos.z));
        let min_split = xs
            .chain(ys)
            .chain(zs)
            .map(|(left, right)| (sah(&tris, &left, &right), left, right))
            .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Tried to compare NaN"));
        match min_split {
            Some((min_cost, left, right)) => {
                if min_cost < cost {
                    let (left_tris, right_tris) = partition(tris.iter(), &left, &right);
                    KdTree::Node {
                        bounds,
                        left: Box::new(KdTree::build(left, left_tris)),
                        right: Box::new(KdTree::build(right, right_tris)),
                    }
                } else {
                    KdTree::Leaf { bounds, tris }
                }
            }
            _ => KdTree::Leaf { bounds, tris },
        }
    }
}

fn sah<'a, I: IntoIterator<Item = &'a Triangle>>(tris: I, left: &AABB, right: &AABB) -> f32 {
    const TRAVERSAL_COST: f32 = 1.0;
    let (left_count, right_count) = split_count(tris.into_iter(), left, right);
    let left_cost = cost(left, left_count);
    let right_cost = cost(right, right_count);
    TRAVERSAL_COST + left_cost + right_cost
}

fn cost(aabb: &AABB, num_tris: usize) -> f32 {
    const INTERSECT_COST: f32 = 2.0;
    INTERSECT_COST * aabb.surface_area() * num_tris as f32
}

fn split_count<'a, I: Iterator<Item = &'a Triangle>>(
    tris: I,
    left: &AABB,
    right: &AABB,
) -> (usize, usize) {
    partition_by(|| 0, |accum, _| *accum += 1, tris, left, right)
}

fn partition_by<'a, Init, I, T, F>(init: Init, f: F, tris: I, left: &AABB, right: &AABB) -> (T, T)
where
    Init: Fn() -> T,
    F: Fn(&mut T, &Triangle),
    I: IntoIterator<Item = &'a Triangle>,
{
    let mut l_accum = init();
    let mut r_accum = init();
    for t in tris.into_iter() {
        let bounds = AABB::from(t.iter().map(|t| &t.pos));
        if left.intersex(&bounds) {
            f(&mut l_accum, t);
        }
        if right.intersex(&bounds) {
            f(&mut r_accum, t);
        }
    }
    (l_accum, r_accum)
}

fn partition<'a, I: Iterator<Item = &'a Triangle>>(
    tris: I,
    left: &AABB,
    right: &AABB,
) -> (Vec<Triangle>, Vec<Triangle>) {
    partition_by(
        Vec::new,
        |accum, t| accum.push(t.clone()),
        tris,
        left,
        right,
    )
}

impl Geometry for KdTree {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        match self {
            KdTree::Leaf { bounds, tris } if bounds.intersects(r) => {
                let mut max = max;
                let mut result = None;
                for tri in tris {
                    let hit_result = tri.intersection(r, min, max);
                    if let Some(hit) = &hit_result {
                        max = hit.t;
                        result = hit_result;
                    }
                }
                result
            }
            KdTree::Node {
                bounds,
                left,
                right,
            } if bounds.intersects(r) => {
                let mut max = max;
                let left = left.intersection(r, min, max);
                if let Some(hit) = &left {
                    max = hit.t;
                }
                let right = right.intersection(r, min, max);
                right.or(left)
            }
            _ => None,
        }
    }
}
