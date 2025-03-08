use super::GameState;
use crate::systems::despawn_recursive;
use bevy::prelude::*;

#[derive(Resource)]
struct LoadingTimer(Timer);

impl Default for LoadingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.3, TimerMode::Once))
    }
}

#[derive(Component)]
struct UiRoot;

pub fn plugin(app: &mut App) {
    app.init_resource::<LoadingTimer>()
        .add_systems(OnEnter(GameState::Loading), spawn_ui)
        .add_systems(Update, load.run_if(in_state(GameState::Loading)))
        .add_systems(
            OnExit(GameState::Loading),
            despawn_recursive::<With<UiRoot>>,
        );
}

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((UiRoot, Node::default()))
        .with_child(Text::new("Loading..."));
}

fn load(
    time: Res<Time>,
    mut loading_timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if loading_timer.0.tick(time.delta()).finished() {
        next_state.set(GameState::Playing);
    }
}
