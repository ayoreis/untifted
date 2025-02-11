use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(32.0)),
            ..Default::default()
        },
    ));
}

const PLAYER_SPEED: f32 = 256.0;

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
