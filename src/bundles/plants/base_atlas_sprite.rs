use crate::components::growth_stages::GrowthStages;
use crate::enums::z_layer::ZLayer;
use bevy::prelude::{AssetServer, Assets, Bundle, Sprite, TextureAtlas, TextureAtlasLayout, Transform, UVec2, Vec2};
use bevy::sprite::Anchor;

#[derive(Bundle)]
pub struct BaseAtlasSpritePlantBundle {
    growth_stages: GrowthStages,
    sprite: Sprite,
    transform: Transform,
}

impl BaseAtlasSpritePlantBundle {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        config: AtlasSpritePlantConfig,
    ) -> Self {
        let texture = asset_server.load(config.texture_path);
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(config.grid_width, config.grid_height),
            config.columns,
            config.rows,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: config.growth_stages.get_initial_index(),
            },
        );
        sprite.anchor = config.anchor;

        let transform = Transform::from_xyz(config.position.x, config.position.y, config.z_layer.into());

        Self {
            growth_stages: config.growth_stages,
            sprite,
            transform,
        }
    }
}

#[derive(Default)]
pub struct AtlasSpritePlantConfig {
    pub texture_path: String,
    pub position: Vec2,
    pub anchor: Anchor,
    pub z_layer: ZLayer,
    pub columns: u32,
    pub rows: u32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub growth_stages: GrowthStages,
}

impl AtlasSpritePlantConfig {
    pub fn new(texture_path: String, position: Vec2) -> Self {
        Self {
            texture_path,
            position,
            ..Default::default()
        }
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_z_layer(mut self, z_layer: ZLayer) -> Self {
        self.z_layer = z_layer;
        self
    }

    pub fn with_atlas_grid(mut self, columns: u32, rows: u32, height: u32, width: u32) -> Self {
        self.columns = columns;
        self.rows = rows;
        self.grid_height = height;
        self.grid_width = width;
        self
    }

    pub fn with_growth_stages(mut self, growth_stages: GrowthStages) -> Self {
        self.growth_stages = growth_stages;
        self
    }
}