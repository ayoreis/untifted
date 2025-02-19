mod pan_orbit_camera;

use bevy::prelude::*;
use pan_orbit_camera::*;

#[derive(Default, Debug)]
enum PlacementAxis {
    X,
    #[default]
    Y,
    Z,
}

#[derive(Default, Debug)]
enum PlacementMode {
    #[default]
    Slice,
    Adjacent,
}

#[derive(Resource, Default, Debug)]
struct Placement {
    mode: PlacementMode,
    axis: PlacementAxis,
    index: i32,
}

#[derive(Component)]
struct PlacementText;

fn main() {
    App::new()
        .init_resource::<Placement>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Editor".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_camera))
        .add_systems(
            Update,
            (update_placement_text, update_placement, pan_orbit_camera),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    placement: Res<Placement>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PlacementText, Text::new(format!("{:?}", *placement))));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 5.0),
    ));
}

fn update_placement_text(
    placement: Res<Placement>,
    mut placement_text: Single<&mut Text, With<PlacementText>>,
) {
    placement_text.0 = format!("{:?}", *placement)
}

fn update_placement(input: Res<ButtonInput<KeyCode>>, mut placement: ResMut<Placement>) {
    if input.just_pressed(KeyCode::Space) {
        placement.mode = match placement.mode {
            PlacementMode::Slice => PlacementMode::Adjacent,
            PlacementMode::Adjacent => PlacementMode::Slice,
        }
    }

    if input.just_pressed(KeyCode::Digit1) {
        placement.axis = PlacementAxis::X;
        placement.index = 0;
    } else if input.just_pressed(KeyCode::Digit2) {
        placement.axis = PlacementAxis::Y;
        placement.index = 0;
    } else if input.just_pressed(KeyCode::Digit3) {
        placement.axis = PlacementAxis::Z;
        placement.index = 0;
    }

    if input.just_pressed(KeyCode::ArrowUp) {
        placement.index += 1;
    } else if input.just_pressed(KeyCode::ArrowDown) {
        placement.index -= 1;
    }
}
