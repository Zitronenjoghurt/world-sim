#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum MapLayer {
    Terrain = 0,
    Plants = 1,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> f32 {
        value as u8 as f32
    }
}