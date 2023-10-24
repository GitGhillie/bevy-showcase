use crate::scene::audio::AudioSourceMarker;
use bevy::prelude::*;

use bevy_fmod::prelude::AudioSource;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct Train;

pub struct TrainsPlugin;

impl Plugin for TrainsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Train>()
            .add_systems(Update, play_sound_on_key);
    }
}

pub(crate) fn play_sound_on_key(
    audio_sources: Query<&AudioSource, With<AudioSourceMarker>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::E) {
        for audio_source in audio_sources.iter() {
            audio_source.play();
            audio_source
                .event_instance
                .set_parameter_by_name("RPM", 420.0, false)
                .unwrap();
            audio_source
                .event_instance
                .set_parameter_by_name("Load", 1.0, false)
                .unwrap();
        }
    }
}
