use crate::{HALF_MAP_HEIGHT_PX, HALF_MAP_WIDTH_PX, TILE_SIZE};
use bevy_ecs_tilemap::prelude::TilePos;
use rand::Rng;

const HALF_MAP_WIDTH_PX_F32: f32 = HALF_MAP_WIDTH_PX as f32;
const HALF_MAP_HEIGHT_PX_F32: f32 = HALF_MAP_HEIGHT_PX as f32;

pub fn get_random_coordinates_from_tile_pos(tile_pos: &TilePos, range: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let offset_x = rng.gen_range(-range..range);
    let offset_y = rng.gen_range(-range..range);

    let x = (tile_pos.x * TILE_SIZE) as f32 + range + offset_x - HALF_MAP_WIDTH_PX_F32;
    let y = (tile_pos.y * TILE_SIZE) as f32 + range + offset_y - HALF_MAP_HEIGHT_PX_F32;

    (x, y)
}