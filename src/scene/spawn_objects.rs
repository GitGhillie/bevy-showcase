use crate::scene::attract_force::AttractMarker;
use crate::scene::{GameState, SceneAssets};
use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::FmodStudio;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::{Damping, ExternalForce, ExternalImpulse};
use random_branch::branch;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Prop;

pub struct SpawnObjectsPlugin;

impl Plugin for SpawnObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Prop>()
            .add_systems(PostStartup, setup_audio)
            .add_systems(
                Update,
                (spawn_prop, setup_prop, yeet_prop).run_if(in_state(GameState::Next)),
            );
    }
}

fn setup_audio(
    mut commands: Commands,
    studio: Res<FmodStudio>,
    player_query: Query<Entity, With<Camera>>,
) {
    let player = player_query.single();

    let event_description = studio.0.get_event("event:/Weapons/Pistol").unwrap();

    commands
        .entity(player)
        .insert(AudioSource::new(event_description));
}

fn spawn_prop(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    my_assets: Res<SceneAssets>,
    mut audio_query: Query<&AudioSource, With<Camera>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let handle: Handle<Scene>;

        // ooga booga
        branch!(
            handle = my_assets.tire.clone_weak(),
            handle = my_assets.police_car.clone_weak(),
            handle = my_assets.trashcan.clone_weak(),
            handle = my_assets.pallet.clone_weak(),
        );

        // Todo? Set the spawn location somewhere far away until the prop is ready
        commands.spawn(SceneBundle {
            scene: handle.clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });

        for audio_source in audio_query.iter_mut() {
            audio_source.play();
        }
    }
}

fn setup_prop(
    mut commands: Commands,
    prop_query: Query<(Entity, &Children), Added<Prop>>,
    cam_transform: Query<&GlobalTransform, With<Camera>>,
) {
    let cam_transform = cam_transform.single();

    for (prop, children) in prop_query.iter() {
        commands
            .entity(prop)
            .insert(
                Transform::from_translation(
                    cam_transform.translation() + (cam_transform.forward() * 3.0),
                )
                .looking_at(
                    cam_transform.translation() + (cam_transform.forward() * 4.0),
                    cam_transform.up(),
                ),
            )
            .insert(ExternalForce::default())
            .insert(ExternalImpulse::default())
            .insert(Damping::default())
            .insert(On::<Pointer<Down>>::target_commands_mut(
                |click, target_commands| {
                    if click.target != click.listener() && click.button == PointerButton::Primary {
                        target_commands.insert(AttractMarker);
                    }
                },
            ));

        // Quick hack to make the prop pickable, since the parent doesn't have a mesh.
        // Would be more efficient to only add this to the collider mesh.
        for child in children {
            commands
                .entity(*child)
                .insert(PickableBundle::default())
                .insert(RaycastPickTarget::default());
        }
    }
}

// Had to separate this out because for some reason it didn't work
// when I set the ExternalImpulse at spawn.
fn yeet_prop(
    mut impulse_query: Query<
        (&mut ExternalImpulse, &GlobalTransform),
        (Added<ExternalImpulse>, With<Prop>),
    >,
) {
    for (mut impulse_component, global_transform) in impulse_query.iter_mut() {
        let direction = global_transform.forward();
        impulse_component.impulse = direction * 30.0;
    }
}
