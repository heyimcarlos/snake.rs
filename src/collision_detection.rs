use bevy::prelude::*;

use crate::{
    board::{Board, TILE_SIZE},
    food::{spawn_food, Food},
    schedule::InGameSet,
    snake::{Position, SnakeDirection, SnakeHead, SnakeSegment},
};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, eat_food.in_set(InGameSet::CollisionDetection));
    }
}

// the food is eaten, if both the food and snake head are in the same position.
fn eat_food(
    mut commands: Commands,
    snake_head_query: Query<(&Transform, &SnakeDirection), With<SnakeHead>>,
    snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
    food_query: Query<(&Transform, Entity), (With<Food>, Without<SnakeSegment>)>,
    board: Res<Board>,
) {
    let Ok((head_transform, _)) = snake_head_query.get_single() else {
        return;
    };

    let Ok((food_tranform, entity)) = food_query.get_single() else {
        return;
    };

    if head_transform.translation != food_tranform.translation {
        return;
    }

    // food eaten, despawn food
    commands.entity(entity).despawn();

    // enlarge snake
    // @todo: implement a eat_food_event that spawns new food and enlarges the snake.
    let Some((tail_pos, _)) = snake_body_query.iter().last() else {
        return;
    };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                board.position_translate(tail_pos.x.into()),
                board.position_translate(tail_pos.y.into()),
                10.0,
            ),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
        Position::new(tail_pos.x, tail_pos.y),
    ));

    spawn_food(commands, board);
}
