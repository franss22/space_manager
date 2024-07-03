use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{astro_bodies::*, mouse::CursorPos};

pub struct StarshipPlugin;
impl Plugin for StarshipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_starship)
            .add_systems(FixedUpdate, move_starship);
    }
}

#[derive(Component, Debug)]
struct Starship;

fn spawn_starship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Triangle2d::new(Vec2::new(8., 0.), Vec2::new(0., 3.), Vec2::new(0., -3.));

    commands.spawn((
        Starship,
        MapShape {
            entity_type: MapShapeType::Ship,
            name: Name::new("Ship"),
            // spatial: SpatialBundle::from_transform(Transform::from_xyz(30.0, 0.0, 0.0)),
            shape: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(triangle)),
                material: materials.add(Color::rgb(0.8, 0.8, 0.1)),
                transform: Transform::default(),
                ..default()
            },
        },
    ));
}

const SHIP_SPEED: f32 = 3.;
const DIST_THRESHOLD: f32 = 4.;

fn move_starship(mut query: Query<&mut Transform, With<Starship>>, cursor_pos: Res<CursorPos>) {
    let mut ship_tr = query.single_mut();
    let dir = cursor_pos.pos - ship_tr.translation;
    if dir.length() > DIST_THRESHOLD {
        ship_tr.translation += dir.normalize_or_zero() * SHIP_SPEED;

        ship_tr.rotation = Quat::from_rotation_arc_2d(Vec2::X, dir.xy().normalize_or_zero());
    }
}
