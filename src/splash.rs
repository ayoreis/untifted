use super::AppState;
use crate::systems::despawn_recursive;
use crate::ui::{button, BLUE, LIGHT_BLUE, LIGHT_RED, RED};
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct EnterGameButton;

#[derive(Component)]
struct QuitAppButton;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), setup_ui)
        .add_systems(
            Update,
            (enter_game_button, quit_app_button, shortcuts).run_if(in_state(AppState::Splash)),
        )
        .add_systems(OnExit(AppState::Splash), despawn_recursive::<With<UiRoot>>);
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Untifted"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));

            parent
                .spawn(button(EnterGameButton, BLUE))
                .with_child(Text::new("Enter game"));

            parent
                .spawn(button(QuitAppButton, RED))
                .with_child(Text::new("Quit app"));
        });
}

fn enter_game_button(
    button: Single<(&Interaction, &mut BackgroundColor), With<EnterGameButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match interaction {
        Interaction::None => {
            background_color.0 = BLUE;
        }
        Interaction::Hovered => {
            background_color.0 = LIGHT_BLUE;
        }
        Interaction::Pressed => {
            next_state.set(AppState::Game);
        }
    }
}

fn quit_app_button(
    button: Single<(&Interaction, &mut BackgroundColor), With<QuitAppButton>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match interaction {
        Interaction::None => {
            background_color.0 = RED;
        }
        Interaction::Hovered => {
            background_color.0 = LIGHT_RED;
        }
        Interaction::Pressed => {
            app_exit_events.send(AppExit::Success);
        }
    }
}

fn shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::Game);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
