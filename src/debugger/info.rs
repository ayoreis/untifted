use super::super::game::playing::state_machine;
use super::editor::TextureAtlasIndex;
use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct UiRoot;

#[derive(Component)]
struct StateMachineText;

#[derive(Component)]
struct TextureAtlasIndexText;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn).add_systems(
        Update,
        (
            update_state_machine_text.run_if(
                in_state(super::State::Enabled).and(resource_changed::<state_machine::State>),
            ),
            update_texture_atlas_index_text
                .run_if(in_state(super::State::Enabled).and(resource_changed::<TextureAtlasIndex>)),
        ),
    );
}

fn spawn(trigger: Trigger<OnAdd, UiRoot>, mut commands: Commands) {
    commands.entity(trigger.entity()).with_children(|parent| {
        parent
            .spawn(Text::new("State machine state: "))
            .with_child((StateMachineText, TextSpan::default()));

        parent
            .spawn(Text::new("Texture atlas index: "))
            .with_child((TextureAtlasIndexText, TextSpan::default()));
    });
}

fn update_state_machine_text(
    state: Res<state_machine::State>,
    mut text: Single<&mut TextSpan, With<StateMachineText>>,
) {
    text.0 = format!("{:?}", *state);
}

fn update_texture_atlas_index_text(
    index: Res<TextureAtlasIndex>,
    mut text: Single<&mut TextSpan, With<TextureAtlasIndexText>>,
) {
    text.0 = format!("{:?}", *index);
}
