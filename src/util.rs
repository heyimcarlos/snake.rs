use rand::Rng;

use crate::snake::{Direction, Position};

pub fn snake_starting_position(board_size: i32) -> Vec<Position> {
    // let mut rng = rand::thread_rng();
    // let start_x = rng.gen_range(5..board_size - 5);
    // let start_y = rng.gen_range(5..board_size - 5);
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

pub fn calc_sprite_index(row: usize, column: usize, columns_per_row: usize) -> usize {
    row * columns_per_row + column
}

pub fn detect_direction(origin: &Position, next: &Position) -> Direction {
    if next.y > origin.y {
        Direction::Up
    } else if next.y < origin.y {
        Direction::Down
    } else if next.x > origin.y {
        Direction::Right
    } else if next.x < origin.x {
        Direction::Left
    } else {
        panic!("Invalid direction")
    }
}
