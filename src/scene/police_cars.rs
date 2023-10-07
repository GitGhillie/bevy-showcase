use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct PoliceMarker;

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
            .insert(Velocity::default());
    }
}

pub(crate) fn play_sound_on_key(
    audio_sources: Query<&AudioSource, With<PoliceMarker>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F) {
        println!("Just pressed");
        for audio_source in audio_sources.iter() {
            println!("starting");
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
}
