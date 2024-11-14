use bevy_egui::egui::{self, epaint, style};

use super::constants::{Theme, LATTE};

pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        override_text_color: Some(theme.text),
        hyperlink_color: theme.rosewater,
        faint_bg_color: theme.surface0,
        extreme_bg_color: theme.crust,
        code_bg_color: theme.mantle,
        warn_fg_color: theme.peach,
        error_fg_color: theme.maroon,
        window_fill: theme.base,
        panel_fill: theme.base,
        window_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.window_stroke
        },
        widgets: style::Widgets {
            noninteractive: make_widget_visual(old.widgets.noninteractive, &theme, theme.base),
            inactive: make_widget_visual(old.widgets.inactive, &theme, theme.surface0),
            hovered: make_widget_visual(old.widgets.hovered, &theme, theme.surface2),
            active: make_widget_visual(old.widgets.active, &theme, theme.surface1),
            open: make_widget_visual(old.widgets.open, &theme, theme.surface0),
        },
        selection: style::Selection {
            bg_fill: theme
                .blue
                .linear_multiply(if theme == LATTE { 0.4 } else { 0.2 }),
            stroke: egui::Stroke {
                color: theme.overlay1,
                ..old.selection.stroke
            },
        },
        window_shadow: epaint::Shadow {
            color: theme.base,
            ..old.window_shadow
        },
        popup_shadow: epaint::Shadow {
            color: theme.base,
            ..old.popup_shadow
        },
        ..old
    });
}


fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}