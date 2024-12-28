use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

use camera::{center_camera_on_player, setup_camera};
use input::{init_cursor_world_coords, update_cursor_world_coords};
use level::LevelManagementPlugin;
use player::{movement::move_player, PlayerManagementPlugin};

mod camera;
mod input;
mod level;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Lightborne".into(),
                        name: Some("lightborne".into()),
                        present_mode: PresentMode::AutoNoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerManagementPlugin)
        .add_plugins(LevelManagementPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, center_camera_on_player.after(move_player))
        .add_systems(Startup, init_cursor_world_coords)
        .add_systems(Update, update_cursor_world_coords)
        .run();
}
