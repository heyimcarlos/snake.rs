use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, epaint::Shadow, TopBottomPanel},
    EguiContexts, EguiPlugin,
};

use crate::state::{GameState, MenuState};

struct Images {
    play_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            play_icon: asset_server.load("play.png"),
        }
    }
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, update_menu.run_if(in_state(MenuState::On)))
            .add_systems(Update, top_menu);
    }
}

fn top_menu(
    mut contexts: EguiContexts,
    // mut next_menu_state: ResMut<NextState<MenuState>>,
    // game_state: Res<State<GameState>>,
    // mut next_game_state: ResMut<NextState<GameState>>,
    // images: Local<Images>,
    // window: Query<&mut Window, With<PrimaryWindow>>,
) {
    TopBottomPanel::top("hi")
        .min_height(50.)
        .frame(egui::Frame {
            fill: egui::Color32::from_hex("#4a752c").unwrap(),
            // inner_margin: egui::Margin {
            //     left: 10.0,
            //     right: 10.0,
            //     top: 10.0,
            //     bottom: 10.0,
            // },
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Score");
        });
}

fn update_menu(
    mut contexts: EguiContexts,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    images: Local<Images>,
    window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let play_icon = contexts.add_image(images.play_icon.clone());

    egui::Window::new("button-group")
        .title_bar(false)
        .default_size(egui::vec2(300.0, 600.0))
        .default_pos(egui::pos2(
            (window.width() - 300.0) / 2.0,
            window.height() / 2.0,
        ))
        .movable(false)
        .collapsible(false)
        .interactable(false)
        .resizable(false)
        .frame(egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            // fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
            // stroke: egui::Stroke::new(2.0, egui::Color32::WHITE),
            // rounding: egui::Rounding::same(10.0),
            // inner_margin: egui::Margin {
            //     left: 10.0,
            //     right: 10.0,
            //     top: 10.0,
            //     bottom: 10.0,
            // },
            shadow: Shadow {
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
                spread: window.width(),
                ..Default::default()
            },
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            ui.spacing_mut().icon_spacing = 50.;
            ui.style_mut().spacing.button_padding = egui::Vec2::new(15., 10.);

            //  NOTE: Menu Buttons
            ui.vertical_centered(|ui| {
                if ui
                    .add(
                        egui::Button::image_and_text(
                            egui::widgets::Image::new(egui::load::SizedTexture::new(
                                play_icon,
                                [25.0, 25.0],
                            )),
                            egui::RichText::new("Play")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(20.0)),
                        )
                        .min_size(egui::vec2(200., 0.))
                        .rounding(8.0)
                        .fill(egui::Color32::from_hex("#15c").unwrap()),
                    )
                    .clicked()
                {
                    match game_state.get() {
                        GameState::Paused => next_game_state.set(GameState::Playing),
                        _ => (),
                    }
                    next_menu_state.set(MenuState::Off);
                }
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Exit")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(20.0)),
                        )
                        .min_size(egui::vec2(200., 0.))
                        .rounding(8.0)
                        .fill(egui::Color32::from_hex("#15c").unwrap()),
                    )
                    .clicked()
                {
                    println!("exiting...");
                    std::process::exit(0);
                }
            });
        });
}
