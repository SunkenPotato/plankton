use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    log::info,
    sprite::Sprite,
};

pub static PLAYER_PATH: &'static str = "player/player.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("Building Player plugin");
        app.add_systems(Startup, init);
    }
}

#[derive(Component)]
#[require(Sprite)]
struct Player;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load(PLAYER_PATH);
    commands.spawn((Player, Sprite::from_image(sprite)));
    commands.spawn(Camera2d);
}
