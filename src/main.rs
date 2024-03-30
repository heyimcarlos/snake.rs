mod board;
mod collision_detection;
mod colors;
mod food;
mod map;
mod schedule;
mod snake;
mod util;

use bevy::prelude::*;
use board::BoardPlugin;
use collision_detection::CollisionDetectionPlugin;
use food::FoodPlugin;
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
        .add_systems(Startup, setup)
        .add_plugins(BoardPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(CollisionDetectionPlugin)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
