use bevy::prelude::*;

use crate::{
    board::Board,
    schedule::InGameSet,
    snake::{Position, SnakeHead, SnakeSegment},
    state::GameState,
};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        );
    }
}
//  NOTE: Maybe I can have a handle_collision event handler that delegates different kind of
//  events?
//  - Maybe the CollisionEvent has two entities and the second one is an option? if it's not
//  present, then that's a snake hit wall event?
//  Collisions
//  1. Snake and SnakeBody
//  2. Snake and Food
//  3. Snake and wall (not an entity)

// #[derive(Event, Debug)]
// struct CollisionEvent {
//     entity: Entity,
//     collided_entity: Option<Entity>,
// }
//
// impl CollisionEvent {
//     fn new(entity: Entity, collided_entity: Option<Entity>) -> Self {
//         Self {
//             entity,
//             collided_entity,
//         }
//     }
// }

fn collision_detection(
    snake_head_query: Query<(&Position, Entity), With<SnakeHead>>,
    snake_body_query: Query<(&Position, Entity), (With<SnakeSegment>, Without<SnakeHead>)>,
    // food_query: Query<(&Transform, Entity), With<Food>>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((head_pos, _)) = snake_head_query.get_single() else {
        return;
    };

    //  NOTE:the snake hits a wall
    if head_pos.x < 0 || head_pos.y < 0 || head_pos.x == board.size || head_pos.y == board.size {
        next_state.set(GameState::GameOver);
        // CollisionEvent::new(head_entity, None);
    }

    // NOTE: the snake hits itself
    for (segment_pos, _) in snake_body_query.iter() {
        if head_pos == segment_pos {
            next_state.set(GameState::GameOver);
            // CollisionEvent::new(head_entity, Some(segment_entity));
        }
    }

    //  TODO: add collision with food
}
