use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, setup_ui);
    }
}

fn setup_ui(mut contexts: EguiContexts) {
    egui::Window::new("Hello dude!").show(contexts.ctx_mut(), |ui| {
        ui.label("World!");
    });
}
