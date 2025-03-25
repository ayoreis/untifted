use super::plane;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[require(
    RigidBody(rigid_body),
    // Mesh3d(mesh3d),
    // MeshMaterial3d(mesh_material3d),
    Collider(collider),
    KinematicCharacterController,
    // plane::Rotation,
    Transform
)]
pub struct Player;

// fn mesh3d() -> Mesh3d {}

// fn mesh_material3d() -> MeshMaterial3d<StandardMaterial> {}

fn rigid_body() -> RigidBody {
    RigidBody::KinematicPositionBased
}

fn collider() -> Collider {
    Collider::cuboid(1.5, 0.5, 0.5)
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update, update2).run_if(in_state(super::State::Playing)),
    );
}

/// Blocks/s
const PLAYER_SPEED: f32 = 5.0;

fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut kinematic_character_controller: Single<&mut KinematicCharacterController, With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let speed = PLAYER_SPEED * time.delta_secs();
    let direction = direction.extend(0.0).normalize_or_zero();
    let velocity = direction * speed;

    kinematic_character_controller.translation = Some(velocity);
}

fn update2(
    kinematic_character_controller_output: Single<
        (Option<&KinematicCharacterControllerOutput>, &mut Transform),
        With<Player>,
    >,
) {
    let (output, mut transform) = kinematic_character_controller_output.into_inner();
    let Some(output) = output else { return };

    transform.translation += output.effective_translation;
}
