use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};

#[derive(Component, Debug)]
struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
    }
}

fn spawn_food(mut commands: Commands, board: Res<Board>) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            board.position_translate(15),
            board.position_translate(15),
            1.0,
        ),
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(TILE_SIZE - 2., TILE_SIZE - 2.)),
            ..default()
        },
        ..Default::default()
    });
}
