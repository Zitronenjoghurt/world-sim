#[derive(Debug)]
#[repr(u8)]
pub enum ZLayer {
    Terrain = 0
}

impl From<ZLayer> for f32 {
    fn from(value: ZLayer) -> f32 {
        value as u8 as f32
    }
}