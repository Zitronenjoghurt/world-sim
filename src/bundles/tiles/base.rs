use crate::components::tile_type::TileType;
use bevy::prelude::{default, Bundle};
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileTextureIndex, TilemapId};

#[derive(Bundle)]
pub struct BaseTileBundle {
    type_: TileType,
    tile_bundle: TileBundle,
}

impl BaseTileBundle {
    pub fn new(
        tile_type: TileType,
        position: TilePos,
        tilemap_id: TilemapId,
        texture_index: TileTextureIndex,
    ) -> BaseTileBundle {
        let tile_bundle = TileBundle {
            position,
            texture_index,
            tilemap_id,
            ..default()
        };

        Self {
            type_: tile_type,
            tile_bundle,
        }
    }
}