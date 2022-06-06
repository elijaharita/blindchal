use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player camera
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());

    // Slightly rotated directional light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(
            Quat::from_rotation_x(std::f32::consts::PI * 0.1)
                * Quat::from_rotation_y(std::f32::consts::PI * 0.1),
        ),
        ..Default::default()
    });

    // Cube
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        material: materials.add(Color::PURPLE.into()),
        transform: Transform::from_translation(Vec3::Z * -4.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 0.25)),
        ..Default::default()
    });
}
