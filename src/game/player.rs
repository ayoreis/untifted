use super::{
	super::debugger, block::Block, cube::Cube, game_loader::Game, loading::LoadingGame, physics,
	plane::Rotation, plane::Translation,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	player: Player,
	mesh_3d: Mesh3d,
	mesh_material_3d: MeshMaterial3d<StandardMaterial>,
	physics_input: physics::Input,
	transform: Transform,
}

pub const SIZE: f32 = 1.0 / 8.0 * 7.0;

impl PlayerBundle {
	pub fn new(
		meshes: &mut Assets<Mesh>,
		texture_atlas_layouts: &Assets<TextureAtlasLayout>,
		texture_atlas: TextureAtlas,
		material_handle: Handle<StandardMaterial>,
		checkpoint: Vec3,
	) -> Self {
		let atlas_size = texture_atlas_layouts
			.get(texture_atlas.layout.id())
			.unwrap()
			.size
			.as_vec2();

		let mut uv = texture_atlas
			.texture_rect(texture_atlas_layouts)
			.unwrap()
			.as_rect();

		uv.min /= atlas_size;
		uv.max /= atlas_size;

		Self {
			player: Player,
			mesh_3d: Mesh3d(meshes.add(Cube::new(SIZE, (uv, uv, uv)))),
			mesh_material_3d: MeshMaterial3d(material_handle),
			physics_input: physics::Input::default(),
			transform: Transform::from_translation(checkpoint + 0.5),
		}
	}
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(super::State = super::State::Playing)]
pub enum State {
	#[default]
	Alive,
	Dead,
}

pub fn respawn(
	games: Res<Assets<Game>>,
	loading_game: Res<LoadingGame>,
	mut rotation: ResMut<Rotation>,
	mut movement_state: ResMut<MovementState>,
	mut next_state: ResMut<NextState<State>>,
	mut player: Single<(&mut Transform, &mut physics::Input)>,
) {
	let game = games.get(loading_game.0.id()).unwrap();
	rotation.set(game.checkpoint.rotation);
	*movement_state = default();
	player.0.translation = Vec3::from(game.checkpoint.translation) + 0.5;
	*player.1 = default();
	next_state.set(State::Alive);
}

#[derive(Resource, Default, Debug, Clone, PartialEq)]
pub enum MovementState {
	#[default]
	Standing,
	Running,
	Jumping,
	Falling,
	Rotating(Vec3),
	Floating,
	Flying,
}

const RUNNING_SPEED: f32 = 10.0;
const JUMP_HEIGHT: f32 = 3.5;
const JUMP_TIME: f32 = 0.4;
const FALL_TIME: f32 = 0.3;
const JUMP_VELOCITY: f32 = (2.0 * JUMP_HEIGHT) / JUMP_TIME;
const JUMP_GRAVITY: f32 = (-2.0 * JUMP_HEIGHT) / (JUMP_TIME * JUMP_TIME);
const FALL_GRAVITY: f32 = (-2.0 * JUMP_HEIGHT) / (FALL_TIME * FALL_TIME);
const AIR_SPEED: f32 = 3.0;

pub fn movement(
	time: Res<Time>,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut rotation: ResMut<Rotation>,
	mut next_state: ResMut<NextState<State>>,
	mut state: ResMut<MovementState>,
	debugger_state: Res<bevy::prelude::State<debugger::State>>,
	player: Single<(&mut Transform, &mut physics::Input, &physics::Output), With<Player>>,
	mut games: ResMut<Assets<Game>>,
	loading_game: Res<LoadingGame>,
	mut translation: ResMut<Translation>,
) {
	let game = games.get_mut(loading_game.0.id()).unwrap();

	let (mut transform, mut input, output) = player.into_inner();
	let input_direction = get_input_direction(&keyboard);

	// Input
	let next_state = 'next_state: {
		if output.just_collided(Block::Spike(0)).is_some() {
			next_state.set(State::Dead);
			return;
		}

		if let Some(new_translation) = output.just_collided(Block::X) {
			translation.0 = new_translation;
			break 'next_state Some(MovementState::Rotating(Vec3::X));
		}

		if let Some(new_translation) = output.just_collided(Block::Y) {
			translation.0 = new_translation;
			break 'next_state Some(MovementState::Rotating(Vec3::Y));
		}

		if let Some(new_translation) = output.just_collided(Block::Z) {
			translation.0 = new_translation;
			break 'next_state Some(MovementState::Rotating(Vec3::Z));
		}

		if let Some(translation) = output.just_collided(Block::Checkpoint) {
			game.checkpoint = Isometry3d::new(translation - 0.5, rotation.get());
		}

		if matches!(**debugger_state, debugger::State::Enabled) {
			let forward = rotation.get() * Vec3::Z;

			if keyboard.just_pressed(KeyCode::ArrowUp) {
				transform.translation += 1.0 * forward;
				translation.0 += 1.0 * forward;
			}

			if keyboard.just_pressed(KeyCode::ArrowDown) {
				transform.translation -= 1.0 * forward;
				translation.0 -= 1.0 * forward;
			}

			if !matches!(*state, MovementState::Rotating(_)) {
				if keyboard.pressed(KeyCode::Digit1) {
					translation.0 = transform.translation;
					break 'next_state Some(MovementState::Rotating(Vec3::X));
				}

				if keyboard.pressed(KeyCode::Digit2) {
					translation.0 = transform.translation;
					break 'next_state Some(MovementState::Rotating(Vec3::Y));
				}

				if keyboard.pressed(KeyCode::Digit3) {
					translation.0 = transform.translation;
					break 'next_state Some(MovementState::Rotating(Vec3::Z));
				}
			}
		}

		match *state {
			MovementState::Standing => 'standing: {
				if !output.grounded {
					break 'standing Some(MovementState::Falling);
				}

				if keyboard.just_pressed(KeyCode::Space) {
					break 'standing Some(MovementState::Jumping);
				}

				if input_direction.x != 0.0 {
					break 'standing Some(MovementState::Running);
				}

				if matches!(**debugger_state, debugger::State::Enabled)
					&& keyboard.just_pressed(KeyCode::Tab)
				{
					break 'standing Some(MovementState::Floating);
				}

				None
			}

			MovementState::Running => 'running: {
				if !output.grounded {
					break 'running Some(MovementState::Falling);
				}

				if keyboard.just_pressed(KeyCode::Space) {
					break 'running Some(MovementState::Jumping);
				};

				if input_direction.x == 0.0 {
					break 'running Some(MovementState::Standing);
				}

				None
			}

			MovementState::Jumping => 'jumping: {
				if input.velocity.y <= 0.0 {
					break 'jumping Some(MovementState::Falling);
				}

				None
			}

			MovementState::Falling => 'falling: {
				if output.grounded {
					break 'falling Some(MovementState::Standing);
				}

				None
			}

			MovementState::Rotating(_) => 'rotating: {
				if rotation.transition_timer.finished() {
					break 'rotating Some(MovementState::default());
				}

				None
			}

			MovementState::Floating => 'floating: {
				if input_direction.length() != 0.0 {
					break 'floating Some(MovementState::Flying);
				}

				if keyboard.just_pressed(KeyCode::Tab) {
					break 'floating Some(MovementState::Falling);
				};

				None
			}

			MovementState::Flying => 'flying: {
				if input_direction.length() == 0.0 {
					break 'flying Some(MovementState::Floating);
				}

				None
			}
		}
	};

	if let Some(next_state) = next_state {
		// Exit
		match *state {
			MovementState::Running => {
				input.velocity.x = 0.0;
			}

			MovementState::Falling => {
				input.velocity.y = 0.0;
			}

			MovementState::Flying => {
				input.velocity = Vec2::ZERO;
			}

			_ => {}
		}

		*state = next_state;

		// Enter
		match *state {
			MovementState::Jumping => {
				input.velocity.y = JUMP_VELOCITY;
			}

			MovementState::Rotating(axis) => {
				rotation.set_axis(axis);
				input.velocity = Vec2::ZERO;
			}

			_ => {}
		}
	}

	// Update
	match *state {
		MovementState::Running => {
			input.velocity.x = input_direction.x * RUNNING_SPEED;
		}

		MovementState::Jumping => {
			input.velocity.x = AIR_SPEED * input_direction.x;
			input.velocity.y += JUMP_GRAVITY * time.delta_secs();
		}

		MovementState::Falling => {
			input.velocity.x = AIR_SPEED * input_direction.x;
			input.velocity.y += FALL_GRAVITY * time.delta_secs();
		}

		MovementState::Rotating(_) => {
			rotation.transition_timer.tick(time.delta());
		}

		MovementState::Flying => {
			input.velocity = input_direction * RUNNING_SPEED;
		}

		_ => {}
	}
}

pub fn die_out_of_bounds(
	mut next_state: ResMut<NextState<State>>,
	player: Single<&GlobalTransform, With<Player>>,
) {
	let distance = player.translation().abs();

	if distance.x > 16.0 || distance.y > 16.0 || distance.z > 16.0 {
		next_state.set(State::Dead);
	}
}

fn get_input_direction(keyboard: &Res<ButtonInput<KeyCode>>) -> Vec2 {
	let mut direction = Vec2::ZERO;

	if keyboard.pressed(KeyCode::KeyW) {
		direction.y += 1.0;
	}

	if keyboard.pressed(KeyCode::KeyA) {
		direction.x -= 1.0;
	}

	if keyboard.pressed(KeyCode::KeyS) {
		direction.y -= 1.0;
	}

	if keyboard.pressed(KeyCode::KeyD) {
		direction.x += 1.0;
	}

	direction.normalize_or_zero()
}
