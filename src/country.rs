use std::{collections::HashMap, fs};

use bevy::prelude::*;

#[derive(Debug)]
struct Country {
    borders: Vec<Vec<Vec<Vec2>>>,
}

impl Country {
    pub fn contains(&self, point: Vec2) -> bool {
        let a = point;
        let b = vec2(a.x + 1e9, a.y);
        self.borders.iter().flatten().any(|poly| {
            (poly
                .windows(2)
                .filter(|w| segments_intersect(a, b, w[0], w[1]))
                .count()
                + segments_intersect(a, b, *poly.last().unwrap(), poly[0]) as usize)
                % 2
                == 1
        })
    }
}

fn segments_intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }
    fn orient(a: Vec2, b: Vec2, c: Vec2) -> f32 {
        cross(b - a, c - a)
    }
    let oa = orient(c, d, a);
    let ob = orient(c, d, b);
    let oc = orient(a, b, c);
    let od = orient(a, b, d);
    oa * ob < 0.0 && oc * od < 0.0
}

fn load_countries() -> HashMap<String, Country> {
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
    countries
}
