use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    utils::default,
};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

#[cfg(debug_assertions)]
pub static WORLD_PATH: &'static str = "world/testworld.ldtk";

#[cfg(not(debug_assertions))]
pub static WORLD_PATH: &'static str = compile_error!("There is no world file at this release yet.");

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(LevelSelection::index(0));

        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load(WORLD_PATH).into();

    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..default()
    });
}
