use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    asset_loader::{ImageAssets, SpritePart},
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    state::{GameState, MenuState},
    util::{direction_from_vec2, snake_starting_position},
};

#[derive(Component, Debug)]
pub struct SnakeHeadDirection {
    current: Direction,
    directions: Vec<Direction>,
}

impl SnakeHeadDirection {
    pub fn queue_direction(&mut self, new_direction: Direction) {
        // NOTE: check that the new direction is not the opposite of the last direction, and that we don't have more than 2 directions queued.
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

#[derive(Resource, Debug, Default)]
pub struct SnakeDirectionQueue {
    pub directions: VecDeque<Direction>,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeDirectionQueue>()
            .insert_resource(MovementTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            })
            .add_systems(OnEnter(GameState::NewGame), spawn_snake)
            .add_systems(OnEnter(GameState::NewGame), load_snake_direction_queue)
            .add_systems(
                Update,
                (movement_controls, touch_movement_controls).in_set(InGameSet::UserInput),
            )
            .add_systems(
                Update,
                (update_position, update_board_position, update_snake_sprite)
                    .chain()
                    .in_set(InGameSet::PositionUpdates),
            );
    }
}

fn load_snake_direction_queue(mut snake_direction_queue: ResMut<SnakeDirectionQueue>) {
    *snake_direction_queue = SnakeDirectionQueue {
        directions: VecDeque::from([Direction::Right; 3]),
    };
}

fn spawn_snake(mut commands: Commands, board: Res<Board>, assets: Res<ImageAssets>) {
    let start_pos = snake_starting_position(board.size);

    // NOTE: load snake head
    commands.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: SpritePart::HeadRight as usize,
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
                index: SpritePart::BodyHorizontal as usize,
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
                index: SpritePart::TailRight as usize,
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

#[derive(Default)]
pub struct TouchMap(pub(crate) Option<u64>);

fn touch_movement_controls(
    mut snake_head_query: Query<&mut SnakeHeadDirection, With<SnakeHead>>,
    touches: Res<Touches>,
    mut touch_map: Local<TouchMap>,
    // mut next_state: ResMut<NextState<GameState>>,
    // game_state: Res<State<GameState>>,
    // menu_state: Res<State<MenuState>>,
) {
    let Ok(mut snake_direction) = snake_head_query.get_single_mut() else {
        return;
    };

    //  NOTE: Check if the touchmap should be cleared.
    if let Some(tm) = touch_map.0 {
        for released in touches.iter_just_released() {
            if released.id() == tm {
                touch_map.0 = None;
                break;
            }
        }
        for released in touches.iter_just_canceled() {
            if released.id() == tm {
                touch_map.0 = None;
                break;
            }
        }

        //  NOTE: if no touches on screen, reset touch map.
        if touches.iter().count() == 0 {
            touch_map.0 = None;
        }
    }

    for touch in touches.iter() {
        match *touch_map {
            TouchMap(None) => {
                touch_map.0 = Some(touch.id());
            }
            TouchMap(Some(tm)) => {
                if tm == touch.id() {
                    let direction = touch.start_position() - touch.position();
                    //  NOTE: Determine if the movement is horizontal or vertical by checking if
                    //  the abs value of x is greater than the abs value of y, if it is, the
                    //  direction is horizontal.
                    let input = direction_from_vec2(direction);
                    for dir in input.iter() {
                        snake_direction.queue_direction(*dir);
                    }
                }
            }
        }
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
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
            KeyCode::ArrowUp
            | KeyCode::ArrowDown
            | KeyCode::ArrowLeft
            | KeyCode::ArrowRight
            | KeyCode::KeyW
            | KeyCode::KeyA
            | KeyCode::KeyS
            | KeyCode::KeyD => Some(*key),
            _ => None,
        })
        .collect();

    //  NOTE: Iterate through the collected keys and queue valid directions
    for key in keys_pressed {
        let direction = match key {
            KeyCode::ArrowUp | KeyCode::KeyW => Direction::Up,
            KeyCode::ArrowDown | KeyCode::KeyS => Direction::Down,
            KeyCode::ArrowLeft | KeyCode::KeyA => Direction::Left,
            KeyCode::ArrowRight | KeyCode::KeyD => Direction::Right,
            _ => continue,
        };
        //  NOTE: new head direction to be queued
        snake_direction.queue_direction(direction);
    }
}

fn update_position(
    mut movement_timer: ResMut<MovementTimer>,
    time: Res<Time>,
    mut snake_head_query: Query<(&mut SnakeHeadDirection, &mut Position), With<SnakeHead>>,
    mut snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
    mut snake_direction_queue: ResMut<SnakeDirectionQueue>,
) {
    movement_timer.timer.tick(time.delta());
    if !movement_timer.timer.just_finished() {
        return;
    }

    let Ok((mut snake_head_direction_input, mut head_pos)) = snake_head_query.get_single_mut()
    else {
        return;
    };

    //  NOTE: check if there's a queued direction and update the current direction also dequeue the first direction
    if let Some(new_direction) = snake_head_direction_input.directions.get(0) {
        snake_head_direction_input.current = *new_direction;
        snake_head_direction_input.directions.remove(0);
    }

    snake_direction_queue
        .directions
        .push_front(snake_head_direction_input.current);
    snake_direction_queue.directions.pop_back();

    let mut prev_pos = head_pos.clone();

    match snake_head_direction_input.current {
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
    mut snake_query: Query<(&Position, &mut TextureAtlas, Entity), With<SnakeSegment>>,
    direction_queue: Res<SnakeDirectionQueue>,
) {
    //  NOTE: zip an immutable iter from directions and a mutable for the snake_query which contains the sprite.
    for (i, (direction, (_, mut sprite, _))) in direction_queue
        .directions
        .iter()
        .zip(snake_query.iter_mut())
        .enumerate()
    {
        sprite.index = if i == 0 {
            match direction {
                Direction::Up => SpritePart::HeadUp as usize,
                Direction::Down => SpritePart::HeadDown as usize,
                Direction::Left => SpritePart::HeadLeft as usize,
                Direction::Right => SpritePart::HeadRight as usize,
            }
        } else if i == direction_queue.directions.len() - 1 {
            //  NOTE: use the segment of the snake that's previous to the snake tail to decide the tails direction
            let prev_direction = direction_queue.directions[i - 1];
            match prev_direction {
                Direction::Up => SpritePart::TailUp as usize,
                Direction::Down => SpritePart::TailDown as usize,
                Direction::Left => SpritePart::TailLeft as usize,
                Direction::Right => SpritePart::TailRight as usize,
            }
        } else {
            let prev_direction = direction_queue.directions[i - 1];
            match (direction, prev_direction) {
                (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                    SpritePart::BodyVertical as usize
                }
                (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
                    SpritePart::BodyHorizontal as usize
                }
                (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                    SpritePart::BodyTopRight as usize
                }
                (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                    SpritePart::BodyTopLeft as usize
                }
                (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                    SpritePart::BodyBottomRight as usize
                }
                (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                    SpritePart::BodyBottomLeft as usize
                }
                _ => {
                    println!("No match");
                    panic!("No match")
                }
            }
        }
    }
}
