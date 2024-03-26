mod board;
mod colors;
mod map;
mod snake;

use bevy::prelude::*;
use board::BoardPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.52, 0.73, 0.17)))
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(BoardPlugin)
        .add_plugins(SnakePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // the default projection is 1000 far, -1000 near
    commands.spawn(Camera2dBundle::default());
}
