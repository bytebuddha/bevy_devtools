use bevy_inspector_egui::egui::Ui;

use crate::{helpers::Tab, DevToolsResources};

pub fn tab_bar(ui: &mut Ui, resources: &mut DevToolsResources) {
    ui.columns(3, |ui| {
        if ui[0]
            .selectable_label(resources.active_tab == Tab::Diagnostics, "🔍 Diagnostics")
            .clicked()
        {
            resources.active_tab = Tab::Diagnostics;
        }
        if ui[1]
            .selectable_label(resources.active_tab == Tab::Tools, "🛠 Tools")
            .clicked()
        {
            resources.active_tab = Tab::Tools;
        }
        if ui[2]
            .selectable_label(resources.active_tab == Tab::Settings, "⚙ Settings")
            .clicked()
        {
            resources.active_tab = Tab::Settings;
        }
    });
}
