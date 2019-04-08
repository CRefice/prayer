use nalgebra_glm as glm;

use crate::ray::Ray;
use crate::Vec3;

#[derive(Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn contains(&self, point: &Vec3) -> bool {
        point >= &self.min && point <= &self.max
    }

    pub fn intersex(&self, b: &AABB) -> bool {
        (self.min.x <= b.max.x && self.max.x >= b.min.x)
            && (self.min.y <= b.max.y && self.max.y >= b.min.y)
            && (self.min.z <= b.max.z && self.max.z >= b.min.z)
    }

    pub fn intersects(&self, r: &Ray) -> bool {
        let tx1 = (self.min.x - r.origin.x) * r.inv_dir.x;
        let tx2 = (self.max.x - r.origin.x) * r.inv_dir.x;
        let ty1 = (self.min.y - r.origin.y) * r.inv_dir.y;
        let ty2 = (self.max.y - r.origin.y) * r.inv_dir.y;
        let tz1 = (self.min.z - r.origin.z) * r.inv_dir.z;
        let tz2 = (self.max.z - r.origin.z) * r.inv_dir.z;

        let (txmin, txmax) = (f32::min(tx1, tx2), f32::max(tx1, tx2));
        let (tymin, tymax) = (f32::min(ty1, ty2), f32::max(ty1, ty2));
        let (tzmin, tzmax) = (f32::min(tz1, tz2), f32::max(tz1, tz2));
        let tmin = f32::max(f32::max(txmin, tymin), tzmin);
        let tmax = f32::min(f32::min(txmax, tymax), tzmax);

        tmax >= 0.0 && tmin <= tmax
    }

    pub fn surface_area(&self) -> f32 {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        let depth = self.max.z - self.min.z;
        2.0 * ((width * height) + (height * depth) + (width * depth))
    }

    pub fn split_x(&self, x: f32) -> (AABB, AABB) {
        let mut left_max = self.max;
        left_max.x = x;
        let mut right_min = self.min;
        right_min.x = x;
        let left = AABB {
            min: self.min,
            max: left_max,
        };
        let right = AABB {
            min: right_min,
            max: self.max,
        };
        (left, right)
    }

    pub fn split_y(&self, y: f32) -> (AABB, AABB) {
        let mut left_max = self.max;
        left_max.y = y;
        let mut right_min = self.min;
        right_min.y = y;
        let left = AABB {
            min: self.min,
            max: left_max,
        };
        let right = AABB {
            min: right_min,
            max: self.max,
        };
        (left, right)
    }

    pub fn split_z(&self, z: f32) -> (AABB, AABB) {
        let mut left_max = self.max;
        left_max.z = z;
        let mut right_min = self.min;
        right_min.z = z;
        let left = AABB {
            min: self.min,
            max: left_max,
        };
        let right = AABB {
            min: right_min,
            max: self.max,
        };
        (left, right)
    }
}

impl<'a, I> From<I> for AABB
where
    I: IntoIterator<Item = &'a Vec3>,
{
    fn from(it: I) -> Self {
        let (min, max) = component_minmax(it.into_iter()).unwrap_or((glm::zero(), glm::zero()));
        AABB { min, max }
    }
}

fn minmax_vec(minmax: (Vec3, Vec3), a: &Vec3) -> (Vec3, Vec3) {
    let (mut min, mut max) = minmax;
    if a.x > max.x {
        max.x = a.x
    } else if a.x < min.x {
        min.x = a.x
    }
    if a.y > max.y {
        max.y = a.y
    } else if a.y < min.y {
        min.y = a.y
    }
    if a.z > max.z {
        max.z = a.z
    } else if a.z < min.z {
        min.z = a.z
    }
    (min, max)
}

fn component_minmax<'a, I: Iterator<Item = &'a Vec3>>(mut it: I) -> Option<(Vec3, Vec3)> {
    let a = it.next()?.clone();
    let minmax = (a, a);
    Some(it.fold(minmax, minmax_vec))
}
