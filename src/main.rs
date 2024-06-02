use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::prelude::*;

#[derive(Component)]
struct AstroBody {
    mass: u32,
}

#[derive(Component)]
struct Orbit {
    semimajor_axis: f32, //mitad del diámetro ancho
    eccentricity: f32, // [0, 1) (cuan redonda es la elipse, 0 es redondo, 1 es parábola (abierta))
    speed: f32,
}

#[derive(Component)]
enum AstroType {
    Planet,
    Sun,
}

#[derive(Bundle)]
struct AstroBundle {
    astro_type: AstroType,
    marker: AstroBody,
    name: Name,
    // orbit: Orbit,
    // spatial: SpatialBundle,
    shape: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
struct OrbitedBody(Entity);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_solar_system))
        .add_systems(FixedUpdate, orbit)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let sun = commands
        .spawn(AstroBundle {
            astro_type: AstroType::Sun,
            marker: AstroBody { mass: 1_000_000 },
            name: Name::new("Sun"),
            // spatial: SpatialBundle::from_transform(Transform::from_xyz(30.0, 0.0, 0.0)),
            shape: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(15.0))),
                material: materials.add(Color::rgb(0.8, 0.8, 0.1)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        })
        .id();
    for i in 0..100 {
        let x: f32 = rng.gen_range(0.0..250.0);
        let y: f32 = rng.gen_range(0.0..250.0);

        commands.spawn((
            AstroBundle {
                astro_type: AstroType::Planet,
                marker: AstroBody { mass: 500 },
                name: Name::new(format!("planet_{}", i)),
                // spatial: SpatialBundle::from_transform(Transform::from_xyz(30.0, 0.0, 0.0)),
                shape: MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(3.0))),
                    material: materials.add(Color::rgb(0.0, 0.1, 1.0)),
                    transform: Transform::from_xyz(10.0 + x, 10.0 + y, 0.0),
                    ..default()
                },
            },
            Orbit {
                semimajor_axis: 15.0,
                eccentricity: 0.3,
                speed: (500.0 / (x + y)),
            },
            OrbitedBody(sun),
        ));
    }
}

const G: f32 = 6.674e-11;

fn orbit(mut query_planets: Query<(&mut Transform, &Orbit, &AstroBody, &OrbitedBody)>) {
    for (mut transform, orbita, astro_body, sun) in query_planets.iter_mut() {
        let rotation = Quat::from_rotation_z(0.01 * orbita.speed);
        let mass = astro_body.mass;
        transform.translation = rotation.mul_vec3(transform.translation);
    }
}
