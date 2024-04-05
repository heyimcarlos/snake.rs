use bevy::prelude::*;

use crate::{
    asset_loader::ImageAssets,
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    state::GameState,
    util::{get_sprite_index, snake_starting_position},
};

#[derive(Component, Debug)]
pub struct SnakeDirection {
    current: Direction,
    directions: Vec<Direction>,
}

impl SnakeDirection {
    pub fn queue_direction(&mut self, new_direction: Direction) {
        // @info: check that the new direction is not the opposite of the last direction, and that we don't have more than 2 directions queued
        if let Some(&last_direction) = self.directions.last() {
            if new_direction != last_direction.opposite() && self.directions.len() < 3 {
                self.directions.push(new_direction);
            }
        } else if self.current != new_direction.opposite() {
            self.directions.push(new_direction);
        }
    }
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection {
            current: Direction::default(),
            directions: vec![Direction::Right],
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
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BeforeGame), spawn_snake)
            .insert_resource(MovementTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            })
            .add_systems(Update, movement_controls.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (update_position, update_board_position)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

fn spawn_snake(mut commands: Commands, board: Res<Board>, assets: Res<ImageAssets>) {
    let start_pos = snake_starting_position(board.size);
    let head_texture = TextureAtlas {
        layout: assets.sprite_sheet_layout.clone(),
        index: get_sprite_index(0, 4, 5),
    };

    // load snake head
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[0].x.into()),
                board.position_translate(start_pos[0].y.into()),
                2.0,
            ),
            texture: assets.sprite_sheet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        },
        TextureAtlas::from(head_texture),
        SnakeHead,
        SnakeSegment,
        Position::from(start_pos[0]),
        SnakeDirection::default(),
    ));

    // TextureAtlas::from(texture_atlas).index,
    // load snake tail
    start_pos[1..].iter().for_each(|segment| {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    board.position_translate(segment.x.into()),
                    board.position_translate(segment.y.into()),
                    10.0,
                ),
                texture: assets.sprite_sheet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            TextureAtlas::from(TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: get_sprite_index(0, 1, 5),
            }),
            SnakeSegment,
            Position::new(segment.x, segment.y),
        ));
    });
}

fn update_board_position(
    board: Res<Board>,
    mut query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = Vec3::new(
            board.position_translate(pos.x),
            board.position_translate(pos.y),
            1.0,
        );
    }
}

fn movement_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_head_query: Query<&mut SnakeDirection, With<SnakeHead>>,
) {
    let Ok(mut snake_direction) = snake_head_query.get_single_mut() else {
        return;
    };

    let keys_pressed: Vec<KeyCode> = keyboard_input
        .get_just_pressed()
        .filter_map(|key| match key {
            KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                Some(*key)
            }
            _ => None,
        })
        .collect();

    // Iterate through the collected keys and queue valid directions
    for key in keys_pressed {
        println!("key pressed: {:?}", key);
        let direction = match key {
            KeyCode::ArrowUp => Direction::Up,
            KeyCode::ArrowDown => Direction::Down,
            KeyCode::ArrowLeft => Direction::Left,
            KeyCode::ArrowRight => Direction::Right,
            _ => continue,
        };
        snake_direction.queue_direction(direction);
    }
}

fn update_position(
    mut movement_timer: ResMut<MovementTimer>,
    time: Res<Time>,
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

    // @info: check if there's a queued direction and update the current direction
    // also dequeue the direction
    if let Some(new_direction) = snake_direction.directions.get(0) {
        snake_direction.current = *new_direction;
        snake_direction.directions.remove(0);
    }

    let mut prev_pos = head_pos.clone();

    match snake_direction.current {
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
    };

    for (mut segment_pos, _) in snake_body_query.iter_mut() {
        let temp = *segment_pos;
        *segment_pos = prev_pos;
        prev_pos = temp;
    }
}
