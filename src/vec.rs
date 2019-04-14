pub use glm::{Vec2, Vec3};
pub use nalgebra_glm as glm;

/*
pub fn component_minmax(a: &Vec3, b: &Vec3) -> (Vec3, Vec3) {
    let (minx, maxx) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (miny, maxy) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
    let (minz, maxz) = if a.z < b.z { (a.z, b.z) } else { (b.z, a.z) };
    (glm::vec3(minx, miny, minz), glm::vec3(maxx, maxy, maxz))
}
*/
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
