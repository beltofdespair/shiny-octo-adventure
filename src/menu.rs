use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, Color32, FontData, FontDefinitions, FontFamily, FontId,
        TextStyle::{Body, Button, Heading},
    },
    EguiContexts,
};

const UI_HEADING: &str = "Shiny Octo Adventure";
const UI_ADD_SPACE: f32 = 100.0;
const INNER_MARGIN: f32 = 120.0;
const HEADING_FONT_FAMILY: &str = "Kings-Regular";
const HEADING_FONT_SIZE: f32 = 50.0;
const BODY_FONT_FAMILY: &str = "Kings-Regular";
const BODY_FONT_SIZE: f32 = 40.0;
const BUTTON_FONT_FAMILY: &str = "Kings-Regular";
const BUTTON_FONT_SIZE: f32 = 40.0;
const STROKE_COLOR: Color32 = egui::Color32::from_gray(50);

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State `GameState::Menu` and is removed
/// when that state is exited.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_custom_fonts);
    app.add_systems(Update, setup_menu.run_if(in_state(GameState::Menu)));
}

fn setup_menu(
    mut egui_contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    get_menu_panel().show(egui_contexts.ctx_mut(), |ui| {
        set_menu_style(ui.style_mut());
        ui.vertical_centered_justified(|ui| {
            ui.add_space(UI_ADD_SPACE);
            ui.heading(UI_HEADING);
            ui.separator();
            ui.add_space(UI_ADD_SPACE);
            if ui.button("Play").clicked() {
                next_state.set(GameState::Playing);
            }
        })
    });
}

fn get_menu_panel() -> egui::CentralPanel {
    egui::CentralPanel::default().frame(egui::Frame {
        inner_margin: egui::style::Margin::same(INNER_MARGIN),
        ..default()
    })
}

fn set_menu_style(style: &mut egui::Style) {
    style.text_styles = [
        (
            Heading,
            FontId::new(
                HEADING_FONT_SIZE,
                egui::FontFamily::Name(HEADING_FONT_FAMILY.into()),
            ),
        ),
        (
            Body,
            FontId::new(
                BODY_FONT_SIZE,
                egui::FontFamily::Name(BODY_FONT_FAMILY.into()),
            ),
        ),
        (
            Button,
            FontId::new(
                BUTTON_FONT_SIZE,
                egui::FontFamily::Name(BUTTON_FONT_FAMILY.into()),
            ),
        ),
    ]
    .into();
    style.visuals.widgets.noninteractive.fg_stroke.color = STROKE_COLOR;
    // egui::Color32::from_gray(250);
}

fn setup_custom_fonts(mut egui_contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        HEADING_FONT_FAMILY.to_owned(),
        FontData::from_static(include_bytes!(
            "../assets/fonts/Kings-Regular.ttf" /* "../assets/fonts/
                                                 * DejaVuSerif.ttf"
                                                 * "../assets/fonts/
                                                 * DemonicLetters.ttf" */
        )),
    );
    fonts
        .families
        .entry(FontFamily::Name(HEADING_FONT_FAMILY.into()))
        .or_default()
        .push(HEADING_FONT_FAMILY.into());
    // fonts
    //     .families
    //     .entry(FontFamily::Proportional)
    //     .or_default()
    //     .insert(0, HEADING_FONT_FAMILY.to_owned());
    // fonts
    //     .families
    //     .entry(FontFamily::Name(HEADING_FONT_FAMILY.into()))
    //     .or_default()
    //     .push(FontFamily::Proportional.to_string());
    egui_contexts.ctx_mut().set_fonts(fonts);
}
