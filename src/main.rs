mod graphics;
mod player;
mod replace_colliders;
mod scene;

use bevy::prelude::*;

use bevy::window::Cursor;
use bevy::window::CursorGrabMode;

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

use crate::replace_colliders::ReplaceColliderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //present_mode: bevy::window::PresentMode::AutoNoVsync,
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
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ReplaceColliderPlugin)
        .add_plugins(FmodPlugin {
            audio_banks_paths: &[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
                "./assets/audio/demo_project/Build/Desktop/SFX.bank",
                "./assets/audio/demo_project/Build/Desktop/Vehicles.bank",
            ],
        })
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.0.get_event("event:/Ambience/City").unwrap();

    commands
        .spawn(MyMusicPlayer)
        .insert(AudioSource::new(event_description));
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    let source = audio_sources.single_mut();
    source.play();
    source
        .event_instance
        .set_parameter_by_name("Traffic", 0.5, false)
        .unwrap();
}

pub fn low_latency_window_plugin() -> bevy::window::WindowPlugin {
    bevy::window::WindowPlugin {
        primary_window: Some(bevy::window::Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..Default::default()
        }),
        ..Default::default()
    }
}
