use crate::bundles::tilemaps::base::BaseTilemapBundle;
use crate::components::tilemap_type::BaseTilemapType;
use crate::enums::z_layer::ZLayer;
use crate::{MAP_HEIGHT_TILES, MAP_WIDTH_TILES, TILE_SIZE};
use bevy::prelude::{AssetServer, Bundle, Handle, Image};
use bevy_ecs_tilemap::map::TilemapType;

#[derive(Bundle)]
pub struct OverworldTilemapBundle {
    base: BaseTilemapBundle,
}

impl OverworldTilemapBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        let texture: Handle<Image> = asset_server.load("overworld.png");
        let base = BaseTilemapBundle::new(
            texture,
            MAP_WIDTH_TILES,
            MAP_HEIGHT_TILES,
            TILE_SIZE,
            TilemapType::default(),
            ZLayer::Terrain,
            BaseTilemapType::Overworld,
        );
        Self { base }
    }
}