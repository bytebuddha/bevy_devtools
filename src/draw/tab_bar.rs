use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsLocation, DevToolsState, DevToolsTab};

pub fn tab_bar(ui: &mut Ui, world: &mut World, location: &mut DevToolsLocation) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let mut resources = ignore_none_error!(
        world.get_resource_mut::<DevToolsState>(),
        "Failed to get DevToolsState resources"
    );
    ui.columns(2, |ui| {
        super::location::draw_location(&mut ui[0], location);
        ui[1].columns(4, |ui| {
            if ui[0]
                .selectable_label(
                    resources.active_tab == DevToolsTab::Diagnostics,
                    DevToolsTab::Diagnostics.icon(),
                )
                .clicked()
            {
                resources.active_tab = DevToolsTab::Diagnostics;
            }
            if ui[1]
                .selectable_label(
                    resources.active_tab == DevToolsTab::World,
                    DevToolsTab::World.icon(),
                )
                .clicked()
            {
                resources.active_tab = DevToolsTab::World;
            }
            if ui[2]
                .selectable_label(
                    resources.active_tab == DevToolsTab::Tools,
                    DevToolsTab::Tools.icon(),
                )
                .clicked()
            {
                resources.active_tab = DevToolsTab::Tools;
            }
            if ui[3]
                .selectable_label(
                    resources.active_tab == DevToolsTab::Settings,
                    DevToolsTab::Settings.icon(),
                )
                .clicked()
            {
                resources.active_tab = DevToolsTab::Settings;
            }
        });
    });
}
