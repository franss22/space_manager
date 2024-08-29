use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use space_manager::{
    astro_bodies::AstroBodiesPlugin, mouse::CameraPlugin, orbit::OrbitPlugin,
    starship::StarshipPlugin,
};
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_plugins(CameraPlugin)
        .add_plugins((AstroBodiesPlugin, StarshipPlugin, OrbitPlugin))
        .run();
}

// fn setup(mut commands: Commands) {}
