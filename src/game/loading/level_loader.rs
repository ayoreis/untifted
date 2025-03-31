use super::super::block::TextureAtlasIndices;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub translation: Vec3,
    pub texture_atlas_indices: TextureAtlasIndices,
}

#[derive(Asset, TypePath, Deserialize, Serialize)]
pub struct Level {
    pub blocks: Vec<Block>,
}

#[derive(Default)]
struct LevelLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
enum LevelLoaderError {
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

pub fn plugin(app: &mut App) {
    app.init_asset_loader::<LevelLoader>().init_asset::<Level>();
}
