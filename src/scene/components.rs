use bevy::prelude::*;

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct AudioSourceMarker(String);

pub(crate) fn register_types(app: &mut App) {
    app.register_type::<AudioSourceMarker>();
}

pub(crate) fn insert_audio_sources(
    mut commands: Commands,
    query: Query<(Entity, &AudioSourceMarker)>,
    studio: Res<FmodStudio>,
) {
    for (ent, audio_marker) in query.iter() {
        let event_description = studio.0.get_event(&*audio_marker.0).unwrap();

        commands
            .entity(ent)
            .insert(AudioSource::new(event_description))
            .remove::<AudioSourceMarker>();
    }
}
