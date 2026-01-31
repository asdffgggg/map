use std::f32::consts::{PI, TAU};

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

pub const BASE_RADIUS: f32 = 1.0;
pub const RELIEF: f32 = 0.3;

pub fn make_globe_mesh() -> (Mesh, Image) {
    let image_bytes = include_bytes!("../World_elevation_map.png");
    let img = image::load_from_memory(image_bytes).unwrap().into_luma8();
    let scale = 2;
    let width = img.width() as usize / scale;
    let height: usize = img.height() as usize / scale;
    dbg!(width, height);
    let mut vertices = Vec::with_capacity(width * height);
    let mut normals = Vec::with_capacity(width * height);
    let mut indices = Vec::new();
    let mut uv = Vec::new();
    let mut textdata = Vec::new();
    let mut columns = Vec::new();
    for mut i in 0..=width {
        let u = i as f32 / width as f32;
        i %= width;
        let x = (i * scale) as u32;
        let phi = i as f32 / width as f32 * -TAU;
        let mut column = Vec::new();
        for j in 0..height {
            let v = j as f32 / height as f32;
            let k = vertices.len() as u32;
            let height = height as u32;
            let y = (j * scale) as u32;
            let px: u8 = img.get_pixel(x, y).0[0];
            let theta = j as f32 / height as f32 * PI;
            let elev = px as f32 / 255.0;
            assert!(elev >= 0.0);
            assert!(elev <= 1.0);
            let r = BASE_RADIUS + elev * RELIEF;
            let x = theta.sin() * phi.cos();
            let z = theta.sin() * phi.sin();
            let y = theta.cos();
            // dbg!(r, x, y, z);
            let n = vec3(x, y, z);
            let rgba = if elev < 0.562 {
                [27, 106, 223, 255]
            } else if elev < 0.676 {
                [70, 220, 90, 255]
            } else if elev < 0.823 {
                [166, 92, 59, 255]
            } else {
                [214, 220, 255, 255]
            };
            normals.push(n);
            vertices.push(n * r);
            uv.push(vec2(u, v));
            fn fix(mut k: u32, w: u32, a: u32, b: u32) -> u32 {
                // print!("{k},{w},{a},{b} -> ",);
                let m = k / w + b;
                k += a;
                k %= w;
                // println!("{}", m * w + k);
                m * w + k
            }
            let new_indices = [
                fix(k, height, 0, 0),
                fix(k, height, 1, 0),
                fix(k, height, 1, 1),
                fix(k, height, 0, 0),
                fix(k, height, 1, 1),
                fix(k, height, 0, 1),
            ];
            if new_indices.iter().all(|&i| i < (width as u32 + 1) * height) {
                indices.extend(new_indices)
            }
            // indices.extend([k, k + 1 + height, k + 1]);
            // indices.extend([k, k + height, k + 1 + height]);
            column.push(rgba)
        }
        columns.push(column)
    }

    for j in 0..height {
        for column in &columns {
            textdata.extend(column[j])
        }
    }
    // dbg!(&vertices, &indices);
    dbg!(indices.iter().max().unwrap(), vertices.len());

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
    .with_inserted_indices(Indices::U32(indices));
    let image = Image::new(
        Extent3d {
            width: width as u32 + 1,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        textdata,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::all(),
    );
    (mesh, image)
}
