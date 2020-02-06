mod camera;
mod config;
mod geom;
mod material;
mod obj;
mod ray;
mod texture;
mod vec;

use rand::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;
use vec::*;

use config::UserConfig;
use geom::*;
use ray::Ray;
use texture::Texture as _;

fn trace(r: &Ray, scene: &Scene, depth: usize) -> Vec3 {
    if depth == 0 {
        return glm::zero();
    }
    if let Some(TraceResult { material, hit }) = scene.trace(r, 0.001, std::f32::MAX) {
        let RayHit { normal, uv, .. } = hit;
        let w0 = -r.direction;
        let (bounce, pdf) = material.bounce(&w0, &hit);
        let incident = trace(&bounce, scene, depth - 1);
        let (brdf, ks) = material.brdf(&w0, &bounce.direction, &normal, uv);
        let specular = brdf / pdf;
        let diffuse = {
            let lambert = material.albedo.sample(uv) / glm::pi::<f32>();
            let kd = (glm::vec3(1.0, 1.0, 1.0) - ks) * (1.0 - material.metalness.sample(uv));
            let pdf = glm::one_over_two_pi::<f32>();
            kd.component_mul(&lambert) / pdf
        };
        let costheta = f32::max(glm::dot(&normal, &bounce.direction), 0.0);
        (diffuse + specular).component_mul(&incident) * costheta + material.emission.sample(uv)
    } else {
        let dir = r.direction.normalize();
        scene.environment.sample(Sphere::uv_at_dir(&dir))
    }
}

fn quit_with_usage() -> ! {
    eprintln!("Usage: prayer CONFIG [OUTPUT]");
    std::process::exit(1)
}

fn main() {
    let mut args = std::env::args();
    let config = args
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| quit_with_usage());
    let image = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| config.with_extension("png"));
    let UserConfig { params, scene } = UserConfig::from_file(&config).unwrap_or_else(|e| {
        eprintln!("Error parsing {}: {}", config.display(), e);
        std::process::exit(1)
    });

    let w = params.resolution.x;
    let h = params.resolution.y;
    let camera = camera::Camera::looking_at(
        glm::vec3(0.0, 1.0, -2.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        80.0,
        w as f32 / h as f32,
    );

    let buffer = (0..w * h)
        .into_par_iter()
        .flat_map(|i| {
            let x = i % w;
            let y = i / w;
            let color = (0..params.samples)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let rand: f32 = rng.gen();
                    let u = (x as f32 + rand) / w as f32;
                    let rand: f32 = rng.gen();
                    let v = (y as f32 + rand) / h as f32;
                    let ray = camera.ray_at(u, v);
                    trace(&ray, &scene, params.max_light_bounces)
                })
                .sum::<Vec3>()
                / params.samples as f32;
            let color = glm::vec3(1.0, 1.0, 1.0) - glm::exp(&(-color * params.exposure));
            vec![
                (color.x.max(0.0).min(1.0).powf(1.0 / params.gamma) * 255.99) as u8,
                (color.y.max(0.0).min(1.0).powf(1.0 / params.gamma) * 255.99) as u8,
                (color.z.max(0.0).min(1.0).powf(1.0 / params.gamma) * 255.99) as u8,
            ]
        })
        .collect::<Vec<_>>();
    image::save_buffer(&image, &buffer, w, h, image::RGB(8)).unwrap()
}
