use bevy::prelude::*;

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::SpatialAudioBundle;
use bevy_fmod::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct AudioSourceMarker(String);

pub struct InsertAudioPlugin;

impl Plugin for InsertAudioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AudioSourceMarker>()
            .add_systems(Update, insert_audio_sources);
    }
}

/// The audio source markers get added by bevy_gltf_components
pub(crate) fn insert_audio_sources(
    mut commands: Commands,
    query: Query<(Entity, &AudioSourceMarker), Without<AudioSource>>,
    studio: Res<FmodStudio>,
) {
    for (ent, audio_marker) in query.iter() {
        let event_description = studio.0.get_event(&audio_marker.0).unwrap();
        commands
            .entity(ent)
            .insert(SpatialAudioBundle::new(event_description));
    }
}
