mod country;
mod globe;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::globe::{BASE_RADIUS, RELIEF, make_globe_mesh};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#globe".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Rotate(false))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, rotate_globe))
        .run();
}

#[derive(Component)]
struct Globe;

#[derive(Component)]
struct Ocean;

#[derive(Resource)]
struct Rotate(bool);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Globe
    let (mesh, image) = make_globe_mesh();
    commands.spawn((
        Globe,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image)),
            // vary key PBR parameters on a grid of spheres to show the effect
            metallic: 0.01,
            perceptual_roughness: 0.632,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(10.0)),
    ));
    // Ocean
    const OCEAN_SHELLS: usize = 4;
    for i in 1..=OCEAN_SHELLS {
        commands.spawn((
            Ocean,
            Mesh3d(
                meshes.add(
                    Sphere {
                        radius: BASE_RADIUS + RELIEF * 0.5625 * i as f32 / OCEAN_SHELLS as f32,
                    }
                    .mesh()
                    .uv(30, 18),
                ),
            ),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba_u8(27, 106, 224, 35),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_scale(Vec3::splat(10.0)),
        ));
    }
    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 7127.9882771332257334,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(72.3008433257, 8.9771, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_globe(
    time: Res<Time>,
    rotate: Res<Rotate>,
    mut globe: Query<&mut Transform, (With<Globe>, Without<Ocean>)>,
    mut ocean: Query<&mut Transform, With<Ocean>>,
) {
    for mut t in &mut ocean {
        t.rotate_local_y(0.4 * time.delta_secs());
    }
    if !rotate.0 {
        return;
    }
    for mut t in &mut globe {
        t.rotate_local_y(0.4 * time.delta_secs());
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut rotate: ResMut<Rotate>,
    mousebutton: Res<ButtonInput<MouseButton>>,
    mut mousemotion: MessageReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    if mousebutton.just_pressed(MouseButton::Right) {
        rotate.0 = !rotate.0;
    }

    if !mousebutton.pressed(MouseButton::Left) {
        return;
    }
    for motion in mousemotion.read() {
        for mut t in &mut camera {
            t.rotate_around(
                Vec3::ZERO,
                Quat::from_rotation_y(time.delta_secs() * -0.4 * motion.delta.x),
            );
            t.rotation = t.rotation.normalize();
            let axis = t.local_x().into();
            t.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(axis, time.delta_secs() * -0.4 * motion.delta.y),
            );
        }
    }
}
