mod camera;
mod config;
mod environment;
mod geom;
mod material;
mod ray;

pub use glm::Vec3;
use nalgebra_glm as glm;

use rand::prelude::*;
use rayon::prelude::*;
use std::path::Path;

use config::UserConfig;
use geom::*;
use ray::Ray;

fn trace(r: &Ray, scene: &Scene, depth: usize) -> Vec3 {
    if depth == 0 {
        return glm::zero();
    }
    if let Some(result) = scene.trace(r, 0.001, std::f32::MAX) {
        let material = result.material;
        let w0 = -r.direction;
        let n = result.hit.normal;
        let (bounce, pdf) = material.bounce(&w0, &result.hit);
        let incident = trace(&bounce, scene, depth - 1);
        let (brdf, ks) = material.brdf(&w0, &bounce.direction, &n);
        let specular = brdf / pdf;
        let diffuse = {
            let lambert = material.color / std::f32::consts::PI;
            let kd = (glm::vec3(1.0, 1.0, 1.0) - ks) * (1.0 - material.metalness);
            let pdf = 1.0 / (2.0 * std::f32::consts::PI);
            kd.component_mul(&lambert) / pdf
        };
        let costheta = f32::max(glm::dot(&n, &bounce.direction), 0.0);
        (diffuse + specular).component_mul(&incident) * costheta + material.emission
    } else {
        let dir = glm::normalize(&r.direction);
        scene.environment.sample_direction(&dir)
    }
}

fn quit_with_usage() -> ! {
    eprintln!("Usage: prayer [OUTPUT] [CONFIG]");
    std::process::exit(1)
}

fn main() {
    let mut args = std::env::args();
    let image = args.nth(1).unwrap_or_else(|| quit_with_usage());
    let config = args.next().unwrap_or_else(|| quit_with_usage());
    let UserConfig { params, scene } =
        UserConfig::from_file(Path::new(&config)).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1)
        });

    let w = params.resolution.x;
    let h = params.resolution.y;
    let camera = camera::Camera::looking_at(
        glm::vec3(7.0, 1.0, 0.0),
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
