use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::{bundle::Bundle, component::Component, system::Commands},
    log::info,
    sprite::Sprite,
};
use bevy_ecs_ldtk::{LdtkEntity, app::LdtkEntityAppExt};

pub static PLAYER_PATH: &str = "player/player.png";
pub static PLAYER_LDTK_IDENT: &str = "Player";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("Building Player plugin");
        app.add_systems(Startup, init);

        app.register_ldtk_entity::<PlayerBundle>(PLAYER_LDTK_IDENT);
    }
}

#[derive(Component, Default)]
#[require(Sprite)]
struct Player;

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(LdtkEntity, Default, Bundle)]
pub struct PlayerBundle {
    _p: Player,
    #[sprite]
    sprite: Sprite,
}
