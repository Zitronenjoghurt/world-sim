use crate::components::enums::tile_type::TileType;
use crate::components::tile::Tile;
use bevy::prelude::Bundle;
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileTextureIndex, TilemapId};

#[derive(Bundle)]
pub struct GrassTileBundle {
    tile: Tile,
    tile_bundle: TileBundle,
}

impl GrassTileBundle {
    pub fn new(
        position: TilePos,
        tilemap_id: TilemapId,
    ) -> Self {
        let (tile, tile_bundle) = Tile::create_standard_bundle(
            position,
            tilemap_id,
            TileTextureIndex(0),
            TileType::Grass,
        );

        Self {
            tile,
            tile_bundle,
        }
    }
}