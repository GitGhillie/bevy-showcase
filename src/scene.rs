mod components;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::graphics;

pub struct SceneLoader;

impl Plugin for SceneLoader {
    fn build(&self, app: &mut App) {
        components::register_types(app);

        app.add_plugins(ComponentsFromGltfPlugin)
            .add_plugins(
                DefaultPickingPlugins
                    .build()
                    .disable::<DefaultHighlightingPlugin>()
                    .disable::<DebugPickingPlugin>(),
            )
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
            )
            .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
            .add_event::<components::DoSomethingComplex>()
            .add_systems(OnEnter(GameState::Next), use_my_assets)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    components::insert_audio_sources,
                    components::play_sound_on_key,
                ),
            );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "level.glb#Scene0")]
    scene: Handle<Scene>,
    #[asset(path = "detail.glb#Scene0")]
    detail: Handle<Scene>,
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.35, 0.25), Vec3::Y),
        cascade_shadow_config: graphics::create_cascade_shadow_config(),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(800.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

fn use_my_assets(mut commands: Commands, my_assets: Res<MyAssets>) {
    commands.spawn((
        SceneBundle {
            scene: my_assets.scene.clone_weak(),
            ..default()
        },
        AsyncSceneCollider::default(),
    ));

    commands.spawn((
        SceneBundle {
            scene: my_assets.detail.clone_weak(),
            ..default()
        },
        Name::from("AAAA"),
    ));
}
