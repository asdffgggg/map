mod country;
mod globe;

use bevy::prelude::*;

use crate::globe::{BASE_RADIUS, RELIEF, make_globe_mesh};

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
    mut images: ResMut<Assets<Image>>,
) {
    //globe
    let base_color = Color::srgb_u8(70, 220, 90);
    let (mesh, image) = make_globe_mesh();
    commands.spawn((
        Globe,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image)),
            // vary key PBR parameters on a grid of spheres to show the effect
            metallic: 0.01,
            perceptual_roughness: 0.632,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(10.0)),
    ));
    //ocean
    commands.spawn((
        Mesh3d(
            meshes.add(
                Sphere {
                    radius: BASE_RADIUS + RELIEF * 0.5625,
                }
                .mesh()
                .uv(200, 110),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb_u8(13, 86, 185))),
        Transform::from_scale(Vec3::splat(10.0)),
    ));
    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 7127.9882771332257334,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(72.3008433257, 8.9771, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Globe>>) {
    for mut t in &mut query {
        t.rotate_local_y(0.4 * time.delta_secs());
    }
}
