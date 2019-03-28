impl Environment {
    pub fn sample_direction(&self, dir: &Vec3) -> Vec3 {
        let u = 0.5 + f32::atan2(dir.z, dir.x) / glm::two_pi::<f32>();
        let v = 0.5 - f32::asin(dir.y) / glm::pi::<f32>();
        self.sample(u, v)
    }
}
