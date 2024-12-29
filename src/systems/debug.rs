use crate::bundles::plants::grass::GrassPlantBundle;
use bevy::prelude::{AssetServer, Assets, ButtonInput, Camera, Commands, GlobalTransform, MouseButton, Query, Res, ResMut, TextureAtlasLayout, Window, With};
use bevy::window::PrimaryWindow;

pub fn spawn_grass_plant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let plant_bundle = GrassPlantBundle::new(
        &asset_server,
        &mut texture_atlas_layouts,
        0.0,
        0.0,
    );
    commands.spawn(plant_bundle);
}

pub fn spawn_grass_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = match q_window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };

    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let (camera, camera_transform) = match camera_q.get_single() {
        Ok(camera_data) => camera_data,
        Err(_) => return,
    };

    let world_pos = match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let plant_bundle = GrassPlantBundle::new(
        &asset_server,
        &mut texture_atlas_layouts,
        world_pos.x,
        world_pos.y,
    );
    commands.spawn(plant_bundle);
}