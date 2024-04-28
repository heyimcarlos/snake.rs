use bevy::{prelude::*, window::WindowResized};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, resize_camera);
    }
}

enum Breakpoints {
    XS = 380,
    SM = 455,
    MD = 660,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20., 0.0),
        ..Default::default()
    });
}

fn resize_camera(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for ev in resize_events.read() {
        for mut transform in query.iter_mut() {
            let baseline = if ev.width < Breakpoints::XS as i32 as f32 {
                200.0
            } else if ev.width < Breakpoints::SM as i32 as f32 {
                225.0
            } else if ev.width < Breakpoints::MD as i32 as f32 {
                450.0
            } else {
                720.0
            };

            println!(
                "dimension: {:?} / baseline: {:?}",
                ev.width.min(ev.height),
                baseline
            );

            let scale = ev.width.min(ev.height) / baseline;
            println!("scale {:?}", scale);
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}
