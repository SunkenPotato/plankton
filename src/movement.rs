use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::{
    app::{FixedPostUpdate, Plugin},
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::{Dir2, Vec2},
    reflect::Reflect,
    time::Time,
};

use crate::register_types;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        register_types!(app, Controller, Acceleration, Direction, Speed);

        app.add_systems(FixedPostUpdate, apply_controller);
    }
}

#[derive(Component, Reflect, Default)]
#[require(Acceleration, Direction, Speed, Collider, RigidBody)]
pub struct Controller;

#[derive(Component, Reflect, Default)]
pub struct Acceleration(pub f32);

#[derive(Component, Reflect, Default)]
pub struct Direction(pub Option<Dir2>);

#[derive(Component, Reflect, Default)]
pub struct Speed(pub f32);

#[derive(Bundle, Default)]
pub struct ControllerBundle {
    pub controller: Controller,
    pub acceleration: Acceleration,
    pub direction: Direction,
    pub speed: Speed,
}

impl ControllerBundle {
    pub const fn new(acceleration: f32, speed: f32, direction: Option<Dir2>) -> Self {
        Self {
            controller: Controller,
            acceleration: Acceleration(acceleration),
            speed: Speed(speed),
            direction: Direction(direction),
        }
    }
}

fn apply_controller(
    mut query: Query<(&Acceleration, &Speed, &Direction, &mut LinearVelocity), With<Controller>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();

    for (acceleration, speed, direction, mut linear_velocity) in &mut query {
        let direction_vec2 = direction.0.map(|d| d.as_vec2()).unwrap_or(Vec2::ZERO);
        let velocity_squared = linear_velocity.length_squared();

        if velocity_squared < speed.0 * speed.0 {
            linear_velocity.0 += (acceleration.0 * delta) * direction_vec2;
        }
    }
}
