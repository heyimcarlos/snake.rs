mod asset_loader;
mod board;
mod collision_detection;
mod colors;
mod despawn;
mod food;
mod schedule;
mod snake;
mod state;
mod util;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use board::BoardPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use food::FoodPlugin;
use schedule::SchedulePlugin;
use snake::SnakePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.52, 0.73, 0.17)))
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_systems(Startup, setup)
        .add_plugins(BoardPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
