use bevy::prelude::*;
use crate::config::*;
//mod config;

pub struct MainCameraPlugin;

#[derive(Component)]
struct MainCamera;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        // transform: Transform {
        //     translation: Vec3::new(0.0, 0.0, 999.0),
        //     ..Default::default()
        // },
        projection: OrthographicProjection {
            left: -(WINDOW_WIDTH / 20.0),
            right: (WINDOW_WIDTH / 20.0),
            top: -(WINDOW_HEIGHT / 20.0),
            bottom: (WINDOW_HEIGHT / 20.0),
            ..Default::default()
        }
        .into(),
        ..default()
    });
}