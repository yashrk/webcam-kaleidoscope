use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub fn get_mesh(texture: Texture2D) -> Vec<Mesh> {
    let points = vec![
        (2., 1.),
        (0., 2.),
        (-2., 1.),
        (-2., -1.),
        (0., -2.),
        (2., -1.),
    ];
    (0..points.len())
        .map(|i| {
            let uvx = if i % 2 == 0 { (1., 0.) } else { (0., 1.) };
            Mesh {
                vertices: vec![
                    Vertex {
                        position: vec3(0., 0., 0.),
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
