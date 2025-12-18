mod globe;

use std::{collections::HashMap, fs};

use bevy::prelude::*;

use crate::globe::make_globe_mesh;

fn main() {
    // App::new().add_plugins(DefaultPlugins).run();
    let mut countries = HashMap::new();
    for entry in fs::read_dir("data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        let json = fs::read_to_string(path).unwrap();
        let data: serde_json::Value = serde_json::from_str(&json).unwrap();
        let feature = &data["features"][0];
        let name = feature["properties"]["shapeName"]
            .as_str()
            .unwrap()
            .to_string();
        let mut borders = Vec::new();
        for a in feature["geometry"]["coordinates"].as_array().unwrap() {
            let mut borders2 = Vec::new();
            for b in a.as_array().unwrap() {
                let mut borders3 = Vec::new();
                for c in b.as_array().unwrap() {
                    let x = c[0].as_f64().unwrap() as f32;
                    let y = c[1].as_f64().unwrap() as f32;
                    borders3.push(vec2(x, y))
                }
                borders2.push(borders3)
            }
            borders.push(borders2)
        }
        countries.insert(name, Country { borders });
    }
    println!("{:?}", countries);
    make_globe_mesh();
}

#[derive(Debug)]
struct Country {
    borders: Vec<Vec<Vec<Vec2>>>,
}
