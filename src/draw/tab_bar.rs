use bevy::prelude::*;
use bevy_inspector_egui::egui::{Ui, Layout, Align};

use crate::{helpers::Tab, DevToolsState};

pub fn tab_bar(ui: &mut Ui, world: &mut World) {
    let mut resources = world.get_resource_mut::<DevToolsState>().unwrap();
    ui.columns(4, |ui| {
        let layout = Layout::top_down(Align::Center);
        ui[0].with_layout(layout.clone(), |ui| {
            if ui.selectable_label(resources.active_tab == Tab::Diagnostics, Tab::Diagnostics.icon())
            .clicked()
            {
                resources.active_tab = Tab::Diagnostics;
            }
        });
        ui[1].with_layout(layout.clone(), |ui| {
            if ui.selectable_label(resources.active_tab == Tab::World, Tab::World.icon())
            .clicked()
            {
                resources.active_tab = Tab::World;
            }
        });
        ui[2].with_layout(layout, |ui| {
            if ui.selectable_label(resources.active_tab == Tab::Tools, Tab::Tools.icon())
            .clicked()
            {
                resources.active_tab = Tab::Tools;
            }
        });
        ui[3].with_layout(layout, |ui| {
            if ui.selectable_label(resources.active_tab == Tab::Settings, Tab::Settings.icon())
            .clicked()
            {
                resources.active_tab = Tab::Settings;
            }
        });
    });
}
