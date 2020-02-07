use crate::geom::{Triangle, Vertex};
use crate::{Vec2, Vec3};

use std::fs;
use std::path::Path;

use nalgebra_glm as glm;

pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Triangle>> {
    let mut verts = Vec::new();
    let mut coords = Vec::new();
    let mut norms = Vec::new();
    let mut tris = Vec::new();

    let text = fs::read_to_string(path)?;
    for mut iter in text
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(str::split_whitespace)
    {
        match iter.next() {
            Some("v") => {
                verts.push(parse_vec3(iter).expect("Unable to parse vertex position"));
            }
            Some("vt") => {
                coords.push(parse_uv(iter).expect("Unable to parse vertex coordinate"));
            }
            Some("vn") => {
                norms.push(parse_vec3(iter).expect("Unable to parse vertex normal"));
            }
            Some("f") => {
                tris.push(
                    parse_triangle(iter, &verts, &coords, &norms).expect("Unable to parse face"),
                );
            }
            _ => (),
        }
    }
    Ok(tris)
}

fn parse_vec3<'a, I: Iterator<Item = &'a str>>(iter: I) -> Option<Vec3> {
    let mut iter = iter.filter_map(|s| s.parse::<f32>().ok());
    let x = iter.next()?;
    let y = iter.next()?;
    let z = iter.next()?;
    Some(Vec3::new(x, y, z))
}

fn parse_uv<'a, I: Iterator<Item = &'a str>>(iter: I) -> Option<Vec2> {
    let mut iter = iter.filter_map(|s| s.parse::<f32>().ok());
    let x = iter.next()?;
    let y = iter.next()?;
    Some(Vec2::new(x, y))
}

fn parse_triangle<'a, I: Iterator<Item = &'a str>>(
    iter: I,
    verts: &[Vec3],
    coords: &[Vec2],
    norms: &[Vec3],
) -> Option<Triangle> {
    let mut iter = iter.map(|s| {
        let mut cmps = s.split('/');
        let pos = cmps
            .next()
            .and_then(|s| s.parse::<isize>().ok())
            .map(|i| index_wrap(i, verts))
            .expect("Position required for triangle definition");
        let coord = cmps
            .next()
            .and_then(|s| s.parse::<isize>().ok())
            .map(|i| index_wrap(i, coords))
            .unwrap_or_else(glm::zero);
        let norm = cmps
            .next()
            .and_then(|s| s.parse::<isize>().ok())
            .map(|i| index_wrap(i, norms));
        (pos, coord, norm)
    });
    let (p1, uv1, n1) = iter.next()?;
    let (p2, uv2, n2) = iter.next()?;
    let (p3, uv3, n3) = iter.next()?;
    let norm = triangle_normal(&p1, &p2, &p3);
    let make_vertex = |(pos, uv, normal): (Vec3, Vec2, Option<Vec3>)| {
        let normal = normal.unwrap_or(norm);
        Vertex { pos, uv, normal }
    };
    Some(Triangle::new(
        make_vertex((p1, uv1, n1)),
        make_vertex((p2, uv2, n2)),
        make_vertex((p3, uv3, n3)),
    ))
}

fn triangle_normal(p1: &Vec3, p2: &Vec3, p3: &Vec3) -> Vec3 {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    e1.cross(&e2).normalize()
}

fn index_wrap<T: Clone>(i: isize, vec: &[T]) -> T {
    if i.is_negative() {
        vec[vec.len() - i.wrapping_abs() as usize].clone()
    } else {
        vec[i as usize - 1].clone()
    }
}
