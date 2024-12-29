use crate::components::tilemap_type::BaseTilemapType;
use crate::enums::z_layer::ZLayer;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{default, Bundle};
use bevy_ecs_tilemap::map::{TilemapSize, TilemapTexture, TilemapTileSize};
use bevy_ecs_tilemap::prelude::{get_tilemap_center_transform, TileStorage, TilemapType};
use bevy_ecs_tilemap::TilemapBundle;

#[derive(Bundle)]
pub struct BaseTilemapBundle {
    type_: BaseTilemapType,
    tilemap_bundle: TilemapBundle,
}

impl BaseTilemapBundle {
    pub fn new(
        texture: Handle<Image>,
        map_width_tiles: u32,
        map_height_tiles: u32,
        tile_size: u32,
        map_type: TilemapType,
        z_layer: ZLayer,
        base_tilemap_type: BaseTilemapType,
    ) -> Self {
        let map_size = TilemapSize { x: map_width_tiles, y: map_height_tiles };
        let tile_size = TilemapTileSize { x: tile_size as f32, y: tile_size as f32 };
        let grid_size = tile_size.into();
        let tile_storage = TileStorage::empty(map_size);

        let tilemap_bundle = TilemapBundle {
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture),
            tile_size,
            grid_size,
            map_type,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, z_layer.into()),
            ..default()
        };

        Self {
            type_: base_tilemap_type,
            tilemap_bundle,
        }
    }
}