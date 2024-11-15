use bevy_egui::egui::{self, Color32, RichText, Ui};

use super::constants::text_size;

pub fn custom_header(ui: &mut Ui, header: impl Into<String>, size: f32) {
    ui.label(RichText::new(header).color(Color32::WHITE).size(size));
}

pub fn header(ui: &mut Ui, header: impl Into<String>) {
    ui.label(RichText::new(header).color(Color32::WHITE).size(text_size::SMALL));
}

pub fn header_with_text(ui: &mut Ui, header: impl Into<String>, content: impl Into<String>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(header).color(Color32::WHITE).size(text_size::SMALL));
        ui.label(RichText::new(content.into()).size(text_size::SMALL));
    });
}