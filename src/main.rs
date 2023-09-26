mod player;
mod scene;

use bevy::prelude::*;

use bevy::window::CursorGrabMode;
use bevy::{prelude::*, window::Cursor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: Cursor {
                    visible: false,
                    grab_mode: CursorGrabMode::Locked,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(scene::SceneLoader)
        .add_plugins(player::PlayerPlugin)
        .run();
}
