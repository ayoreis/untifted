use super::{colors, State};
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Menu), on_enter)
            .add_systems(OnExit(State::Menu), on_exit)
            .add_systems(
                Update,
                (enter_game_interactions, quit_app_interactions).run_if(in_state(State::Menu)),
            );
    }
}

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct EnterGameButton;

#[derive(Component)]
struct QuitAppButton;

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
            parent.spawn(Text::new("Untifted"));

            parent
                .spawn((
                    EnterGameButton,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        ..default()
                    },
                    Button,
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(colors::BLUE),
                ))
                .with_child(Text::new("Enter game"));

            parent
                .spawn((
                    QuitAppButton,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        ..default()
                    },
                    Button,
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(colors::RED),
                ))
                .with_child(Text::new("Quit app"));
        });
}

fn enter_game_interactions(
    mut next_state: ResMut<NextState<State>>,
    button: Single<(&Interaction, &mut BackgroundColor), With<EnterGameButton>>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match &interaction {
        Interaction::None => {
            background_color.0 = colors::BLUE;
        }
        Interaction::Hovered => {
            background_color.0 = colors::LIGHT_BLUE;
        }
        Interaction::Pressed => {
            next_state.set(State::Game);
        }
    }
}

fn quit_app_interactions(
    button: Single<(&Interaction, &mut BackgroundColor), With<QuitAppButton>>,
    mut app_exit_events: EventWriter<AppExit>,
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
            app_exit_events.send(AppExit::Success);
        }
    }
}

fn on_exit(mut commands: Commands, ui_root: Single<Entity, With<UiRoot>>) {
    commands.entity(*ui_root).despawn_recursive();
}
