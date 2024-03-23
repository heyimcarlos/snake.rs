use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Default, Copy, Debug)]
enum Cell {
    #[default]
    Empty,
    SnakeSegment,
    Food,
}

// @info: the map is a good resource, because it's a singleton and is not dependent on any entity.
#[derive(Resource, Debug, Default)]
struct Map {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![Cell::Empty; width]; height],
            width,
            height,
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(32, 18))
            .add_systems(Startup, load_map);
    }
}
//  [
//      y1 [x1, x2]
//      y2 [x1, x2]
// ]
//
const CELL_WIDTH: f32 = 40.;
const CELL_HEIGHT: f32 = 40.;

// default window size is 1280 x 720
// x expands from (-640, 640)
// y expands from (-360, 360)
// top-left most cell should be (-640, 360) if every cell took one pixel

fn load_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
    asset_server: Res<AssetServer>,
) {
    // println!("MAP LOADING; width: {}; height: {}", map.width, map.height);
    let Ok(window) = window_query.get_single() else {
        return;
    };

    // let Ok(camera) = camera_query.get_single() else {
    //     return;
    // };
    // println!("{}", camera.world_to_viewport());
    println!("WINDOW: W:{}, H:{}", window.width(), window.height());
    for row in 0..map.height {
        for column in 0..map.width {
            let start_x = -window.width() / 2.;
            let start_y = -window.height() / 2.;
            let x = start_x + column as f32 * CELL_WIDTH + CELL_WIDTH / 2.;
            let y = start_y + row as f32 * CELL_HEIGHT + CELL_HEIGHT / 2.;
            match map.grid[row][column] {
                Cell::Empty => {
                    // println!("LOADING CELL");
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.),
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::new(1., 1.)),
                            ..default()
                        },
                        ..default()
                    });

                    commands.spawn(SceneBundle {
                        scene: asset_server.load("FloorTile.glb#Scene0"),
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                    });
                }
                Cell::Food => {
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::ZERO),
                        sprite: Sprite {
                            color: Color::BLUE,
                            custom_size: Some(Vec2::new(1., 1.)),
                            ..default()
                        },
                        ..default()
                    });
                }
                Cell::SnakeSegment => {
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::ZERO),
                        sprite: Sprite {
                            color: Color::ORANGE_RED,
                            ..default()
                        },
                        ..default()
                    });
                }
            };
        }
    }
}
