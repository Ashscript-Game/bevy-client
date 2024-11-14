use bevy_egui::egui::{self, Color32, RichText, Ui};

pub fn header(ui: &mut Ui, header: impl Into<String>) {
    ui.label(RichText::new(header).color(Color32::WHITE));
}

pub fn header_with_text(ui: &mut Ui, header: impl Into<String>, content: impl Into<String>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(header).color(Color32::WHITE));
        ui.label(content.into());
    });
}