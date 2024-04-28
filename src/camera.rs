use bevy::{prelude::*, window::WindowResized};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, resize_camera);
    }
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
            if ev.width < 500.0 {
                let baseline = 225.0;
                let scale = ev.width.min(ev.height) / baseline;
                transform.scale = Vec3::new(scale, scale, 1.0);
            }
        }
    }
}
