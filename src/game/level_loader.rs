use super::block;
use bevy::{
	asset::{AssetLoader, LoadContext, io::Reader},
	prelude::*,
};
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Block {
	pub kind: block::Block,
	pub translation: Vec3,
}

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct Level {
	pub blocks: Vec<Block>,
}

#[derive(Default)]
pub struct LevelLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum LevelLoaderError {
	#[error("Error reading level: {0}")]
	Io(#[from] io::Error),
	#[error("Error parsing level: {0}")]
	Parse(#[from] serde_json::Error),
}

impl AssetLoader for LevelLoader {
	type Asset = Level;
	type Settings = ();
	type Error = LevelLoaderError;

	async fn load(
		&self,
		reader: &mut dyn Reader,
		_settings: &Self::Settings,
		_load_context: &mut LoadContext<'_>,
	) -> Result<Self::Asset, Self::Error> {
		let mut bytes = Vec::new();
		reader.read_to_end(&mut bytes).await?;
		Ok(serde_json::from_slice(&bytes)?)
	}
}
