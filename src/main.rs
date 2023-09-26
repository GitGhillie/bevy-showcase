mod player;
mod scene;

use bevy::prelude::*;

use bevy::window::CursorGrabMode;
use bevy::{prelude::*, window::Cursor};

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

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
        .add_plugins(FmodPlugin {
            audio_banks_paths: &[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ],
        })
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.0.get_event("event:/Music/Level 03").unwrap();

    commands
        .spawn(MyMusicPlayer)
        .insert(AudioSource::new(event_description));
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single_mut().play();
}
