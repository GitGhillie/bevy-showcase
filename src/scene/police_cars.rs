use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::SpatialAudioBundle;
use bevy_fmod::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct PoliceMarker;

#[derive(Component, Default)]
pub(crate) struct Engine {
    rpm: f32,
    _load: f32,
}

pub struct PoliceCarPlugin;

impl Plugin for PoliceCarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PoliceMarker>()
            .add_systems(PreUpdate, (setup, play_sound_on_key));
    }
}

fn setup(
    mut commands: Commands,
    query: Query<Entity, (With<PoliceMarker>, Without<AudioSource>)>,
    studio: Res<FmodStudio>,
) {
    for ent in query.iter() {
        // FMOD audio event
        let event_description = studio.0.get_event("event:/Vehicles/Car Engine").unwrap();

        commands
            .entity(ent)
            .insert(SpatialAudioBundle::new(event_description))
            .insert(Engine {
                rpm: 3300.0,
                _load: 1.0,
            });
    }
}

fn play_sound_on_key(
    mut audio_sources: Query<(&AudioSource, &mut Engine), With<PoliceMarker>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F) {
        for (audio_source, _engine) in audio_sources.iter() {
            audio_source.play();
            audio_source
                .event_instance
                .set_parameter_by_name("RPM", 2500.0, false)
                .unwrap();
            audio_source
                .event_instance
                .set_parameter_by_name("Load", 1.0, false)
                .unwrap();
        }
    }

    if input.pressed(KeyCode::Up) {
        for (audio_source, mut engine) in audio_sources.iter_mut() {
            engine.rpm += 5.0;

            audio_source
                .event_instance
                .set_parameter_by_name("RPM", engine.rpm, false)
                .unwrap();
        }
    }

    if input.pressed(KeyCode::Down) {
        for (audio_source, mut engine) in audio_sources.iter_mut() {
            engine.rpm -= 5.0;

            audio_source
                .event_instance
                .set_parameter_by_name("RPM", engine.rpm, false)
                .unwrap();
        }
    }
}
