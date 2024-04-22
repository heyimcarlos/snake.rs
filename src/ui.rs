use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}

struct Images {
    play_icon: Handle<Image>,
    settings_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            play_icon: asset_server.load("play.png"),
            settings_icon: asset_server.load("settings.png"),
        }
    }
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_state::<MenuState>()
            .add_systems(Update, setup_ui);
    }
}

fn setup_ui(
    mut contexts: EguiContexts,
    menu_state: Res<State<MenuState>>,
    images: Local<Images>,
    mut is_initialized: Local<bool>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };

    egui::TopBottomPanel::top("main menu top").show(contexts.ctx_mut(), |ui| {
        ui.label(
            egui::RichText::new("Game By ")
                .color(egui::Color32::GRAY)
                .font(egui::FontId::monospace(24.0)),
        );

        //  NOTE: add a new bar to the menu
        // egui::menu::bar(ui, |ui| {
        //     egui::menu::menu_button(ui, "File", |ui| {
        //         if ui.button("Quit").clicked() {
        //             std::process::exit(0);
        //         }
        //     })
        // });
    });

    //  NOTE: Covers the reamining space, including the game view.
    // egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    //     ui.label("central panel");
    // });

    // if !*is_initialized {
    //     *is_initialized = true;
    // }

    let play_icon = contexts.add_image(images.play_icon.clone());
    let settings_icon = contexts.add_image(images.settings_icon.clone());

    println!("{}", window.height());
    egui::Window::new("Menu")
        .default_size(egui::vec2(300.0, 600.0))
        .default_pos(egui::pos2(
            (window.width() - 300.0) / 2.0,
            (window.height() - 300.0) / 2.0,
        ))
        .movable(false)
        .collapsible(false)
        .interactable(false)
        .resizable(false)
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 150),
            stroke: egui::Stroke::new(2.0, egui::Color32::WHITE),
            rounding: egui::Rounding::same(10.0),
            inner_margin: egui::Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 10.0,
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
                    println!("Playing game..");
                }
                if ui
                    .add(
                        egui::Button::image_and_text(
                            egui::widgets::Image::new(egui::load::SizedTexture::new(
                                settings_icon,
                                [25.0, 25.0],
                            )),
                            egui::RichText::new("Settings")
                                .color(egui::Color32::WHITE)
                                .font(egui::FontId::monospace(20.0)),
                        )
                        .min_size(egui::vec2(200., 0.))
                        .rounding(8.0)
                        .fill(egui::Color32::from_hex("#15c").unwrap()),
                    )
                    .clicked()
                {
                    println!("Settings...");
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
            // egui::menu::bar(ui, |ui| {
            //     if ui.button("▶ Play").clicked() {
            //         println!("Playing game..");
            //     }
            // });

            //  NOTE: button nesting
            // ui.menu_button("My menu", |ui| {
            //     ui.menu_button("My sub-menu", |ui| {
            //         if ui.button("Close the menu").clicked() {
            //             ui.close_menu();
            //         }
            //     });
            // });

            // if let Some(button_style) = ui.style_mut().text_styles.get_mut(&egui::TextStyle::Button)
            // {
            //     *button_style = egui::FontId::new(20.0, egui::FontFamily::Monospace);
            //     // *button_style = egui::style
            // }
            // Image
        });
}
