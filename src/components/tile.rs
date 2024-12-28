use crate::components::enums::tile_type::TileType;
use bevy::prelude::{default, Component};
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::{TileBundle, TilePos, TileTextureIndex};

#[derive(Component)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    pub fn create_standard_bundle(
        position: TilePos,
        tilemap_id: TilemapId,
        texture_index: TileTextureIndex,
        tile_type: TileType,
    ) -> (Tile, TileBundle) {
        let tile = Tile {
            tile_type,
        };
        let tile_bundle = TileBundle {
            position,
            texture_index,
            tilemap_id,
            ..default()
        };
        (tile, tile_bundle)
    }
}