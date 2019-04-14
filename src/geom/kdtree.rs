use rayon::prelude::*;

use crate::Ray;

use super::aabb::*;
use super::{Geometry, RayHit};

pub enum KdTree<T> {
    Leaf {
        bounds: AABB,
        geoms: Vec<T>,
    },
    Node {
        bounds: AABB,
        left: Box<KdTree<T>>,
        right: Box<KdTree<T>>,
    },
}

struct Split {
    dim: usize,
    pos: f32,
    cost: f32,
}

impl<T: Bounds + Clone + Sync> KdTree<T> {
    pub fn new(geoms: Vec<T>) -> Self {
        let bounds = total_bounds(&geoms);
        Self::build(bounds, geoms)
    }

    fn build(bounds: AABB, geoms: Vec<T>) -> Self {
        let cost = cost(&bounds, geoms.len());
        let splits = (0..3)
            .into_par_iter()
            .filter_map(|dim| optimal_split(geoms.iter(), &bounds, dim as usize));
        let min = splits.min_by(|a, b| a.cost.partial_cmp(&b.cost).expect("Tried to compare NaN"));
        match &min {
            Some(split) if split.cost < cost => {
                let (left, right) = bounds.split_dimension(split.pos, split.dim);
                let (left_geoms, right_geoms) = partition_dimension(geoms, split.pos, split.dim);
                KdTree::Node {
                    bounds,
                    left: Box::new(KdTree::build(left, left_geoms)),
                    right: Box::new(KdTree::build(right, right_geoms)),
                }
            }
            _ => KdTree::Leaf { bounds, geoms },
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum State {
    Start,
    End,
}

#[derive(PartialEq, PartialOrd)]
struct Marker {
    pos: f32,
    state: State,
}

fn cost(aabb: &AABB, num_geoms: usize) -> f32 {
    const INTERSECT_COST: f32 = 2.0;
    INTERSECT_COST * aabb.surface_area() * num_geoms as f32
}

fn total_bounds<G: Bounds>(geoms: &[G]) -> AABB {
    let bounds = geoms.get(0).map(|g| g.bounds()).unwrap_or_default();
    geoms
        .iter()
        .map(Bounds::bounds)
        .fold(bounds, |a, b| AABB::union(&a, &b))
}

fn sorted_markers<'a, I, G>(geoms: I, dimension: usize) -> Vec<Marker>
where
    I: Iterator<Item = &'a G>,
    G: Bounds + 'a,
{
    let count = geoms.size_hint().1.unwrap_or(0);
    let mut markers = Vec::with_capacity(count * 2);
    for bound in geoms.map(Bounds::bounds) {
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
    markers.par_sort_by(|a, b| a.partial_cmp(&b).expect("Tried sorting NaNs"));
    markers
}

fn optimal_split<'a, I, G>(geoms: I, bounds: &AABB, dim: usize) -> Option<Split>
where
    I: Iterator<Item = &'a G>,
    G: Bounds + 'a,
{
    const TRAVERSAL_COST: f32 = 1.0;
    let markers = sorted_markers(geoms, dim);
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

fn partition_dimension<I, G>(geoms: I, split: f32, dimension: usize) -> (Vec<G>, Vec<G>)
where
    I: IntoIterator<Item = G>,
    G: Bounds + Clone,
{
    let mut l_accum = Vec::new();
    let mut r_accum = Vec::new();
    for g in geoms.into_iter() {
        let bounds = g.bounds();
        if bounds.min[dimension] <= split {
            l_accum.push(g.clone());
        }
        if bounds.max[dimension] > split {
            r_accum.push(g);
        }
    }
    (l_accum, r_accum)
}

impl<T: Geometry> Geometry for KdTree<T> {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        match self {
            KdTree::Leaf { bounds, geoms } if bounds.intersects(r) => {
                let mut max = max;
                let mut result = None;
                for tri in geoms {
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

impl<T> Bounds for KdTree<T> {
    fn bounds(&self) -> AABB {
        match self {
            KdTree::Leaf { bounds, .. } | KdTree::Node { bounds, .. } => bounds.clone(),
        }
    }
}
