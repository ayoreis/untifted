use crate::game::block::build_texture_atlas_indices;

use super::super::game::{
	self,
	block::{Block, BlockBundle, normalise_uvs},
	camera::GameCamera,
	cube::build_uvs,
	level_loader::{self, Level},
	loading::{
		INDEX_CHECKPOINT, INDEX_SPIKE, INDEX_X, INDEX_Y, INDEX_Z, LEVEL_PATH, MyTextureAtlasLayout,
		TEXTURE_ATLAS_COLUMNS, TEXTURE_ATLAS_ROWS, TextureAtlasImage, TextureAtlasMaterial,
	},
	plane::{Rotation, Translation},
};
use bevy::{
	ecs::spawn::SpawnIter, log::tracing_subscriber::fmt::MakeWriter, prelude::*,
	window::PrimaryWindow,
};
use glam::USizeVec3;
use std::fs::OpenOptions;

#[derive(Resource, Default, Debug)]
pub struct TextureAtlasIndex(pub usize);

#[derive(Component)]
struct TextureAtlasImageButton;

#[derive(Component)]
struct SelectedTextureAtlasImageButton;

pub fn plugin(app: &mut App) {
	app.add_plugins(MeshPickingPlugin)
		.init_resource::<TextureAtlasIndex>()
		.add_observer(spawn)
		.add_systems(
			Update,
			(
				button_interactions,
				(draw, save).run_if(in_state(game::State::Playing)),
			)
				.run_if(in_state(super::State::Enabled)),
		);
}

fn spawn(
	trigger: Trigger<OnAdd, super::ui::Root>,
	mut commands: Commands,
	texture_atlas_layout: Res<MyTextureAtlasLayout>,
	texture_atlas_image: Res<TextureAtlasImage>,
) {
	let buttons = (0..(TEXTURE_ATLAS_COLUMNS * TEXTURE_ATLAS_ROWS) as usize)
		.map(|index| {
			(
				TextureAtlasImageButton,
				Node {
					position_type: PositionType::Relative,
					aspect_ratio: Some(1.0),
					..default()
				},
				Outline::new(Val::Px(2.0), Val::ZERO, Color::NONE),
				ZIndex::default(),
				Button,
				ImageNode::from_atlas_image(
					texture_atlas_image.0.clone(),
					TextureAtlas {
						layout: texture_atlas_layout.0.clone(),
						index,
					},
				),
			)
		})
		.collect::<Vec<_>>();

	commands.entity(trigger.target()).with_children(|parent| {
		parent.spawn((
			Node {
				flex_grow: 1.0,
				..default()
			},
			children![(
				Node {
					height: Val::Percent(100.0),
					aspect_ratio: Some(TEXTURE_ATLAS_COLUMNS as f32 / TEXTURE_ATLAS_ROWS as f32,),
					display: Display::Grid,
					grid_template_columns: RepeatedGridTrack::fr(TEXTURE_ATLAS_COLUMNS as u16, 1.0,),
					grid_template_rows: RepeatedGridTrack::fr(TEXTURE_ATLAS_ROWS as u16, 1.0),
					..default()
				},
				Children::spawn(SpawnIter(buttons.into_iter())),
			)],
		));
	});
}

fn button_interactions(
	mut commands: Commands,
	mut texture_atlas_index: ResMut<TextureAtlasIndex>,
	mut buttons: Query<
		(
			Entity,
			&Interaction,
			&mut Outline,
			&mut ZIndex,
			&ImageNode,
			Option<&SelectedTextureAtlasImageButton>,
		),
		With<TextureAtlasImageButton>,
	>,
	old_selected: Option<Single<Entity, With<SelectedTextureAtlasImageButton>>>,
) {
	for (entity, interaction, mut outline, mut z_index, image_node, selected) in &mut buttons {
		match *interaction {
			Interaction::None => match &selected {
				Some(_) => {
					outline.color = Color::WHITE;
				}
				None => {
					outline.color = Color::NONE;
					z_index.0 = 0;
				}
			},

			Interaction::Hovered => {
				outline.color = Color::WHITE.with_alpha(0.5);
				z_index.0 = 1;
			}

			Interaction::Pressed => {
				outline.color = Color::WHITE;
				z_index.0 = 1;

				texture_atlas_index.0 = image_node.texture_atlas.as_ref().unwrap().index;

				if let Some(entity) = &old_selected {
					commands
						.entity(**entity)
						.remove::<SelectedTextureAtlasImageButton>();
				}

				commands
					.entity(entity)
					.insert(SelectedTextureAtlasImageButton);
			}
		}
	}
}

fn draw(
	mut commands: Commands,
	mut param_set: ParamSet<(
		(MeshRayCast, Query<&Block>),
		(ResMut<Assets<Mesh>>, Query<(&mut Block, &mut Mesh3d)>),
	)>,
	mouse: Res<ButtonInput<MouseButton>>,
	index: Res<TextureAtlasIndex>,
	texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
	texture_atlas_layout: Res<MyTextureAtlasLayout>,
	plane_rotation: ResMut<Rotation>,
	material: Res<TextureAtlasMaterial>,
	window: Single<&Window, With<PrimaryWindow>>,
	camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
	plane_translation: Res<Translation>,
) {
	let Some(cursor_position) = window.cursor_position() else {
		return;
	};

	let mut mesh_ray_cast = param_set.p0();
	let ray = camera
		.0
		.viewport_to_world(camera.1, cursor_position)
		.unwrap();

	let hit = mesh_ray_cast
		.0
		.cast_ray(
			ray,
			&MeshRayCastSettings::default()
				.always_early_exit()
				.with_filter(&|entity| mesh_ray_cast.1.get(entity).map_or(false, |_| true)),
		)
		.first();

	if let Some(hit) = hit {
		let (entity, hit) = hit.to_owned();

		if mouse.pressed(MouseButton::Left) {
			let (mut meshes, mut blocks) = param_set.p1();

			if let Ok((mut block, mesh_handle)) = blocks.get_mut(entity) {
				*block = match index.0 {
					index if INDEX_SPIKE.contains(&index) => Block::Spike(index),
					INDEX_X => Block::X,
					INDEX_Y => Block::Y,
					INDEX_Z => Block::Z,
					INDEX_CHECKPOINT => Block::Checkpoint,
					index => match &*block {
						Block::Generic(indices) => Block::Generic(match hit.normal.abs() {
							Vec3::X => indices.with_x(index),
							Vec3::Y => indices.with_y(index),
							Vec3::Z => indices.with_z(index),
							_ => unreachable!(),
						}),
						_ => Block::Generic(USizeVec3::splat(index)),
					},
				};

				let mesh = meshes.get_mut(mesh_handle.0.id()).unwrap();
				let texture_atlas_indices = build_texture_atlas_indices(&*block);
				let uvs = build_uvs(normalise_uvs(
					&*texture_atlas_layouts,
					&texture_atlas_layout.0,
					texture_atlas_indices,
				));

				mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
			};
		}

		if mouse.pressed(MouseButton::Right) {
			commands.entity(entity).despawn();
		}
	} else if mouse.pressed(MouseButton::Left) {
		let Some(distance) = ray.intersect_plane(
			plane_translation.0,
			InfinitePlane3d::new(plane_rotation.get() * Vec3::Z),
		) else {
			return;
		};

		let translation = ray.get_point(distance).floor();
		let (mut meshes, ..) = param_set.p1();
		let block = match index.0 {
			index if INDEX_SPIKE.contains(&index) => Block::Spike(index),
			INDEX_X => Block::X,
			INDEX_Y => Block::Y,
			INDEX_Z => Block::Z,
			INDEX_CHECKPOINT => Block::Checkpoint,
			index => Block::Generic(USizeVec3::splat(index)),
		};

		commands.spawn(BlockBundle::new(
			block,
			&mut meshes,
			&texture_atlas_layouts,
			&texture_atlas_layout.0,
			material.0.clone(),
			translation,
		));
	}
}

fn save(keyboard: Res<ButtonInput<KeyCode>>, blocks: Query<(&Block, &Transform), With<Block>>) {
	if !(keyboard.pressed(KeyCode::ControlLeft) && keyboard.pressed(KeyCode::KeyS)) {
		return;
	}

	let level = Level {
		blocks: blocks
			.iter()
			.map(|(block, transform)| level_loader::Block {
				kind: block.clone(),
				translation: transform.translation - 0.5,
			})
			.collect(),
	};

	let file = OpenOptions::new()
		.truncate(true)
		.write(true)
		.open(format!("assets/{LEVEL_PATH}"))
		.unwrap();

	serde_json::to_writer::<_, Level>(file.make_writer(), &level).unwrap();
}
