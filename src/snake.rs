use bevy::{prelude::*, utils::tracing::Instrument};

use crate::board::{Board, TILE_SIZE};

const SNAKE_SPEED: f32 = 30.0;

#[derive(Resource, Debug)]
pub struct SnakeBody {
    segments: Vec<Position>,
}

impl Default for SnakeBody {
    fn default() -> Self {
        Self {
            segments: Vec::from([
                Position { x: 3, y: 1 },
                Position { x: 2, y: 1 },
                Position { x: 1, y: 1 },
            ]),
        }
    }
}

#[derive(Component, Debug)]
pub struct Snake;

#[derive(Debug, Component)]
pub struct Position {
    x: u8,
    y: u8,
}

#[derive(Component, Debug)]
pub struct SnakeDirection {
    pub value: Direction,
    pub next_value: Direction,
}

#[derive(Resource, Debug, Default)]
pub struct MovementTimer {
    timer: Timer,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection {
            value: Direction::Right,
            next_value: Direction::Right,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::from_seconds(2., TimerMode::Repeating),
        })
        .init_resource::<SnakeBody>()
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

fn spawn_snake(mut commands: Commands, board: Res<Board>, snake: Res<SnakeBody>) {
    // get snake head
    let head = snake.segments.first().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                // @todo: randomize the spawn of the snake
                board.position_translate(head.x.into()),
                board.position_translate(head.y.into()),
                10.0,
            ),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        Snake,
        SnakeDirection::default(),
    ));

    let tail = &snake.segments[1..];
    for segment in tail {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    // @todo: randomize the spawn of the snake
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
            Snake,
        ));
    }
}

fn snake_movement_controls(
    mut query: Query<(&mut Transform, &mut SnakeDirection), With<Snake>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    board: Res<Board>,
    mut snake: ResMut<SnakeBody>,
    mut movement_timer: ResMut<MovementTimer>,
) {
    // for (mut transform, direction) in query.iter_mut() {
    //     println!("Snake position: {:?}", transform.translation);
    // }
    let Ok((mut transform, mut snake_direction)) = query.get_single_mut() else {
        return;
    };
    println!("Snake's head direction: {:?}", snake_direction);
    // dbg!(
    //     transform.translation.x,
    //     transform.translation.y,
    //     direction.value,
    //     direction.next_value
    // );
    //
    if snake_direction.value != snake_direction.next_value {
        snake_direction.value = snake_direction.next_value;
    }
    //
    // // @info: this is a decent strategy. But what if we used positoin instead of direction?
    if keyboard_input.pressed(KeyCode::ArrowUp) && snake_direction.value != Direction::Down {
        snake_direction.next_value = Direction::Up;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && snake_direction.value != Direction::Up {
        snake_direction.next_value = Direction::Down;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft)
        && snake_direction.value != Direction::Right
    {
        snake_direction.next_value = Direction::Left;
    } else if keyboard_input.pressed(KeyCode::ArrowRight)
        && snake_direction.value != Direction::Left
    {
        snake_direction.next_value = Direction::Right;
    }
    //
    if !movement_timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    // @info: we have to create a new snake segnment and spawn it as the new head. then we have to
    // remove the last segment of the snake. This is how we simulate the snake moving.

    // move head
    let new_segment = snake.segments.first().clone();

    // match snake_direction.value {
    //     Direction::Up => {
    //         let mut head = snake.segments.first_mut().unwrap();
    //         head.y += 1;
    //         let mut tail = snake.segments.iter_mut().skip(1);
    //         for segment in tail {
    //             segment.y += 1;
    //         }
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

    // dbg!(movement_timer.timer.elapsed_secs());
    //
    // match snake_direction.value {
    //     Direction::Up => transform.translation.y += TILE_SIZE,
    //     Direction::Down => transform.translation.y -= TILE_SIZE,
    //     Direction::Left => transform.translation.x -= TILE_SIZE,
    //     Direction::Right => transform.translation.x += TILE_SIZE,
    // }
}
