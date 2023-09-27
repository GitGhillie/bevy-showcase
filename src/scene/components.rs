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
    query: Query<(Entity, &AudioSourceMarker, &Children)>,
    studio: Res<FmodStudio>,
) {
    for (ent, audio_marker, children) in query.iter() {
        let event_description = studio.0.get_event(&*audio_marker.0).unwrap();

        commands
            .entity(ent)
            .insert((
                PickableBundle::default(),
                RaycastPickTarget::default(),
                On::<Pointer<Down>>::send_event::<DoSomethingComplex>(),
                // On::<Pointer<Over>>::run(|event: Listener<Pointer<Over>>| {
                //     info!("Out {:?}", event.target);
                // }),
            ))
            .insert(AudioSource::new(event_description))
            .remove::<AudioSourceMarker>();

        // The goal here is to add PickableBundle and RaycastPickTarget to the entity with
        // the mesh for mod_picking to work. But with bevy_mod_picking the components you add in
        // Blender are one level higher in the hierarchy, hence this hack.
        for &child in children {
            commands
                .entity(child)
                .insert((PickableBundle::default(), RaycastPickTarget::default()));
        }
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
    parent_query: Query<(&Parent)>,
    parent_components: Query<&AudioSource>,
) {
    for event in greetings.iter() {
        // The event gives us the mesh that was clicked, the audio source is on the parent
        let parent = parent_query.get(event.0).unwrap().get();
        let source = parent_components.get(parent).unwrap();
        source.play();
    }
}
