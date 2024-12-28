use crate::components::tags::Tile;
use crate::components::tile_type::TileType;
use bevy::prelude::{default, Bundle};
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileTextureIndex, TilemapId};

#[derive(Bundle)]
pub struct BaseTileBundle {
    tile: Tile,
    tile_type: TileType,
    standard_tile: TileBundle,
}

impl BaseTileBundle {
    pub fn new(
        position: TilePos,
        tilemap_id: TilemapId,
        texture_index: TileTextureIndex,
        tile_type: TileType,
    ) -> Self {
        Self {
            tile: Tile,
            tile_type,
            standard_tile: TileBundle {
                position,
                texture_index,
                tilemap_id,
                ..default()
            },
        }
    }
}