use super::{colors, State};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Game), on_enter)
            .add_systems(OnExit(State::Game), on_exit)
            .add_systems(Update, buttons.run_if(in_state(State::Game)));
    }
}

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct ExitGameButton;

fn on_enter(mut commands: Commands) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ExitGameButton,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        ..default()
                    },
                    Button,
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(colors::RED),
                ))
                .with_child(Text::new("Exit game"));

            parent.spawn(Text::new("TODO: Put game back here"));
        });
}

fn buttons(
    mut next_state: ResMut<NextState<State>>,
    button: Single<(&Interaction, &mut BackgroundColor), With<ExitGameButton>>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match &interaction {
        Interaction::None => {
            background_color.0 = colors::RED;
        }
        Interaction::Hovered => {
            background_color.0 = colors::LIGHT_RED;
        }
        Interaction::Pressed => {
            next_state.set(State::Menu);
        }
    }
}

fn on_exit(mut commands: Commands, ui_root: Single<Entity, With<UiRoot>>) {
    commands.entity(*ui_root).despawn_recursive();
}
