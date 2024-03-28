use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};

const SNAKE_SPEED: f32 = 30.0;

#[derive(Component, Debug)]
pub struct Snake;

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
            value: Direction::Up,
            next_value: Direction::Up,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementTimer {
            timer: Timer::from_seconds(0.4, TimerMode::Repeating),
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
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                // @todo: randomize the spawn of the snake
                board.position_translate(0),
                board.position_translate(19),
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
        SnakeDirection::default(),
    ));
}

fn snake_movement_controls(
    mut query: Query<(&mut Transform, &mut SnakeDirection), With<Snake>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    board: Res<Board>,
    mut movement_timer: ResMut<MovementTimer>,
) {
    let Ok((mut transform, mut direction)) = query.get_single_mut() else {
        return;
    };
    dbg!(
        transform.translation.x,
        transform.translation.y,
        direction.value,
        direction.next_value
    );

    if direction.value != direction.next_value {
        direction.value = direction.next_value;
    }

    // @info: this is a decent strategy. But what if we used positoin instead of direction?
    if keyboard_input.pressed(KeyCode::ArrowUp) && direction.value != Direction::Down {
        direction.next_value = Direction::Up;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && direction.value != Direction::Up {
        direction.next_value = Direction::Down;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) && direction.value != Direction::Right {
        direction.next_value = Direction::Left;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) && direction.value != Direction::Left {
        direction.next_value = Direction::Right;
    }

    if !movement_timer.timer.tick(time.delta()).just_finished() {
        return;
    }
    dbg!(movement_timer.timer.elapsed_secs());

    match direction.value {
        Direction::Up => transform.translation.y += TILE_SIZE,
        Direction::Down => transform.translation.y -= TILE_SIZE,
        Direction::Left => transform.translation.x -= TILE_SIZE,
        Direction::Right => transform.translation.x += TILE_SIZE,
    }
}
