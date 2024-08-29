use bevy::prelude::*;
use bevy::{math::Vec2, prelude::Component};
use bevy_prototype_lyon::prelude::*;

#[derive(Component, Debug)]

pub struct EllipseOrbit {
    pub a_semimajor: f32,
    pub b_semiminor: f32,
    pub rotation: f32,
    pub start_t: i128,
}

impl EllipseOrbit {
    fn new(a: f32, b: f32, rotation: f32) -> Self {
        Self {
            a_semimajor: a,
            b_semiminor: b,
            rotation,
        }
    }

    fn eccentricity(&self) -> f32 {
        (1.0 - self.b_semiminor.powi(2) / self.a_semimajor.powi(2)).sqrt()
    }

    fn point(&self, t: f32) -> Vec2 {
        let x = self.a_semimajor * t.cos();
        let y = self.b_semiminor * t.sin();
        let x_rot = x * self.rotation.cos() - y * self.rotation.sin();
        let y_rot = x * self.rotation.sin() + y * self.rotation.cos();
        Vec2::new(x_rot, y_rot)
    }
}

fn spawn_orbit(mut commands: Commands) {
    let orbit = EllipseOrbit::new(70.0, 50.0, 1.0);
    let shape = orbit_shape(orbit);
    commands.spawn(shape);
}

fn orbit_shape(
    orbit: EllipseOrbit,
) -> (
    bevy_prototype_lyon::entity::ShapeBundle,
    bevy_prototype_lyon::draw::Fill,
    bevy_prototype_lyon::draw::Stroke,
) {
    let mut transform = Transform::from_xyz(orbit.a_semimajor * orbit.eccentricity(), 0.0, 0.0);
    transform.rotate_around(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::from_rotation_z(orbit.rotation),
    );

    let shape = shapes::Ellipse {
        radii: Vec2::new(orbit.a_semimajor, orbit.b_semiminor),
        ..Default::default()
    };

    let ellipse_bundle = ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        spatial: SpatialBundle {
            transform,
            ..Default::default()
        },
        ..Default::default()
    };

    return (
        ellipse_bundle,
        Fill::color(Color::NONE),
        Stroke::new(Color::WHITE, 0.5),
    );
}

pub struct OrbitPlugin;
impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_orbit);
    }
}
