use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub fn get_triangles(nlevels: i16, texture: Texture2D) -> Vec<Mesh> {
    let uv = [(0., 0.), (1., 0.), (0.5, 1.)];
    let mut res = Vec::new();
    let mut top_points: Vec<_> = (0..3 * nlevels)
        .map(|i| {
            (
                ((-3 * nlevels + 1 + 2 * i) as f32, (2 * nlevels - 1) as f32),
                ((i + 1) % 3) as usize,
            )
        })
        .collect();

    while top_points.len() > 0 {
        let bottom_points: Vec<_> = top_points[0..top_points.len() - 1]
            .into_iter()
            .map(|p| ((p.0 .0 + 1., p.0 .1 - 2.), ((2 + p.1) % 3) as usize))
            .collect();
        for i in 0..bottom_points.len() {
            res.push(Mesh {
                vertices: vec![
                    Vertex {
                        position: vec3(bottom_points[i].0 .0, bottom_points[i].0 .1, 0.),
                        uv: vec2(uv[bottom_points[i].1].0, uv[bottom_points[i].1].1),
                        color: WHITE,
                    },
                    Vertex {
                        position: vec3(top_points[i].0 .0, top_points[i].0 .1, 0.),
                        uv: vec2(uv[top_points[i].1].0, uv[top_points[i].1].1),
                        color: WHITE,
                    },
                    Vertex {
                        position: vec3(top_points[i + 1].0 .0, top_points[i + 1].0 .1, 0.),
                        uv: vec2(uv[top_points[i + 1].1].0, uv[top_points[i + 1].1].1),
                        color: WHITE,
                    },
                ],
                indices: vec![0, 1, 2],
                texture: Some(texture.clone()),
            });
            if i < bottom_points.len() - 1 {
                res.push(Mesh {
                    vertices: vec![
                        Vertex {
                            position: vec3(bottom_points[i].0 .0, bottom_points[i].0 .1, 0.),
                            uv: vec2(uv[bottom_points[i].1].0, uv[bottom_points[i].1].1),
                            color: WHITE,
                        },
                        Vertex {
                            position: vec3(
                                bottom_points[i + 1].0 .0,
                                bottom_points[i + 1].0 .1,
                                0.,
                            ),
                            uv: vec2(uv[bottom_points[i + 1].1].0, uv[bottom_points[i + 1].1].1),
                            color: WHITE,
                        },
                        Vertex {
                            position: vec3(top_points[i + 1].0 .0, top_points[i + 1].0 .1, 0.),
                            uv: vec2(uv[top_points[i + 1].1].0, uv[top_points[i + 1].1].1),
                            color: WHITE,
                        },
                    ],
                    indices: vec![0, 1, 2],
                    texture: Some(texture.clone()),
                });
            }
        }
        top_points = bottom_points;
    }
    res
}
