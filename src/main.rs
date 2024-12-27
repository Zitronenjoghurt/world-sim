mod components;
mod bundles;
mod systems;
mod plugins;

use crate::plugins::test::TestPlugin;
use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugins(TestPlugin)
        .run();
}