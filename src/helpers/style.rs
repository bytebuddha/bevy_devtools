use bevy_inspector_egui::bevy_egui::egui;

pub struct DraculaEgui;

impl DraculaEgui {
    pub const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(40, 42, 54);

    pub const CURRENT_LINE: egui::Color32 = egui::Color32::from_rgb(68, 71, 90);

    pub const FOREGROUND: egui::Color32 = egui::Color32::from_rgb(248, 248, 242);

    pub const COMMENT: egui::Color32 = egui::Color32::from_rgb(98, 114, 164);

    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(139, 233, 253);

    pub const GREEN: egui::Color32 = egui::Color32::from_rgb(80, 250, 123);

    pub const ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 184, 108);

    pub const PINK: egui::Color32 = egui::Color32::from_rgb(255, 123, 198);

    pub const PURPLE: egui::Color32 = egui::Color32::from_rgb(189, 147, 249);

    pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 85, 85);

    pub const YELLOW: egui::Color32 = egui::Color32::from_rgb(241, 250, 140);

    pub fn style() -> egui::Style {
        egui::Style {
            visuals: egui::Visuals {
                widgets: egui::style::Widgets {
                    noninteractive: egui::style::WidgetVisuals {
                        bg_fill: Self::BACKGROUND,
                        bg_stroke: egui::Stroke::new(1.0, Self::CURRENT_LINE),
                        fg_stroke: egui::Stroke::new(1.0, Self::FOREGROUND),
                        corner_radius: 4.0,
                        expansion: 0.0,
                    },
                    inactive: egui::style::WidgetVisuals {
                        bg_fill: Self::CURRENT_LINE,
                        bg_stroke: Default::default(),
                        fg_stroke: egui::Stroke::new(1.0, Self::FOREGROUND),
                        corner_radius: 4.0,
                        expansion: 0.0,
                    },
                    hovered: egui::style::WidgetVisuals {
                        bg_fill: Self::PURPLE,
                        bg_stroke: egui::Stroke::new(1.0, Self::FOREGROUND),
                        fg_stroke: egui::Stroke::new(1.5, Self::FOREGROUND),
                        corner_radius: 4.0,
                        expansion: 1.0,
                    },
                    active: egui::style::WidgetVisuals {
                        bg_fill: Self::PINK,
                        bg_stroke: egui::Stroke::new(1.0, Self::FOREGROUND),
                        fg_stroke: egui::Stroke::new(2.0, Self::FOREGROUND),
                        corner_radius: 0.0,
                        expansion: 2.0,
                    },
                    ..Default::default()
                },
                selection: egui::style::Selection {
                    bg_fill: Self::PURPLE,
                    stroke: egui::Stroke::new(1.5, egui::Color32::WHITE),
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
