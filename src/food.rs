use bevy::prelude::*;

use crate::{
    board::{Board, TILE_SIZE},
    snake::Position,
    util::food_position,
};

#[derive(Component, Debug)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
    }
}

pub fn spawn_food(mut commands: Commands, board: Res<Board>) {
    let food_pos = food_position(board.size);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                board.position_translate(food_pos.x.into()),
                board.position_translate(food_pos.x.into()),
                1.0,
            ),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(TILE_SIZE - 2., TILE_SIZE - 2.)),
                ..default()
            },
            ..Default::default()
        },
        Food,
        Position::from(food_pos),
    ));
}
