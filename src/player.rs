use avian2d::prelude::{Collider, Friction, LinearVelocity, Restitution, RigidBody};
use bevy::{
    app::{FixedUpdate, Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        bundle::Bundle,
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    math::{Dir3, Vec2},
    reflect::Reflect,
    render::camera::{OrthographicProjection, Projection},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::world::{FRICTION, RESTITUTION};

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
        app.add_systems(FixedUpdate, Controller::controller_system);
        app.add_systems(Update, (movement, lock_camera_to_player));

        app.register_ldtk_entity::<PlayerBundle>(PLAYER_LDTK_IDENT);

        app.register_type::<WalkAction>();
        app.register_type::<Controller>();
    }
}

#[derive(Component, Default)]
#[require(Sprite)]
struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    _p: Player,
    sprite: Sprite,
    controller: Controller,
    rigid_body: RigidBody,
    friction: Friction,
    restitution: Restitution,
    collider: Collider,
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
            collider: Collider::rectangle(
                entity_instance.height as f32,
                entity_instance.width as f32,
            ),
            friction: Friction::new(FRICTION),
            restitution: Restitution::new(RESTITUTION),
            ..default()
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct Controller {
    action: Option<WalkAction>,
}

impl Controller {
    pub fn new() -> Self {
        Self { action: None }
    }

    pub fn action(&mut self, action: WalkAction) {
        self.action.replace(action);
    }

    fn apply(&mut self, linear_velocity: &mut LinearVelocity, delta: f32) {
        self.action
            .as_ref()
            .inspect(|a| Self::_apply(linear_velocity, delta, a));
        self.action.take();
    }

    fn _apply(linear_velocity: &mut LinearVelocity, delta: f32, action: &WalkAction) {
        let direction_vector = action
            .direction
            .map(|d| d.as_vec3())
            .unwrap_or_default()
            .truncate();

        if action.acceleration == f32::INFINITY {
            linear_velocity.0 = direction_vector * action.speed;
        } else if action.acceleration == f32::NEG_INFINITY {
            linear_velocity.0 = Vec2::ZERO;
        } else {
            let speed_sqr = linear_velocity.0.length_squared();
            let max_speed_sqr = action.speed * action.speed;

            // Decelerate if the direction is [0, 0] or if the current speed is higher than the max speed specified by the struct
            if direction_vector == Vec2::ZERO && speed_sqr > 0. || speed_sqr >= max_speed_sqr {
                // dv = dt * a
                let velocity_offset = delta * action.acceleration;

                linear_velocity.0.x += velocity_offset
                    * match linear_velocity.0.x < 0. {
                        true => 1.,
                        false => -1.,
                    };

                linear_velocity.0.y += velocity_offset
                    * match linear_velocity.0.y < 0. {
                        true => 1.,
                        false => -1.,
                    };
            } else {
                // otherwise, accelerate
                // v = v + dt * a * dir
                linear_velocity.0 += direction_vector * delta * action.acceleration;
            }
        }
    }

    fn controller_system(
        mut query: Query<(&mut LinearVelocity, &mut Controller)>,
        time: Res<Time>,
    ) {
        let delta = time.delta().as_secs_f32();

        for (mut lv, mut controller) in &mut query {
            controller.apply(&mut lv, delta);
        }
    }
}

#[derive(Reflect)]
pub struct WalkAction {
    pub acceleration: f32,
    pub speed: f32,
    pub direction: Option<Dir3>,
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

fn movement(mut controller: Single<&mut Controller, With<Player>>, kb: Res<ButtonInput<KeyCode>>) {
    let direction = if kb.pressed(KEY_UP) {
        Some(Dir3::Y)
    } else if kb.pressed(KEY_DOWN) {
        Some(Dir3::NEG_Y)
    } else if kb.pressed(KEY_LEFT) {
        Some(Dir3::NEG_X)
    } else if kb.pressed(KEY_RIGHT) {
        Some(Dir3::X)
    } else {
        None
    };

    let acc_multiple = direction.map_or(1., |a| match a {
        Dir3::Y => ACC_MULTIPLE_UPWARDS,
        _ => 1.,
    });

    controller.action(WalkAction {
        direction,
        speed: SPEED,
        acceleration: ACCELERATION * acc_multiple,
    })
}

fn lock_camera_to_player(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    **camera = **player;
}
