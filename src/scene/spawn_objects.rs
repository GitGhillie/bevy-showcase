use crate::scene::attract_force::AttractMarker;
use crate::scene::{GameState, SceneAssets};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::{Damping, ExternalForce};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Prop;

pub struct SpawnObjectsPlugin;

impl Plugin for SpawnObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Prop>().add_systems(
            Update,
            (spawn_prop, setup_prop).run_if(in_state(GameState::Next)),
        );
    }
}

fn spawn_prop(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    my_assets: Res<SceneAssets>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        // todo: probably set the spawn location somewhere far away until the prop is ready
        commands.spawn(SceneBundle {
            scene: my_assets.trashcan.clone_weak(),
            ..default()
        });
    }
}

fn setup_prop(mut commands: Commands, prop_query: Query<(Entity, &Children), Added<Prop>>) {
    for (prop, children) in prop_query.iter() {
        commands
            .entity(prop)
            .insert(ExternalForce::default())
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
