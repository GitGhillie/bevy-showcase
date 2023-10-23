use crate::scene::attract_force::AttractMarker;
use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::{ExternalForce, RigidBody};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub(crate) struct PoliceMarker;

#[derive(Component, Default)]
pub(crate) struct Engine {
    rpm: f32,
    load: f32,
}

pub struct PoliceCarPlugin;

impl Plugin for PoliceCarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PoliceMarker>()
            .add_systems(Update, (setup, play_sound_on_key, propagate_attract_marker));
    }
}

fn setup(
    mut commands: Commands,
    query: Query<(Entity, &Children), (With<PoliceMarker>, Without<AudioSource>)>,
    //child_query: Query<>
    studio: Res<FmodStudio>,
) {
    for (ent, children) in query.iter() {
        // FMOD audio event
        let event_description = studio.0.get_event("event:/Vehicles/Car Engine").unwrap();

        commands
            .entity(ent)
            .insert(AudioSource::new(event_description))
            .insert(Velocity::default())
            .insert(Engine {
                rpm: 3300.0,
                load: 1.0,
            })
            .insert(ExternalForce::default())
            .insert(On::<Pointer<Down>>::target_commands_mut(
                |click, target_commands| {
                    if click.target != click.listener() && click.button == PointerButton::Primary {
                        target_commands.insert(AttractMarker);
                        println!("Attract!");
                    }
                },
            ));

        // Quick hack to make the car pickable, since the parent doesn't have a mesh.
        // Would be more efficient to only add this to the collider mesh.
        for child in children {
            commands
                .entity(*child)
                .insert(PickableBundle::default())
                .insert(RaycastPickTarget::default());
        }
    }
}

// Because the attract marker gets inserted on the child mesh that was clicked (bug?)
// we have to propagate it up to the actual police car entity.
fn propagate_attract_marker(
    mut commands: Commands,
    child_query: Query<(Entity, &Parent), With<AttractMarker>>,
    parent_query: Query<Entity, (Without<AttractMarker>, With<RigidBody>)>,
) {
    for (child, childs_parent) in &child_query {
        if let Ok(parent) = parent_query.get(childs_parent.get()) {
            commands.entity(parent).insert(AttractMarker);
            commands.entity(child).remove::<AttractMarker>();
        }
    }
}

fn play_sound_on_key(
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
