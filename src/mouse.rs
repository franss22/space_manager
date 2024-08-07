use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorPos {
    pub pos: Vec3,
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(FixedPreUpdate, calc_cursor_pos);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Make sure to add the marker component when you set up your camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(CursorPos::default())
}

fn calc_cursor_pos(
    mut mycoords: ResMut<CursorPos>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.pos.x = world_position.x;
        mycoords.pos.y = world_position.y;

        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
