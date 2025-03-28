use super::block::SCALED_TILE_SIZE;
use bevy::prelude::*;

#[derive(Component)]
#[require(Camera3d, Name(name), Projection(projection), Transform(transform))]
pub struct GameCamera;

fn name() -> Name {
    Name::new("Camera")
}

fn projection() -> Projection {
    Projection::from(OrthographicProjection {
        scale: 1.0 / SCALED_TILE_SIZE,
        ..OrthographicProjection::default_3d()
    })
}

fn transform() -> Transform {
    Transform::from_xyz(0.0, 0.0, 100.0)
}
