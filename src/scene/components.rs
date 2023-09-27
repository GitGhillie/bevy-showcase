use bevy::prelude::*;

use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

use bevy_eventlistener::prelude::*;
use bevy_mod_picking::prelude::*;

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
            // .insert((
            //     PickableBundle::default(),
            //     RaycastPickTarget::default(),
            //     On::<Pointer<Over>>::run(|event: Listener<Pointer<Over>>| {
            //         info!("Out {:?}", event.target);
            //     }),
            // ))
            .insert(AudioSource::new(event_description))
            .remove::<AudioSourceMarker>();
    }
}
