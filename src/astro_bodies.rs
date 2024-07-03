use std::ops::Range;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::prelude::*;

const G: f32 = 6.674e-11;

pub struct AstroBodiesPlugin;
impl Plugin for AstroBodiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_solar_system)
            .add_systems(FixedUpdate, orbit);
    }
}

#[derive(Component)]
pub struct Orbit {
    // pub semimajor_axis: f32, //mitad del diámetro ancho
    // pub eccentricity: f32, // [0, 1) (cuan redonda es la elipse, 0 es redondo, 1 es parábola (abierta))
    pub speed: f32,
}

#[derive(Component)]
pub enum MapShapeType {
    Planet,
    Sun,
    Ship,
}

#[derive(Bundle)]
pub struct MapShape {
    pub entity_type: MapShapeType,
    pub name: Name,
    pub shape: MaterialMesh2dBundle<ColorMaterial>,
}

fn random_2d_pt(x_range: Range<f32>, y_range: Range<f32>) -> Transform {
    let mut rng = rand::thread_rng();
    Transform::from_xyz(rng.gen_range(x_range), rng.gen_range(y_range), 0.0)
}

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sun = commands
        .spawn(MapShape {
            entity_type: MapShapeType::Sun,
            name: Name::new("Sun"),
            // spatial: SpatialBundle::from_transform(Transform::from_xyz(30.0, 0.0, 0.0)),
            shape: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(15.0))),
                material: materials.add(Color::rgb(0.8, 0.8, 0.1)),
                ..default()
            },
        })
        .id();

    let circle = Mesh2dHandle(meshes.add(Circle::new(3.0)));
    let planet_color = materials.add(Color::rgb(0.0, 0.1, 1.0));
    for i in 0..100 {
        let pos = random_2d_pt(10.0..250.0, 10.0..250.0);

        let planet = commands.spawn(planet(i, &circle, &planet_color, pos)).id();
        commands.entity(sun).push_children(&[planet]);
    }
}

fn planet(
    i: i32,
    circle: &Mesh2dHandle,
    planet_color: &Handle<ColorMaterial>,
    pos: Transform,
) -> (MapShape, Orbit) {
    (
        MapShape {
            entity_type: MapShapeType::Planet,
            name: Name::new(format!("planet_{}", i)),
            shape: MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: planet_color.clone(),
                transform: pos,
                ..default()
            },
        },
        Orbit {
            speed: (500.0 / pos.translation.length()),
        },
    )
}

fn orbit(mut query_planets: Query<(&mut Transform, &Orbit)>) {
    for (mut transform, orbit) in query_planets.iter_mut() {
        let rotation = Quat::from_rotation_z(0.01 * orbit.speed);
        // let mass = astro_body.mass;
        transform.translation = rotation.mul_vec3(transform.translation);
    }
}
