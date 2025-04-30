use bevy::{
	asset::{AssetLoader, LoadContext, io::Reader},
	prelude::*,
};
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct Game {
	pub checkpoint: Isometry3d,
}

#[derive(Default)]
pub struct GameLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum GameLoaderError {
	#[error("Error reading game: {0}")]
	Io(#[from] io::Error),
	#[error("Error parsing game: {0}")]
	Parse(#[from] serde_json::Error),
}

impl AssetLoader for GameLoader {
	type Asset = Game;
	type Settings = ();
	type Error = GameLoaderError;

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
