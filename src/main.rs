use std::fs;

use bevy::prelude::*;

fn main() {
    // App::new().add_plugins(DefaultPlugins).run();
    for entry in fs::read_dir("data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        let json = fs::read_to_string(path).unwrap();
        let data = serde_json::from_str(json).unwrap();
        let feature = data["features"][0];
        let name = feature["properties"]["shapeName"];
        for a in feature["geometry"]["coordinates"].as_array().unwrap(){
        for b in a .as_array().unwrap(){
        for c in b .as_array().unwrap(){
        let x = c[0].as_number().unwrap();
        let y = c[1].as_number().unwrap();
        }
        }
        
        }

    }
}

struct Country {
    name: String,
    borders: Vec<Vec<Vec<Vec2>>>,
}
