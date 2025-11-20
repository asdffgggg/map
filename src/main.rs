use std::fs;

// use bevy::prelude::*;

fn main() {
    // App::new().add_plugins(DefaultPlugins).run();
    for entry in fs::read_dir("data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display())
    }
}

// struct Country {
//     name: String,
//     borders: Vec<Vec<Vec2>>,
// }
