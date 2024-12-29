use crate::bundles::tiles::base::BaseTileBundle;
use crate::components::tile_type::TileType;
use bevy::prelude::{Bundle, Entity};
use bevy_ecs_tilemap::prelude::{TilePos, TileTextureIndex, TilemapId};

#[derive(Bundle)]
pub struct GrassTileBundle {
    base: BaseTileBundle,
}

impl GrassTileBundle {
    pub fn new(x: u32, y: u32, tilemap_entity: Entity) -> Self {
        let base = BaseTileBundle::new(
            TileType::Grass,
            TilePos { x, y },
            TilemapId(tilemap_entity),
            TileTextureIndex(0),
        );
        Self { base }
    }
}