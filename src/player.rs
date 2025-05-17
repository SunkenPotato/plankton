use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        bundle::Bundle,
        component::Component,
        query::{With, Without},
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    math::Dir2,
    render::camera::{OrthographicProjection, Projection},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::{
    movement::{Acceleration, Controller, ControllerBundle, Direction},
    world::{FRICTION, PhysicsBundle, RESTITUTION},
};

pub static PLAYER_PATH: &str = "player/player.png";
pub static PLAYER_LDTK_IDENT: &str = "Player";

const KEY_UP: KeyCode = KeyCode::KeyW;
const KEY_DOWN: KeyCode = KeyCode::KeyS;
const KEY_LEFT: KeyCode = KeyCode::KeyA;
const KEY_RIGHT: KeyCode = KeyCode::KeyD;

const SPEED: f32 = 30.;
const ACCELERATION: f32 = 10.;
const ACC_MULTIPLE_UPWARDS: f32 = 2.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("Building Player plugin");
        app.add_systems(Startup, init);
        app.add_systems(Update, (lock_camera_to_player, keyboard_movement));
        app.register_ldtk_entity::<PlayerBundle>(PLAYER_LDTK_IDENT);
    }
}

#[derive(Component, Default)]
#[require(Sprite)]
struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    _p: Player,
    sprite: Sprite,
    physics_bundle: PhysicsBundle,
    controller_bundle: ControllerBundle,
}

impl bevy_ecs_ldtk::prelude::LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &bevy_ecs_ldtk::prelude::EntityInstance,
        _: &bevy_ecs_ldtk::prelude::LayerInstance,
        tileset: Option<&bevy::prelude::Handle<bevy::prelude::Image>>,
        _: Option<&bevy_ecs_ldtk::prelude::TilesetDefinition>,
        _: &bevy::prelude::AssetServer,
        _: &mut bevy::prelude::Assets<bevy::prelude::TextureAtlasLayout>,
    ) -> Self {
        Self {
            sprite: bevy_ecs_ldtk::utils::sprite_from_entity_info(tileset),
            physics_bundle: PhysicsBundle::new(
                Collider::rectangle(entity_instance.width as f32, entity_instance.height as f32),
                RigidBody::Dynamic,
                FRICTION,
                RESTITUTION,
                true,
            ),
            controller_bundle: ControllerBundle::new(ACCELERATION, SPEED, None),
            ..default()
        }
    }
}

fn init(mut commands: Commands) {
    const CAMERA_SCALE: f32 = 1. / 5.;

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn keyboard_movement(
    player: Single<(&mut Acceleration, &mut Direction), With<Controller>>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let (mut acceleration, mut direction) = player.into_inner();

    let (acceleration_multiple, new_direction) = if kb.pressed(KEY_UP) {
        (ACC_MULTIPLE_UPWARDS, Some(Dir2::Y))
    } else if kb.pressed(KEY_DOWN) {
        (1., Some(Dir2::NEG_Y))
    } else if kb.pressed(KEY_RIGHT) {
        (1., Some(Dir2::X))
    } else if kb.pressed(KEY_LEFT) {
        (1., Some(Dir2::NEG_X))
    } else {
        (1., None)
    };

    direction.0 = new_direction;
    acceleration.0 = ACCELERATION * acceleration_multiple;
}

fn lock_camera_to_player(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    **camera = **player;
}
