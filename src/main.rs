use bevy::{
    DefaultPlugins,
    app::App,
    log::{info, warn},
};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub static IS_DEBUG: bool = cfg!(debug_assertions);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(LdtkPlugin);

    if IS_DEBUG {
        warn!("This game was built for debug!");
        warn!("If you are not an end-user, it is highly recommended to use a stable release.");

        app.add_plugins(WorldInspectorPlugin::new());
    }

    info!("Exiting");
}
