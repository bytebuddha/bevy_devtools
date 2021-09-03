use bevy::app::AppBuilder;

use crate::{
    DevTool, DevToolsDiagnostics, DevToolsSetting, DevToolsSettings, DevToolsPanel, DevToolsPanels,
    DevToolsTools, DiagnosticGroup,
};

pub trait DevToolsExt {
    fn devtools_enabled(&mut self) -> &mut AppBuilder;
    fn devtools_active_panel(&mut self, _: usize) -> &mut AppBuilder;

    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut AppBuilder;
    fn devtools_panel(&mut self, panel: DevToolsPanel) -> &mut AppBuilder;
    fn devtools_tool(&mut self, tool: DevTool) -> &mut AppBuilder;
    fn devtools_setting(&mut self, setting: DevToolsSetting) -> &mut AppBuilder;
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder;

    fn devtools_with_panels<F: FnMut(&mut DevToolsPanels)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_tools<F: FnMut(&mut DevToolsTools)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_settings<F: FnMut(&mut DevToolsSettings)>(
        &mut self,
        func: F,
    ) -> &mut AppBuilder;
    fn devtools_with_diagnostics<F: FnMut(&mut DevToolsDiagnostics)>(
        &mut self,
        func: F,
    ) -> &mut AppBuilder;
}

impl<'a> DevToolsExt for &'a mut AppBuilder {
    fn devtools_enabled(&mut self) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<DevToolsSettings>()
            .unwrap();
        if let Some(key) = settings.get_key_mut(&["devtools", "enabled"]) {
            if let Some(value) = key.value.as_bool_mut() {
                *value = !*value;
            }
        }
        self
    }
    fn devtools_active_panel(&mut self, panel: usize) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<DevToolsSettings>()
            .unwrap();
        if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
            if let Some(num) = setting.value.as_integer_mut() {
                *num = panel as i32;
            }
        }
        self
    }
    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut AppBuilder {
        self.world_mut().insert_resource(crate::DevToolsTopPanel(func));
        self
    }
    fn devtools_panel(&mut self, panel: DevToolsPanel) -> &mut AppBuilder {
        let mut panels = self.world_mut().get_resource_mut::<DevToolsPanels>().unwrap();
        panels.0.push(panel);
        self
    }
    fn devtools_tool(&mut self, tool: DevTool) -> &mut AppBuilder {
        let mut tools = self
            .world_mut()
            .get_resource_mut::<DevToolsTools>()
            .unwrap();
        tools.0.push(tool);
        self
    }
    fn devtools_setting(&mut self, setting: DevToolsSetting) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<DevToolsSettings>()
            .unwrap();
        settings.0.push(setting);
        self
    }
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder {
        let mut diagnostics = self
            .world_mut()
            .get_resource_mut::<DevToolsDiagnostics>()
            .unwrap();
        diagnostics.0.push(diagnostic);
        self
    }

    fn devtools_with_panels<F: FnMut(&mut DevToolsPanels)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut panels = self.world_mut().get_resource_mut::<DevToolsPanels>().unwrap();
        func(&mut *panels);
        self
    }
    fn devtools_with_tools<F: FnMut(&mut DevToolsTools)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut tools = self
            .world_mut()
            .get_resource_mut::<DevToolsTools>()
            .unwrap();
        func(&mut *tools);
        self
    }
    fn devtools_with_settings<F: FnMut(&mut DevToolsSettings)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<DevToolsSettings>()
            .unwrap();
        func(&mut *settings);
        self
    }
    fn devtools_with_diagnostics<F: FnMut(&mut DevToolsDiagnostics)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut diagnostics = self
            .world_mut()
            .get_resource_mut::<DevToolsDiagnostics>()
            .unwrap();
        func(&mut *diagnostics);
        self
    }
}
