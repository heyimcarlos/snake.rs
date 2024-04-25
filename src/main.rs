mod asset_loader;
mod board;
mod camera;
mod collision_detection;
mod colors;
mod despawn;
mod food;
mod schedule;
mod score;
mod snake;
mod state;
mod ui;
mod util;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use board::BoardPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use food::FoodPlugin;
use schedule::SchedulePlugin;
use score::ScorePlugin;
use snake::SnakePlugin;
use state::StatePlugin;
use ui::GameUiPlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::hex("#578a34").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // canvas: Some("#snake-canvas".into()),
                prevent_default_event_handling: false,
                resizable: false,
                resolution: WindowResolution::new(650.0, 750.0),
                mode: bevy::window::WindowMode::Windowed,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GameUiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
