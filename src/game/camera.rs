use super::block::SCALED_TILE_SIZE;
use bevy::prelude::*;

#[derive(Component)]
#[require(Name(name), Transform(transform), Camera3d, Projection(projection))]
pub struct GameCamera;

fn name() -> Name {
    Name::new("Camera")
}

fn transform() -> Transform {
    Transform::from_xyz(0.0, 0.0, DISTANCE)
}

fn projection() -> Projection {
    Projection::from(OrthographicProjection {
        scale: 1.0 / SCALED_TILE_SIZE,
        ..OrthographicProjection::default_3d()
    })
}

const DISTANCE: f32 = 100.0;
