use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

pub struct AmbientSoundPlugin;

#[derive(Component)]
struct MyMusicPlayer;

impl Plugin for AmbientSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostStartup, play_music);
    }
}

fn setup(mut commands: Commands, studio: Res<FmodStudio>) {
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
