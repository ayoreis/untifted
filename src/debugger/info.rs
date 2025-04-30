use super::{super::game::player::MovementState, editor::TextureAtlasIndex};
use bevy::prelude::*;

#[derive(Component)]
#[require(Node {
	flex_direction: FlexDirection::Column,
	..default()
})]
pub struct UiRoot;

#[derive(Component)]
struct MovementStateText;

#[derive(Component)]
struct TextureAtlasIndexText;

pub fn plugin(app: &mut App) {
	app.add_observer(spawn).add_systems(
		Update,
		(
			update_movement_state_text
				.run_if(in_state(super::State::Enabled).and(resource_changed::<MovementState>)),
			update_texture_atlas_index_text
				.run_if(in_state(super::State::Enabled).and(resource_changed::<TextureAtlasIndex>)),
		),
	);
}

fn spawn(trigger: Trigger<OnAdd, super::ui::Root>, mut commands: Commands) {
	commands.entity(trigger.target()).with_children(|parent| {
		parent
			.spawn(Text::new("State machine state: "))
			.with_child((MovementStateText, TextSpan::default()));

		parent
			.spawn(Text::new("Texture atlas index: "))
			.with_child((TextureAtlasIndexText, TextSpan::default()));
	});
}

fn update_movement_state_text(
	state: Res<MovementState>,
	mut text: Single<&mut TextSpan, With<MovementStateText>>,
) {
	text.0 = format!("{:?}", *state);
}

fn update_texture_atlas_index_text(
	index: Res<TextureAtlasIndex>,
	mut text: Single<&mut TextSpan, With<TextureAtlasIndexText>>,
) {
	text.0 = format!("{:?}", index.0);
}
