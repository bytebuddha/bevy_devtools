use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{helpers::Tab, DevToolsState};

pub fn tab_bar(ui: &mut Ui, world: &mut World) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let mut resources = world.get_resource_mut::<DevToolsState>().unwrap();
    ui.columns(4, |ui| {
        if ui[0]
            .selectable_label(
                resources.active_tab == Tab::Diagnostics,
                Tab::Diagnostics.icon(),
            )
            .clicked()
        {
            resources.active_tab = Tab::Diagnostics;
        }
        if ui[1]
            .selectable_label(resources.active_tab == Tab::World, Tab::World.icon())
            .clicked()
        {
            resources.active_tab = Tab::World;
        }
        if ui[2]
            .selectable_label(resources.active_tab == Tab::Tools, Tab::Tools.icon())
            .clicked()
        {
            resources.active_tab = Tab::Tools;
        }
        if ui[3]
            .selectable_label(resources.active_tab == Tab::Settings, Tab::Settings.icon())
            .clicked()
        {
            resources.active_tab = Tab::Settings;
        }
    });
}
