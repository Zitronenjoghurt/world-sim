use crate::bundles::plants::base::BasePlantBundle;
use crate::bundles::plants::base_atlas_sprite::{AtlasSpritePlantConfig, BaseAtlasSpritePlantBundle};
use crate::components::growth_stages::GrowthStages;
use crate::components::plant_type::PlantType;
use crate::enums::z_layer::ZLayer;
use bevy::prelude::{AssetServer, Assets, Bundle, TextureAtlasLayout, Vec2};
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

const GROWTH_PATTERNS: [[usize; 4]; 4] = [
    [0, 1, 1, 4],
    [0, 1, 1, 5],
    [0, 2, 3, 6],
    [0, 2, 3, 7]
];

#[derive(Bundle)]
pub struct GrassPlantBundle {
    base: BasePlantBundle,
    visuals: BaseAtlasSpritePlantBundle,
}

impl GrassPlantBundle {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        x: f32,
        y: f32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let chosen_growth_stages = GROWTH_PATTERNS.choose(&mut rng).unwrap_or(&GROWTH_PATTERNS[0]).to_vec();
        let growth_stages = GrowthStages(chosen_growth_stages);

        let base = BasePlantBundle::new(
            PlantType::Grass,
            0.0,
            30.0,
            0.1,
            0.025,
        );

        let atlas_config =
            AtlasSpritePlantConfig::new("grass.png".to_string(), Vec2 { x, y })
                .with_atlas_grid(8, 1, 8, 8)
                .with_anchor(Anchor::BottomCenter)
                .with_z_layer(ZLayer::Plants)
                .with_growth_stages(growth_stages);

        let visuals = BaseAtlasSpritePlantBundle::new(
            asset_server,
            texture_atlas_layouts,
            atlas_config,
        );

        Self { base, visuals }
    }
}