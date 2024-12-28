use crate::bundles::tiles::grass::GrassTileBundle;
use crate::enums::z_layer::ZLayer;
use crate::{MAP_HEIGHT_TILES, MAP_WIDTH_TILES};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{get_tilemap_center_transform, TilePos, TileStorage, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::TilemapBundle;

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // ToDo: Fix deprecation
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(0.25)),
        ..default()
    });

    let texture: Handle<Image> = asset_server.load("overworld.png");

    let map_size = TilemapSize { x: MAP_WIDTH_TILES, y: MAP_HEIGHT_TILES };
    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let mut tile_storage = TileStorage::empty(map_size);

    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands.spawn(
                GrassTileBundle::new(
                    tile_pos,
                    TilemapId(tilemap_entity),
                )
            ).id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture),
        tile_size,
        grid_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, ZLayer::Terrain.into()),
        ..default()
    });
}