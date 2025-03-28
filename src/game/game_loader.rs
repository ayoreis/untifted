use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use serde::Deserialize;
use std::io;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize, Resource)]
pub struct Game {
    pub level: String,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            level: "level".into(),
        }
    }
}

#[derive(Default)]
struct GameLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
enum GameLoaderError {
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

pub fn plugin(app: &mut App) {
    app.init_asset_loader::<GameLoader>().init_asset::<Game>();
}
