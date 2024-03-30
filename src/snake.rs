use bevy::{prelude::*, utils::dbg};

use crate::{
    board::{Board, TILE_SIZE},
    util::snake_starting_position,
};

#[derive(Component, Debug)]
pub struct SnakeDirection {
    value: Direction,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection {
            value: Direction::Right,
        }
    }
}

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        .add_systems(PostStartup, spawn_snake)
        .add_systems(
            Update,
            (snake_movement_controls, snake_position_update).chain(),
        );
    }
}

fn spawn_snake(mut commands: Commands, board: Res<Board>) {
    let start_pos = snake_starting_position(board.size);
    println!("start pos: {:?}", start_pos);

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
        SnakeDirection::default(),
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

fn snake_position_update(
    board: Res<Board>,
    mut query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = Vec3::new(
            board.position_translate(pos.x.into()),
            board.position_translate(pos.y.into()),
            1.,
        )
    }
}

fn snake_movement_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut movement_timer: ResMut<MovementTimer>,
    mut snake_head_query: Query<(&mut SnakeDirection, &mut Position), With<SnakeHead>>,
    mut snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
) {
    let Ok((mut snake_direction, mut head_pos)) = snake_head_query.get_single_mut() else {
        return;
    };

    let new_direction = if keyboard_input.pressed(KeyCode::ArrowUp) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        Direction::Right
    } else {
        snake_direction.value
    };

    // store previous head position, before updating it;
    let mut prev_pos = head_pos.clone();

    // update head position based on direction
    snake_direction.value = new_direction;

    movement_timer.timer.tick(time.delta());
    if !movement_timer.timer.just_finished() {
        return;
    }

    match snake_direction.value {
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
    }

    for (mut segment_pos, _) in snake_body_query.iter_mut() {
        let temp = *segment_pos;
        *segment_pos = prev_pos;
        prev_pos = temp;
    }
}
