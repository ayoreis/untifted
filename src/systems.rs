use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub fn despawn_recursive<T: QueryFilter>(mut commands: Commands, entities: Query<Entity, T>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
