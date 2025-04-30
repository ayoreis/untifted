use bevy::{ecs::query::QueryFilter, prelude::*};

pub fn despawn<T: QueryFilter>(mut commands: Commands, entities: Query<Entity, T>) {
	for entity in &entities {
		commands.entity(entity).despawn();
	}
}
