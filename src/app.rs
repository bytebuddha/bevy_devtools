use bevy::app::AppBuilder;

use crate::{
    Tool, DiagnosticPanel, Setting, Settings, Panel, Panels,
    Tools, DiagnosticGroup,
};

pub trait DevToolsExt {
    fn devtools_enabled(&mut self) -> &mut AppBuilder;
    fn devtools_active_panel(&mut self, _: usize) -> &mut AppBuilder;

    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut AppBuilder;
    fn devtools_panel(&mut self, panel: Panel) -> &mut AppBuilder;
    fn devtools_panel_index(&mut self, index: usize, panel: Panel) -> &mut AppBuilder;
    fn devtools_tool(&mut self, tool: Tool) -> &mut AppBuilder;
    fn devtools_setting(&mut self, setting: Setting) -> &mut AppBuilder;
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder;

    fn devtools_with_panels<F: FnMut(&mut Panels)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_tools<F: FnMut(&mut Tools)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_settings<F: FnMut(&mut Settings)>(
        &mut self,
        func: F,
    ) -> &mut AppBuilder;
    fn devtools_with_diagnostics<F: FnMut(&mut DiagnosticPanel)>(
        &mut self,
        func: F,
    ) -> &mut AppBuilder;
}

impl<'a> DevToolsExt for &'a mut AppBuilder {
    fn devtools_enabled(&mut self) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<Settings>()
            .unwrap();
        settings.toggle_enabled();
        self
    }
    fn devtools_active_panel(&mut self, panel: usize) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<Settings>()
            .unwrap();
        if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
            if let Some(num) = setting.value.as_integer_mut() {
                *num = panel as i32;
            }
        }
        self
    }
    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut AppBuilder {
        self.world_mut().insert_resource(crate::TopPanel(func));
        self
    }
    fn devtools_panel(&mut self, panel: Panel) -> &mut AppBuilder {
        let mut panels = self.world_mut().get_resource_mut::<Panels>().unwrap();
        panels.0.push(panel);
        self
    }
    fn devtools_panel_index(&mut self, index: usize, panel: Panel) -> &mut AppBuilder {
        let mut panels = self.world_mut().get_resource_mut::<Panels>().unwrap();
        panels.0.insert(index, panel);
        self
    }
    fn devtools_tool(&mut self, tool: Tool) -> &mut AppBuilder {
        let mut tools = self
            .world_mut()
            .get_resource_mut::<Tools>()
            .unwrap();
        tools.0.push(tool);
        self
    }
    fn devtools_setting(&mut self, setting: Setting) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<Settings>()
            .unwrap();
        settings.0.push(setting);
        self
    }
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder {
        let mut diagnostics = self
            .world_mut()
            .get_resource_mut::<DiagnosticPanel>()
            .unwrap();
        diagnostics.0.push(diagnostic);
        self
    }

    fn devtools_with_panels<F: FnMut(&mut Panels)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut panels = self.world_mut().get_resource_mut::<Panels>().unwrap();
        func(&mut *panels);
        self
    }
    fn devtools_with_tools<F: FnMut(&mut Tools)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut tools = self
            .world_mut()
            .get_resource_mut::<Tools>()
            .unwrap();
        func(&mut *tools);
        self
    }
    fn devtools_with_settings<F: FnMut(&mut Settings)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut settings = self
            .world_mut()
            .get_resource_mut::<Settings>()
            .unwrap();
        func(&mut *settings);
        self
    }
    fn devtools_with_diagnostics<F: FnMut(&mut DiagnosticPanel)>(
        &mut self,
        mut func: F,
    ) -> &mut AppBuilder {
        let mut diagnostics = self
            .world_mut()
            .get_resource_mut::<DiagnosticPanel>()
            .unwrap();
        func(&mut *diagnostics);
        self
    }
}
