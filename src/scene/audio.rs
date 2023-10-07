use bevy::prelude::*;

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

use bevy_eventlistener::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct AudioSourceMarker(String);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct Train;

pub(crate) fn register_types(app: &mut App) {
    app.register_type::<AudioSourceMarker>();
    app.register_type::<Train>();
}

// Assuming that all audio sources are a child of a mesh (i.e. it's clickable)
pub(crate) fn insert_audio_sources(
    mut commands: Commands,
    query: Query<(Entity, &AudioSourceMarker), Without<AudioSource>>,
    studio: Res<FmodStudio>,
) {
    for (ent, audio_marker) in query.iter() {
        let event_description = studio.0.get_event(&*audio_marker.0).unwrap();
        commands
            .entity(ent)
            .insert(AudioSource::new(event_description))
            .insert(Velocity::default());
    }
}

#[derive(Event)]
pub(crate) struct DoSomethingComplex(Entity, f32);

impl From<ListenerInput<Pointer<Down>>> for DoSomethingComplex {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        DoSomethingComplex(event.target, event.hit.depth)
    }
}

pub(crate) fn play_sound_on_click(
    mut greetings: EventReader<DoSomethingComplex>,
    parent_query: Query<&Parent>,
    parent_components: Query<&AudioSource>,
) {
    for event in greetings.iter() {
        // The event gives us the mesh that was clicked, the audio source is on the parent
        let parent = parent_query.get(event.0).unwrap().get();
        let source = parent_components.get(parent).unwrap();
        source.play();
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
