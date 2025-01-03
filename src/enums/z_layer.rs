#[derive(Debug, Default)]
#[repr(u8)]
pub enum ZLayer {
    #[default]
    Terrain = 0,
    Plants = 1,
}

impl From<ZLayer> for f32 {
    fn from(value: ZLayer) -> f32 {
        value as u8 as f32
    }
}