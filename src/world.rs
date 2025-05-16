use avian2d::prelude::{Collider, Friction, Restitution, RigidBody};
use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Res},
    },
    reflect::Reflect,
    utils::default,
};
use bevy_ecs_ldtk::{
    IntGridCell, LdtkWorldBundle, LevelSelection,
    app::{LdtkIntCell, LdtkIntCellAppExt},
};

use crate::debug_value;

pub static WORLD_PATH: &str =
    debug_value!("testworld.ldtk", compile_error!("There is no world yet."));

pub static TILE_LAYER_ID: &str = "Tiles";

const SAND_ID: i32 = 1;

pub const RESTITUTION: f32 = 0.1;
pub const FRICTION: f32 = 0.25;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(LevelSelection::index(0));

        app.add_systems(Startup, spawn_world);

        app.register_ldtk_int_cell::<TileBundle>(SAND_ID);
    }
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load(WORLD_PATH).into();

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..default()
    });
}

#[derive(Component, Default)]
struct TileMarker;

#[derive(Component, Default)]
struct PhysicsBundleMarker;

#[derive(Bundle, Default)]
struct TileBundle {
    _m: TileMarker,
    collider: Collider,
    rigid_body: RigidBody,
    friction: Friction,
    restitution: Restitution,
    ig_val: IntGridValue,
}

#[derive(Component, Reflect, Default, Debug)]
pub struct IntGridValue(i32);

impl LdtkIntCell for TileBundle {
    fn bundle_int_cell(
        int_grid_cell: IntGridCell,
        layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
    ) -> Self {
        let grid_size = layer_instance.grid_size as f32;

        Self {
            collider: Collider::rectangle(grid_size, grid_size),
            rigid_body: RigidBody::Static,
            restitution: Restitution::new(0.1),
            friction: Friction::new(0.25),
            ig_val: IntGridValue(int_grid_cell.value),
            ..default()
        }
    }
}
