pub use glm::{Vec2, Vec3};
pub use nalgebra_glm as glm;

pub fn component_minmax((mut min, mut max): (Vec3, Vec3), a: &Vec3) -> (Vec3, Vec3) {
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
