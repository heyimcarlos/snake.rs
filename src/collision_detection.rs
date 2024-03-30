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

fn collision_detection(
    // mut commands: Commands,
    snake_head_query: Query<&Position, With<SnakeHead>>,
    snake_body_query: Query<&Position, (With<SnakeSegment>, Without<SnakeHead>)>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((head_pos)) = snake_head_query.get_single() else {
        return;
    };

    // if the snake hits a wall
    if head_pos.x < 0 || head_pos.y < 0 || head_pos.x >= board.size || head_pos.y >= board.size {
        next_state.set(GameState::GameOver);
    }

    // if the snake hits itself
    for segment_pos in snake_body_query.iter() {
        if head_pos == segment_pos {
            next_state.set(GameState::GameOver);
        }
    }
}
