use std::f32::consts::{PI, TAU};

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub fn make_globe_mesh() -> Mesh {
    let image_bytes = include_bytes!("../World_elevation_map.png");
    let img = image::load_from_memory(image_bytes).unwrap().into_luma8();
    let scale = 3;
    let width = img.width() as usize / scale;
    let height: usize = img.height() as usize / scale;
    let mut vertices = Vec::with_capacity(width * height);
    let mut normals = Vec::with_capacity(width * height);
    let mut indices = Vec::new();
    let base_radius = 1.0;
    let relief = 0.3;
    for i in 0..width {
        let x = (i * scale) as u32;
        let theta = i as f32 / width as f32 * TAU;
        for j in 0..height {
            let k = vertices.len() as u32;
            let height = height as u32;
            let y = (j * scale) as u32;
            let px = img.get_pixel(x, y).0[0];
            let phi = j as f32 / height as f32 * PI - PI / 2.0;
            let r = base_radius + px as f32 / 255.0 * relief;
            let x = theta.sin() * phi.cos();
            let y = theta.sin() * phi.sin();
            let z = theta.cos();
            let n = vec3(x, y, z);
            normals.push(n);
            vertices.push(n * r);
            indices.extend([k, k + 1, k + 1 + height]);
            indices.extend([k, k + 1 + height, k + height]);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}
