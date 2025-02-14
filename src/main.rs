use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Tile;

const TILE_SIZE: f32 = 32.0;
const PLAYER_SPEED: f32 = 256.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, third_dimension))
        .run();
}

fn spawn_layer_1(mut commands: Commands, tiles: Query<Entity, With<Tile>>) {
    for tile in &tiles {
        commands.entity(tile).despawn();
    }

    commands.spawn((
        Tile,
        Sprite {
            color: Color::hsl(0.0, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -64.0, 0.0),
    ));

    commands.spawn((
        Tile,
        Sprite {
            color: Color::hsl(0.0, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(64.0, 64.0, 0.0),
    ));

    commands.spawn((
        Tile,
        Sprite {
            color: Color::hsl(0.0, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(-64.0, 64.0, 0.0),
    ));
}

fn spawn_layer_2(mut commands: Commands, tiles: Query<Entity, With<Tile>>) {
    for tile in &tiles {
        commands.entity(tile).despawn();
    }

    commands.spawn((
        Tile,
        Sprite {
            color: Color::hsl(180.0, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(-64.0, 0.0, 0.0),
    ));

    commands.spawn((
        Tile,
        Sprite {
            color: Color::hsl(180.0, 0.5, 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(64.0, 0.0, 0.0),
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn(Text::new("Layer 0"));

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn player_movement(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0
    };

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0
    };

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0
    };

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0
    };

    let Vec3 { z, .. } = player.translation;
    player.translation += direction.extend(z) * PLAYER_SPEED * time.delta_secs();
}

fn third_dimension(
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut text: Single<&mut Text>,
    tiles: Query<Entity, With<Tile>>,
) {
    if input.pressed(KeyCode::Digit1) {
        text.0 = "Level 1".into();
        spawn_layer_1(commands, tiles);
    } else if input.pressed(KeyCode::Digit2) {
        text.0 = "Level 2".into();
        spawn_layer_2(commands, tiles);
    }
}
