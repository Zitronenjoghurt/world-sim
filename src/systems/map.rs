use crate::bundles::tilemaps::overworld::OverworldTilemapBundle;
use crate::bundles::tiles::grass::GrassTileBundle;
use crate::components::tilemap_type::BaseTilemapType;
use bevy::prelude::{default, AssetServer, Camera2dBundle, Commands, Entity, Query, Res, Transform, Vec3};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage, TilemapSize};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(0.25)),
        ..default()
    });
}

pub fn spawn_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let overworld = OverworldTilemapBundle::new(&asset_server);
    commands.spawn(overworld);
}

pub fn spawn_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileStorage, &TilemapSize, &BaseTilemapType)>,
) {
    for (entity, mut storage, size, tilemap_type) in query.iter_mut() {
        match tilemap_type {
            BaseTilemapType::Overworld => spawn_grass(&mut commands, entity, &mut storage, size),
        }
    }
}

fn spawn_grass(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    tilemap_size: &TilemapSize,
) {
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_bundle = GrassTileBundle::new(x, y, tilemap_entity);
            let tile_entity = commands.spawn(tile_bundle).id();
            tile_storage.set(&TilePos { x, y }, tile_entity);
        }
    }
}