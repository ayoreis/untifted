use super::block;
use super::plane::Translation;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[require(
	Name(name),
	Translation,
    Transform,
    // Mesh3d(mesh3d),
    // MeshMaterial3d(mesh_material3d),
    KinematicCharacterController,
    Velocity,
    Collider(collider),
)]
pub struct Player;

fn name() -> Name {
    Name::new("Player")
}

// fn mesh3d() -> Mesh3d {}
// fn mesh_material3d() -> MeshMaterial3d<StandardMaterial> {}

fn collider() -> Collider {
    let half_extent = block::SIZE / 2.0;
    Collider::cuboid(half_extent, half_extent, half_extent)
}
