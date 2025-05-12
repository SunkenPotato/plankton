pub mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::PlayerPlugin;

pub static IS_DEBUG: bool = cfg!(debug_assertions);

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(PlayerPlugin);

    if IS_DEBUG {
        warn!("This game was built for debug!");
        warn!("If you are not an end-user, it is highly recommended to use a stable release.");

        app.add_plugins(WorldInspectorPlugin::new());
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
