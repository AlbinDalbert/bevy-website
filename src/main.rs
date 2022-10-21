use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod config;
mod main_camera;

fn main() {
    App::new()
        .insert_resource(ClearColor(config::CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: "WASM APP".to_string(),
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(main_camera::MainCameraPlugin)
        .run();
}
