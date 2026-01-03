mod country;
mod globe;

use bevy::prelude::*;

use crate::globe::make_globe_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Globe;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Globe,
        Mesh3d(meshes.add(make_globe_mesh())),
        MeshMaterial3d(materials.add(Color::srgb_u8(30, 121, 160))),
        Transform::from_scale(Vec3::splat(1.0)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Globe>>) {
    for mut t in &mut query {
        t.rotate_local_y(0.1 * time.delta_secs());
    }
}
