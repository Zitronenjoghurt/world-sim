use crate::components::enums::plant_type::PlantType;
use crate::components::growth_stages::GrowthStages;
use crate::components::plant::Plant;
use crate::enums::z_layer::ZLayer;
use bevy::prelude::{AssetServer, Assets, Bundle, Sprite, TextureAtlas, TextureAtlasLayout, Transform, UVec2, Vec2};
use bevy::sprite::Anchor::BottomCenter;
use rand::prelude::SliceRandom;
use rand::thread_rng;

const GROWTH_PATTERNS: [[usize; 3]; 3] = [
    [0, 1, 2],
    [0, 3, 4],
    [0, 5, 6],
];

#[derive(Bundle)]
pub struct GrassPlantBundle {
    plant: Plant,
    growth_stages: GrowthStages,
    sprite: Sprite,
    transform: Transform,
}

impl GrassPlantBundle {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
        position: Vec2,
    ) -> Self {
        let texture = asset_server.load("grass.png");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(8, 8),
            7,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut rng = thread_rng();
        let chosen_growth_stages = GROWTH_PATTERNS.choose(&mut rng).unwrap_or(&GROWTH_PATTERNS[0]).to_vec();
        let growth_stages = GrowthStages(chosen_growth_stages);

        let mut sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        sprite.anchor = BottomCenter;

        Self {
            plant: Plant::new(PlantType::Grass, 3, 5.0),
            growth_stages,
            sprite,
            transform: Transform::from_xyz(position.x, position.y, ZLayer::Plants.into()),
        }
    }
}