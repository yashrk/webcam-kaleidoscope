use std::collections::HashSet;

use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

fn _get_hex(texture: Texture2D, center: (i32, i32)) -> Vec<Mesh> {
    let (cx, cy) = (center.0 as f32, center.1 as f32);
    let points = [
        (2. + cx, 1. + cy),
        (0. + cx, 2. + cy),
        (-2. + cx, 1. + cy),
        (-2. + cx, -1. + cy),
        (0. + cx, -2. + cy),
        (2. + cx, -1. + cy),
    ];
    (0..points.len())
        .map(|i| {
            let uvx = if i % 2 == 0 { (1., 0.) } else { (0., 1.) };
            Mesh {
                vertices: vec![
                    Vertex {
                        position: vec3(cx, cy, 0.),
                        uv: vec2(0., 0.),
                        color: WHITE,
                    },
                    Vertex {
                        position: vec3(points[i].0, points[i].1, 0.),
                        uv: vec2(uvx.0, 1.),
                        color: WHITE,
                    },
                    Vertex {
                        position: vec3(
                            points[(i + 1) % points.len()].0,
                            points[(i + 1) % points.len()].1,
                            0.,
                        ),
                        uv: vec2(uvx.1, 1.),
                        color: WHITE,
                    },
                ],
                indices: vec![0, 1, 2],
                texture: Some(texture.clone()),
            }
        })
        .collect()
}

pub fn get_hexagons(nlevels: i16, texture: Texture2D) -> Vec<Mesh> {
    let mut res = _get_hex(texture.clone(), (0, 0));
    let mut centers = HashSet::from([(0, 0)]);
    let mut new_centers = HashSet::from([(0, 0)]);
    for _nlevel in 0..nlevels {
        let mut centers_to_add = HashSet::<(i32, i32)>::from_iter(
            new_centers
                .iter()
                .flat_map(|x| {
                    vec![
                        (x.0 - 4, x.1),
                        (x.0 + 4, x.1),
                        (x.0 - 2, x.1 - 3),
                        (x.0 - 2, x.1 + 3),
                        (x.0 + 2, x.1 - 3),
                        (x.0 + 2, x.1 + 3),
                    ]
                })
                .filter(|x| !centers.contains(x))
                .collect::<Vec<(i32, i32)>>(),
        );
        centers.extend(centers_to_add.clone());
        new_centers.clone_from(&centers_to_add);
        centers_to_add
            .drain()
            .for_each(|x| res.append(&mut _get_hex(texture.clone(), x)))
    }
    res
}
