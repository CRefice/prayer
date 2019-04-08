use nalgebra_glm as glm;

type Vec3 = glm::TVec3<f32>;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let inv_dir = glm::vec3(1.0, 1.0, 1.0).component_div(&direction);
        Ray {
            origin,
            direction,
            inv_dir,
        }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
