mod asset_loader;
mod board;
mod camera;
mod collision_detection;
mod colors;
mod despawn;
mod food;
mod schedule;
mod snake;
mod state;
mod ui;
mod util;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use board::BoardPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use food::FoodPlugin;
use schedule::SchedulePlugin;
use snake::SnakePlugin;
use state::StatePlugin;
use ui::GameUiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.52, 0.73, 0.17)))
        .add_plugins(DefaultPlugins)
        .add_plugins(GameUiPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
