use bevy::prelude::*;
use space_manager::{
    astro_bodies::AstroBodiesPlugin, mouse::CameraPlugin, starship::StarshipPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins((AstroBodiesPlugin, StarshipPlugin))
        .run();
}

// fn setup(mut commands: Commands) {}
