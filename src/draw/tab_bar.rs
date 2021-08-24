use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsLocation, DevToolsState, DevToolsTabs};

pub fn tab_bar(
    ui: &mut Ui,
    world: &mut World,
    location: &mut DevToolsLocation,
    tabs: &DevToolsTabs,
) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let mut resources = ignore_none_error!(
        world.get_resource_mut::<DevToolsState>(),
        "Failed to get DevToolsState resources"
    );
    ui.columns(2, |ui| {
        super::location::draw_location(&mut ui[0], location);
        ui[1].columns(tabs.0.len(), |ui| {
            for (dex, tab) in tabs.0.iter().enumerate() {
                if ui[dex]
                    .selectable_label(dex == resources.active_tab, &tab.icon)
                    .clicked()
                {
                    resources.active_tab = dex;
                }
            }
        });
    });
}
