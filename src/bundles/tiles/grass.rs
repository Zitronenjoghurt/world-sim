use crate::bundles::tiles::base::BaseTileBundle;
use crate::components::tile_type::TileType;
use bevy::prelude::Bundle;
use bevy_ecs_tilemap::prelude::{TilePos, TileTextureIndex, TilemapId};

#[derive(Bundle)]
pub struct GrassBundle {
    base: BaseTileBundle,
}

impl GrassBundle {
    pub fn new(
        position: TilePos,
        tilemap_id: TilemapId,
    ) -> Self {
        Self {
            base: BaseTileBundle::new(
                position,
                tilemap_id,
                TileTextureIndex(0),
                TileType::Grass,
            )
        }
    }
}