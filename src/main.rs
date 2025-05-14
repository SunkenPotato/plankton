pub mod player;
pub mod world;

use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use player::PlayerPlugin;
use world::WorldPlugin;

pub static IS_DEBUG: bool = cfg!(debug_assertions);

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins((PlayerPlugin, WorldPlugin))
        .add_plugins(PhysicsPlugins::default());

    if IS_DEBUG {
        warn!("This game was built for debug!");
        warn!("If you are an end-user, it is highly recommended to use a stable release.");

        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
            PhysicsDebugPlugin::default(),
        ));
    }

    app.add_systems(Last, exit_hook);

    app.run()
}

fn exit_hook(mut event: EventReader<AppExit>) {
    for _ in event.read() {
        info!("Exiting");
        // add exit logic here later
    }
}
