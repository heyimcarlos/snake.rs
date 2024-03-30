use bevy::prelude::*;

use crate::{
    board::{Board, TILE_SIZE},
    util::snake_starting_position,
};

const SNAKE_SPEED: f32 = 30.0;

// #[derive(Component, Debug)]
// pub struct SnakeDirection {
//     pub value: Direction,
//     pub next_value: Direction,
// }

// impl Default for SnakeDirection {
//     fn default() -> Self {
//         SnakeDirection {
//             value: Direction::Right,
//             next_value: Direction::Right,
//         }
//     }
// }

#[derive(Component, Debug)]
pub struct SnakeHead;

#[derive(Component, Debug)]
pub struct SnakeSegment;

#[derive(Debug, Component, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Resource, Debug, Default)]
pub struct MovementTimer {
    timer: Timer,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::from_seconds(2., TimerMode::Repeating),
        })
        .add_systems(PostStartup, spawn_snake)
        .add_systems(Update, snake_movement_controls);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

fn spawn_snake(mut commands: Commands, board: Res<Board>) {
    let start_pos = snake_starting_position(board.size);

    // load snake head
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[0].x.into()),
                board.position_translate(start_pos[0].y.into()),
                10.0,
            ),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        SnakeHead,
        SnakeSegment,
        Position::from(start_pos[0]),
    ));

    // load snake tail
    start_pos[1..].iter().for_each(|segment| {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    board.position_translate(segment.x.into()),
                    board.position_translate(segment.y.into()),
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
            Position::new(segment.x, segment.y),
        ));
    });
}

fn snake_movement_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    board: Res<Board>,
    // mut snake: ResMut<SnakeBody>,
    // snake_direction_query: Query<&SnakeDirection, With<Snake>>,
    mut movement_timer: ResMut<MovementTimer>,
) {
    // for (mut transform, direction) in query.iter_mut() {
    //     println!("Snake position: {:?}", transform.translation);
    // }
    // let Ok(snake_direction) = snake_direction_query.get_single() else {
    //     return;
    // };
    // println!("Snake's head direction: {:?}", snake_direction);

    // if snake_direction.value != snake_direction.next_value {
    //     snake_direction.value = snake_direction.next_value;
    // }

    // if keyboard_input.pressed(KeyCode::ArrowUp) && snake_direction.value != Direction::Down {
    //     snake_direction.next_value = Direction::Up;
    // } else if keyboard_input.pressed(KeyCode::ArrowDown) && snake_direction.value != Direction::Up {
    //     snake_direction.next_value = Direction::Down;
    // } else if keyboard_input.pressed(KeyCode::ArrowLeft)
    //     && snake_direction.value != Direction::Right
    // {
    //     snake_direction.next_value = Direction::Left;
    // } else if keyboard_input.pressed(KeyCode::ArrowRight)
    //     && snake_direction.value != Direction::Left
    // {
    //     snake_direction.next_value = Direction::Right;
    // }

    // if !movement_timer.timer.tick(time.delta()).just_finished() {
    //     return;
    // }
    //
    // let head = snake.segments.first().clone().unwrap();
    // let new_head = match snake_direction.value {
    //     Direction::Up => Position {
    //         x: head.x,
    //         y: head.y + 1,
    //     },
    //     Direction::Down => Position {
    //         x: head.x,
    //         y: head.y - 1,
    //     },
    //     Direction::Left => Position {
    //         x: head.x - 1,
    //         y: head.y,
    //     },
    //     Direction::Right => Position {
    //         x: head.x + 1,
    //         y: head.y,
    //     },
    // };
    //
    // snake.segments.insert(0, new_head);
    // snake.segments.pop();
    // println!("Snake segments: {:?}", snake.segments);

    // @info: we have to create a new snake segnment and spawn it as the new head. then we have to
    // remove the last segment of the snake. This is how we simulate the snake moving.

    // move head
    // snake.segments.push(Position {});
    // match snake_direciton.

    // match snake_direction.value {
    //     Direction::Up => {
    //         // *snake = SnakeBody {
    //         //     segments: snake.segments.iter().map(|segment| {
    //         //         let mut new_segment = segment.clone();
    //         //         new_segment.y += 1;
    //         //         new_segment
    //         //     }),
    //         // };
    //         let mut new_head = snake.segments.first().unwrap().clone();
    //         new_head.y += 1;
    //         snake.segments.insert(0, new_head);
    //     }
    //     Direction::Down => {
    //         let mut head = snake.segments.first_mut().unwrap();
    //         head.y -= 1;
    //         let mut tail = snake.segments.iter_mut().skip(1);
    //         for segment in tail {
    //             segment.y -= 1;
    //         }
    //     }
    //     Direction::Left => {
    //         let mut head = snake.segments.first_mut().unwrap();
    //         head.x -= 1;
    //         let mut tail = snake.segments.iter_mut().skip(1);
    //         for segment in tail {
    //             segment.x -= 1;
    //         }
    //     }
    //     Direction::Right => {
    //         let mut head = snake.segments.first_mut().unwrap();
    //         head.x += 1;
    //         let mut tail = snake.segments.iter_mut().skip(1);
    //         for segment in tail {
    //             segment.x += 1;
    //         }
    //     }
    // }
    //
    // dbg!(movement_timer.timer.elapsed_secs());
    //
    // match snake_direction.value {
    //     Direction::Up => transform.translation.y += TILE_SIZE,
    //     Direction::Down => transform.translation.y -= TILE_SIZE,
    //     Direction::Left => transform.translation.x -= TILE_SIZE,
    //     Direction::Right => transform.translation.x += TILE_SIZE,
    // }
}
