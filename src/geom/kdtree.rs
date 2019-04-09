use rayon::prelude::*;

use crate::Ray;

use super::aabb::*;
use super::mesh::*;
use super::{Geometry, RayHit};

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

struct Split {
    dim: usize,
    pos: f32,
    cost: f32,
}

impl KdTree {
    pub fn build(bounds: AABB, tris: Vec<Triangle>) -> Self {
        let cost = cost(&bounds, tris.len());
        let splits = (0..3)
            .into_par_iter()
            .filter_map(|dim| optimal_split(tris.iter(), &bounds, dim as usize));
        let min = splits.min_by(|a, b| a.cost.partial_cmp(&b.cost).expect("Tried to compare NaN"));
        match &min {
            Some(split) if split.cost < cost => {
                //dbg!(&bounds);
                //dbg!(&split);
                let (left, right) = bounds.split_dimension(split.pos, split.dim);
                assert!(left.surface_area() < bounds.surface_area());
                assert!(right.surface_area() < bounds.surface_area());
                //eprintln!("Recursing further");
                let (left_tris, right_tris) = partition_dimension(tris, split.pos, split.dim);
                KdTree::Node {
                    bounds,
                    left: Box::new(KdTree::build(left, left_tris)),
                    right: Box::new(KdTree::build(right, right_tris)),
                }
            }
            _ => KdTree::Leaf { bounds, tris },
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    End,
}

use std::cmp::Ordering;
impl PartialOrd for State {
    fn partial_cmp(&self, b: &State) -> Option<Ordering> {
        Some(match (self, b) {
            (State::Start, State::Start) => Ordering::Equal,
            (State::Start, State::End) => Ordering::Less,
            (State::End, State::Start) => Ordering::Greater,
            (State::End, State::End) => Ordering::Equal,
        })
    }
}

#[derive(Debug)]
struct Marker {
    state: State,
    pos: f32,
}

fn cost(aabb: &AABB, num_tris: usize) -> f32 {
    const INTERSECT_COST: f32 = 2.0;
    INTERSECT_COST * aabb.surface_area() * num_tris as f32
}

fn sorted_markers<'a, I: Iterator<Item = &'a Triangle>>(tris: I, dimension: usize) -> Vec<Marker> {
    let count = tris.size_hint().1.unwrap_or(0);
    let bounds = tris.map(|t| AABB::from(t.iter().map(|v| &v.pos)));
    let mut markers = Vec::with_capacity(count * 2);
    for bound in bounds {
        let start = Marker {
            state: State::Start,
            pos: bound.min[dimension],
        };
        let end = Marker {
            state: State::End,
            pos: bound.max[dimension],
        };
        markers.push(start);
        markers.push(end);
    }
    markers.par_sort_by(|a, b| {
        (a.pos, &a.state)
            .partial_cmp(&(b.pos, &b.state))
            .expect("Tried sorting NaNs")
    });
    markers
}

fn optimal_split<'a, I: Iterator<Item = &'a Triangle>>(
    tris: I,
    bounds: &AABB,
    dim: usize,
) -> Option<Split> {
    const TRAVERSAL_COST: f32 = 1.0;
    let markers = sorted_markers(tris, dim);
    let count = markers.len() / 2;
    let (mut left, mut right): (usize, usize) = (0, count);
    dedup_splits(
        markers
            .into_iter()
            .map(|marker| {
                let pos = marker.pos;
                match marker.state {
                    State::Start => left += 1,
                    State::End => right -= 1,
                }
                assert!(left <= count, right <= count);
                let (l, r) = bounds.split_dimension(pos, dim);
                let cost = TRAVERSAL_COST + cost(&l, left) + cost(&r, right);
                Split { pos, dim, cost }
            })
            .peekable(),
    )
    .min_by(|a, b| a.cost.partial_cmp(&b.cost).expect("Tried comparing NaNs"))
}

fn dedup_splits<I: Iterator<Item = Split>>(
    it: std::iter::Peekable<I>,
) -> impl Iterator<Item = Split> {
    struct Dedup<I: Iterator<Item = Split>> {
        it: std::iter::Peekable<I>,
    }
    impl<I: Iterator<Item = Split>> Iterator for Dedup<I> {
        type Item = Split;
        fn next(&mut self) -> Option<Self::Item> {
            let mut ret = self.it.next()?;
            while let Some(next) = self.it.peek() {
                if next.pos != ret.pos {
                    return Some(ret);
                }
                ret = self.it.next()?;
            }
            Some(ret)
        }
    }
    Dedup { it }
}

fn partition_dimension<I>(tris: I, split: f32, dimension: usize) -> (Vec<Triangle>, Vec<Triangle>)
where
    I: IntoIterator<Item = Triangle>,
{
    let mut l_accum = Vec::new();
    let mut r_accum = Vec::new();
    for t in tris.into_iter() {
        let bounds = AABB::from(t.iter().map(|t| &t.pos));
        if bounds.min[dimension] <= split {
            l_accum.push(t.clone());
        }
        if bounds.max[dimension] > split {
            r_accum.push(t);
        }
    }
    (l_accum, r_accum)
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
