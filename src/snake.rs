use std::collections::VecDeque;

use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    asset_loader::{ImageAssets, SpritePart},
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    state::GameState,
    util::{detect_direction, snake_starting_position},
};

#[derive(Component, Debug)]
pub struct SnakeHeadDirection {
    current: Direction,
    directions: Vec<Direction>,
}

impl SnakeHeadDirection {
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

impl Default for SnakeHeadDirection {
    fn default() -> Self {
        SnakeHeadDirection {
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

#[derive(Resource, Debug)]
pub struct SnakeDirectionQueue {
    directions: VecDeque<Direction>,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BeforeGame), spawn_snake)
            .insert_resource(SnakeDirectionQueue {
                // @info: the snake directions queue starts with all 3 initial segments of the
                // snake moving right.
                directions: VecDeque::from([Direction::Right; 3]),
            })
            .insert_resource(MovementTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            })
            .add_systems(Update, movement_controls.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (
                    update_position.after(movement_controls),
                    update_board_position.after(update_position),
                    update_snake_sprite.after(update_position),
                )
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

fn spawn_snake(mut commands: Commands, board: Res<Board>, assets: Res<ImageAssets>) {
    let start_pos = snake_starting_position(board.size);

    // load snake head
    commands.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: assets.get_sprite_index(SpritePart::HeadRight),
            },
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
        SnakeHead,
        SnakeSegment,
        Position::from(start_pos[0]),
        SnakeHeadDirection::default(),
    ));

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[1].x),
                board.position_translate(start_pos[1].y),
                10.0,
            ),
            texture: assets.sprite_sheet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: assets.get_sprite_index(SpritePart::BodyHorizontal),
            },
            ..default()
        },
        SnakeSegment,
        Position::new(start_pos[1].x, start_pos[1].y),
    ));

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[2].x),
                board.position_translate(start_pos[2].y),
                10.0,
            ),
            texture: assets.sprite_sheet.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: assets.get_sprite_index(SpritePart::TailRight),
            },
            ..default()
        },
        SnakeSegment,
        Position::new(start_pos[2].x, start_pos[2].y),
    ));
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
    mut snake_head_query: Query<&mut SnakeHeadDirection, With<SnakeHead>>,
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
        let direction = match key {
            KeyCode::ArrowUp => Direction::Up,
            KeyCode::ArrowDown => Direction::Down,
            KeyCode::ArrowLeft => Direction::Left,
            KeyCode::ArrowRight => Direction::Right,
            _ => continue,
        };
        // @info: new head direction to be queued
        snake_direction.queue_direction(direction);
    }
}

// @todo: use a vecdeque to keep the snakes positions.
// @todo: use a vecdequeue for directions
fn update_position(
    mut movement_timer: ResMut<MovementTimer>,
    time: Res<Time>,
    mut snake_head_query: Query<(&mut SnakeHeadDirection, &mut Position), With<SnakeHead>>,
    mut snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
    mut direction_queue: ResMut<SnakeDirectionQueue>,
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

    // @info: new head direction is already in current, so push it to the direction queue.
    direction_queue
        .directions
        .push_front(snake_direction.current);
    direction_queue.directions.pop_back();

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

fn update_snake_sprite(
    mut snake_query: Query<
        (&Position, &mut TextureAtlas, Entity),
        (With<SnakeSegment>, Without<SnakeHead>),
    >,
    mut snake_head_query: Query<(&SnakeHeadDirection, &mut TextureAtlas), With<SnakeHead>>,
    assets: Res<ImageAssets>,
) {
    // head
    let Ok((snake_direction, mut sprite)) = snake_head_query.get_single_mut() else {
        return;
    };
    sprite.index = match snake_direction.current {
        Direction::Up => assets.get_sprite_index(SpritePart::HeadUp),
        Direction::Down => assets.get_sprite_index(SpritePart::HeadDown),
        Direction::Left => assets.get_sprite_index(SpritePart::HeadLeft),
        Direction::Right => assets.get_sprite_index(SpritePart::HeadRight),
    };

    // body
    let mut updates: Vec<(Entity, usize)> = vec![];

    for (front, mid, back) in snake_query.iter().tuple_windows() {
        let a = detect_direction(mid.0, front.0);
        let b = detect_direction(mid.0, back.0);

        let sprite_index = match (a, b) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => {
                assets.get_sprite_index(SpritePart::BodyVertical)
            }
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                assets.get_sprite_index(SpritePart::BodyHorizontal)
            }
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                assets.get_sprite_index(SpritePart::BodyTopRight)
            }
            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                assets.get_sprite_index(SpritePart::BodyTopLeft)
            }
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                assets.get_sprite_index(SpritePart::BodyBottomRight)
            }
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                assets.get_sprite_index(SpritePart::BodyBottomLeft)
            }
            _ => {
                continue;
            }
        };
        updates.push((mid.2, sprite_index));
    }

    for (entity, sprite_index) in updates {
        if let Ok((_, mut sprite, _)) = snake_query.get_mut(entity) {
            sprite.index = sprite_index;
        }
    }

    // tail
    if let Some((_, mut tail_sprite, _)) = snake_query.iter_mut().last() {
        tail_sprite.index = match snake_direction.current {
            Direction::Up => assets.get_sprite_index(SpritePart::TailUp),
            Direction::Down => assets.get_sprite_index(SpritePart::TailDown),
            Direction::Left => assets.get_sprite_index(SpritePart::TailLeft),
            Direction::Right => assets.get_sprite_index(SpritePart::TailRight),
        };
    }
}
