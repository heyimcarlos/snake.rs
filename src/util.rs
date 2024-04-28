use bevy::math::Vec2;
use rand::Rng;

use crate::snake::{Direction, Position};

pub fn snake_starting_position(board_size: i32) -> Vec<Position> {
    let start_x = board_size / 2 - 5;
    let start_y = board_size / 2;

    vec![
        Position {
            x: start_x,
            y: start_y,
        },
        Position {
            x: start_x - 1,
            y: start_y,
        },
        Position {
            x: start_x - 2,
            y: start_y,
        },
    ]
}

pub fn food_position(board_size: i32) -> Position {
    let mut rng = rand::thread_rng();
    Position {
        x: rng.gen_range(5..board_size - 5),
        y: rng.gen_range(5..board_size - 5),
    }
}

pub fn _calc_sprite_index(row: usize, column: usize, columns_per_row: usize) -> usize {
    row * columns_per_row + column
}

pub fn _detect_direction(from: &Position, to: &Position) -> Direction {
    if to.y > from.y {
        Direction::Up
    } else if to.y < from.y {
        Direction::Down
    } else if to.x > from.x {
        Direction::Right
    } else if to.x < from.x {
        Direction::Left
    } else {
        dbg!(from, to);
        panic!("Invalid direction")
    }
}

//  NOTE:  takes a vectorized input from a joystick or touchscreen and crush it down into our binary input format
const DEADZONE: f32 = 50.0;
const AXIS_DEADZONE: f32 = 0.2;
//  NOTE: magic pre calulated normalized variable for when x and y are both 1
const DIAGONAL_NORMALIZED: f32 = 0.707107;
const UNIT_TL: Vec2 = Vec2 {
    x: DIAGONAL_NORMALIZED,
    y: DIAGONAL_NORMALIZED,
};
const UNIT_TR: Vec2 = Vec2 {
    x: -DIAGONAL_NORMALIZED,
    y: DIAGONAL_NORMALIZED,
};
const UNIT_BL: Vec2 = Vec2 {
    x: DIAGONAL_NORMALIZED,
    y: -DIAGONAL_NORMALIZED,
};
const UNIT_BR: Vec2 = Vec2 {
    x: -DIAGONAL_NORMALIZED,
    y: -DIAGONAL_NORMALIZED,
};
pub fn direction_from_vec2(vec: Vec2) -> Vec<Direction> {
    let mut direction_queue: Vec<Direction> = vec![];
    let magnitude = vec.length();
    if magnitude > DEADZONE {
        let dir = vec.normalize_or_zero();

        let left = dir.distance_squared(Vec2::X);
        let topleft = dir.distance_squared(UNIT_TL);
        let top = dir.distance_squared(Vec2::Y);
        let topright = dir.distance_squared(UNIT_TR);
        let right = dir.distance_squared(-Vec2::X);
        let bottomleft = dir.distance_squared(UNIT_BL);
        let bottom = dir.distance_squared(-Vec2::Y);
        let bottomright = dir.distance_squared(UNIT_BR);

        if top < AXIS_DEADZONE {
            direction_queue.push(Direction::Up);
            return direction_queue;
        } else if bottom < AXIS_DEADZONE {
            direction_queue.push(Direction::Down);
            return direction_queue;
        }

        if left < right {
            if left < AXIS_DEADZONE {
                //  NOTE: Vertical axis deadzone going left.
                direction_queue.push(Direction::Left);
            } else if topleft < left {
                direction_queue.push(Direction::Left);
                direction_queue.push(Direction::Up);
            } else if bottomleft < left {
                direction_queue.push(Direction::Left);
                direction_queue.push(Direction::Down);
            }
        } else {
            if right < AXIS_DEADZONE {
                //  NOTE: Vertical axis deadzone going right.
                direction_queue.push(Direction::Right)
            } else if topright < left {
                direction_queue.push(Direction::Right);
                direction_queue.push(Direction::Up);
            } else if bottomright < left {
                direction_queue.push(Direction::Right);
                direction_queue.push(Direction::Down);
            }
        }
    }
    direction_queue
}
