use crate::components::enums::plant_type::PlantType;
use crate::components::growth_stages::GrowthStages;
use crate::components::plant::Plant;
use crate::enums::z_layer::ZLayer;
use bevy::prelude::{AssetServer, Assets, Bundle, Sprite, TextureAtlas, TextureAtlasLayout, Transform, UVec2, Vec2};

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

        let growth_stages = GrowthStages(vec![0, 1, 2]);

        Self {
            plant: Plant::new(PlantType::Grass, 3, 5.0),
            growth_stages,
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            transform: Transform::from_xyz(position.x, position.y, ZLayer::Plants.into()),
        }
    }
}