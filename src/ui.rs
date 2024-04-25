use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, epaint::Shadow, TopBottomPanel},
    EguiContexts, EguiPlugin,
};

use crate::{
    asset_loader::{ImageAssets, SpritePart},
    score::Score,
    state::{GameState, MenuState},
};

struct Images {
    play_icon: Handle<Image>,
    apple_icon: Handle<Image>,
    trophy_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            play_icon: asset_server.load("play.png"),
            apple_icon: asset_server.load("snake-graphics.png"),
            trophy_icon: asset_server.load("trophy.png"),
        }
    }
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, update_menu.run_if(in_state(MenuState::On)))
            .add_systems(Update, update_top_bar);
    }
}

fn update_top_bar(
    mut contexts: EguiContexts,
    score: Res<Score>,
    images: Local<Images>,
    texture_atlas: Res<Assets<TextureAtlasLayout>>,
    assets: Res<ImageAssets>,
) {
    let apple_icon = contexts.add_image(images.apple_icon.clone());
    TopBottomPanel::top("hi")
        .min_height(60.)
        .show_separator_line(true)
        .frame(egui::Frame {
            fill: egui::Color32::from_hex("#4a752c").unwrap(),
            inner_margin: egui::Margin {
                left: 15.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.horizontal(|ui| {
                        if let Some(atlas) = texture_atlas.get(&assets.sprite_sheet_layout) {
                            let apple_rect = atlas.textures[SpritePart::Apple as usize].clone();
                            let uv: egui::Rect = egui::Rect::from_min_max(
                                egui::pos2(
                                    apple_rect.min.x / atlas.size.x,
                                    apple_rect.min.y / atlas.size.y,
                                ),
                                egui::pos2(
                                    apple_rect.max.x / atlas.size.x,
                                    apple_rect.max.y / atlas.size.y,
                                ),
                            );

                            ui.add(
                                egui::Image::new(egui::load::SizedTexture::new(
                                    apple_icon,
                                    [35.0, 35.0],
                                ))
                                .uv(uv),
                            );
                        }
                        ui.label(
                            egui::RichText::new(format!("{}", score.value))
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(20.0)),
                        );
                    });
                },
            );
        });
}

fn update_menu(
    mut contexts: EguiContexts,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    images: Local<Images>,
    score: Res<Score>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    texture_atlas: Res<Assets<TextureAtlasLayout>>,
    assets: Res<ImageAssets>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let apple_icon = contexts.add_image(images.apple_icon.clone());
    let play_icon = contexts.add_image(images.play_icon.clone());
    let trophy_icon = contexts.add_image(images.trophy_icon.clone());

    egui::Window::new("button-group")
        .title_bar(false)
        .default_size(egui::vec2(300.0, 600.0))
        .default_pos(egui::pos2(
            (window.width() - 300.0) / 2.0,
            (window.height() - 400.0) / 2.0,
        ))
        .movable(false)
        .collapsible(false)
        .interactable(false)
        .resizable(false)
        .frame(egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            shadow: Shadow {
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
                spread: window.width(),
                ..Default::default()
            },
            ..Default::default()
        })
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                egui::Frame::group(ui.style_mut())
                    .rounding(egui::Rounding::same(10.0))
                    .fill(egui::Color32::from_hex("#4dc1f9").unwrap())
                    .stroke(egui::Stroke::NONE)
                    .show(ui, |ui| {
                        ui.set_width(300.0);
                        ui.set_height(200.0);

                        ui.horizontal_centered(|ui| {
                            ui.add_space(75.0);
                            ui.horizontal_centered(|ui| {
                                //  NOTE: apple
                                if let Some(atlas) = texture_atlas.get(&assets.sprite_sheet_layout)
                                {
                                    let apple_rect =
                                        atlas.textures[SpritePart::Apple as usize].clone();
                                    let uv: egui::Rect = egui::Rect::from_min_max(
                                        egui::pos2(
                                            apple_rect.min.x / atlas.size.x,
                                            apple_rect.min.y / atlas.size.y,
                                        ),
                                        egui::pos2(
                                            apple_rect.max.x / atlas.size.x,
                                            apple_rect.max.y / atlas.size.y,
                                        ),
                                    );

                                    ui.add(
                                        egui::Image::new(egui::load::SizedTexture::new(
                                            apple_icon,
                                            [40.0, 40.0],
                                        ))
                                        .uv(uv),
                                    );
                                }
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new(format!("{}", score.previous))
                                        .color(egui::Color32::WHITE)
                                        .font(egui::FontId::monospace(20.0)),
                                );
                            });

                            // Spacer between groups
                            ui.add_space(20.0);

                            ui.horizontal_centered(|ui| {
                                //  NOTE: trophy
                                ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                    trophy_icon,
                                    [40.0, 40.0],
                                )));
                                ui.label(
                                    egui::RichText::new(format!("{}", score.highest))
                                        .color(egui::Color32::WHITE)
                                        .font(egui::FontId::monospace(20.0)),
                                );
                            });
                        });
                    });

                //  NOTE: Add some space between the scores and buttons
                ui.add_space(10.0);

                egui::Frame::group(ui.style_mut())
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(300.0, 0.));
                        ui.spacing_mut().icon_spacing = 85.;
                        ui.style_mut().spacing.button_padding = egui::Vec2::new(15., 10.);

                        //  NOTE: Menu Buttons
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
                                .min_size(egui::vec2(300., 0.))
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
                                .min_size(egui::vec2(300., 0.))
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
        });
}
