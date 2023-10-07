use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct PoliceMarker;

#[derive(Component, Default)]
pub(crate) struct Engine {
    rpm: f32,
    load: f32,
}

pub(crate) fn register_types(app: &mut App) {
    app.register_type::<PoliceMarker>();
}

pub(crate) fn insert_audio_sources(
    mut commands: Commands,
    query: Query<Entity, (With<PoliceMarker>, Without<AudioSource>)>,
    studio: Res<FmodStudio>,
) {
    for ent in query.iter() {
        let event_description = studio.0.get_event("event:/Vehicles/Car Engine").unwrap();

        commands
            .entity(ent)
            .insert(AudioSource::new(event_description))
            .insert(Velocity::default())
            .insert(Engine {
                rpm: 3300.0,
                load: 1.0,
            });
    }
}

pub(crate) fn play_sound_on_key(
    mut audio_sources: Query<(&AudioSource, &mut Engine), With<PoliceMarker>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F) {
        for (audio_source, engine) in audio_sources.iter() {
            audio_source.play();
            audio_source
                .event_instance
                .set_parameter_by_name("RPM", 3300.0, false)
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
