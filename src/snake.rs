use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    state::GameState,
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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
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
        app.add_systems(OnEnter(GameState::BeforeGame), spawn_snake)
            .insert_resource(MovementTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            })
            .add_systems(Update, snake_movement_controls.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                snake_position_update.in_set(InGameSet::EntityUpdates),
            );
    }
}

fn spawn_snake(mut commands: Commands, board: Res<Board>, assets: Res<SceneAssets>) {
    let start_pos = snake_starting_position(board.size);

    // load snake head
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[0].x.into()),
                board.position_translate(start_pos[0].y.into()),
                2.0,
            ),
            texture: assets.snake_head.clone(),
            sprite: Sprite {
                // color: Color::BLUE,
                custom_size: Some(Vec2::new(TILE_SIZE - 3., TILE_SIZE - 3.)),
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
                texture: assets.snake.clone(),
                sprite: Sprite {
                    // color: Color::BLUE,
                    custom_size: Some(Vec2::new(TILE_SIZE - 3., TILE_SIZE - 3.)),
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
    movement_timer.timer.tick(time.delta());
    if !movement_timer.timer.just_finished() {
        return;
    }

    let Ok((mut snake_direction, mut head_pos)) = snake_head_query.get_single_mut() else {
        return;
    };

    let new_direction = if keyboard_input.pressed(KeyCode::ArrowUp)
        && snake_direction.value != Direction::Down
    {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && snake_direction.value != Direction::Up {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::ArrowLeft)
        && snake_direction.value != Direction::Right
    {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::ArrowRight)
        && snake_direction.value != Direction::Left
    {
        Direction::Right
    } else {
        snake_direction.value
    };

    // @todo: right now if the user quickly presses a new direction and immediatedly presses the
    // direction in which the snake is moving, the direction that's pressed first will be ignored.

    // update head position based on direction
    snake_direction.value = new_direction;

    // store previous head position, before updating it;
    let mut prev_pos = head_pos.clone();

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
