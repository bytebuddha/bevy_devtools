use bevy::app::App;

use crate::{
    Tool, DiagnosticPanel, Setting, Settings, Panel, Panels,
    Tools, DiagnosticGroup,
};

pub trait DevToolsExt {
    fn devtools_enabled(&mut self) -> &mut App;
    fn devtools_active_panel(&mut self, _: usize) -> &mut App;

    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut App;
    fn devtools_panel(&mut self, panel: Panel) -> &mut App;
    fn devtools_panel_index(&mut self, index: usize, panel: Panel) -> &mut App;
    fn devtools_tool(&mut self, tool: Tool) -> &mut App;
    fn devtools_setting(&mut self, setting: Setting) -> &mut App;
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut App;

    fn devtools_with_panels<F: FnMut(&mut Panels)>(&mut self, func: F) -> &mut App;
    fn devtools_with_tools<F: FnMut(&mut Tools)>(&mut self, func: F) -> &mut App;
    fn devtools_with_settings<F: FnMut(&mut Settings)>(
        &mut self,
        func: F,
    ) -> &mut App;
    fn devtools_with_diagnostics<F: FnMut(&mut DiagnosticPanel)>(
        &mut self,
        func: F,
    ) -> &mut App;
}

impl<'a> DevToolsExt for &'a mut App {
    fn devtools_enabled(&mut self) -> &mut App {
        let mut settings = self.world.get_resource_mut::<Settings>().unwrap();
        settings.toggle_enabled();
        self
    }
    fn devtools_active_panel(&mut self, panel: usize) -> &mut App {
        let mut settings = self.world.get_resource_mut::<Settings>().unwrap();
        if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
            if let Some(num) = setting.value.as_integer_mut() {
                *num = panel as i32;
            }
        }
        self
    }
    fn devtools_top_panel(&mut self, func: fn(&mut crate::egui::Ui, &mut bevy::prelude::World)) -> &mut App {
        self.world.insert_resource(crate::TopPanel(func));
        self
    }
    fn devtools_panel(&mut self, panel: Panel) -> &mut App {
        let mut panels = self.world.get_resource_mut::<Panels>().unwrap();
        panels.0.push(panel);
        self
    }
    fn devtools_panel_index(&mut self, index: usize, panel: Panel) -> &mut App {
        let mut panels = self.world.get_resource_mut::<Panels>().unwrap();
        panels.0.insert(index, panel);
        self
    }
    fn devtools_tool(&mut self, tool: Tool) -> &mut App {
        let mut tools = self
            .world
            .get_resource_mut::<Tools>()
            .unwrap();
        tools.0.push(tool);
        self
    }
    fn devtools_setting(&mut self, setting: Setting) -> &mut App {
        let mut settings = self
            .world
            .get_resource_mut::<Settings>()
            .unwrap();
        settings.0.push(setting);
        self
    }
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut App {
        let mut diagnostics = self
            .world
            .get_resource_mut::<DiagnosticPanel>()
            .unwrap();
        diagnostics.0.push(diagnostic);
        self
    }

    fn devtools_with_panels<F: FnMut(&mut Panels)>(&mut self, mut func: F) -> &mut App {
        let mut panels = self.world.get_resource_mut::<Panels>().unwrap();
        func(&mut *panels);
        self
    }
    fn devtools_with_tools<F: FnMut(&mut Tools)>(
        &mut self,
        mut func: F,
    ) -> &mut App {
        let mut tools = self.world.get_resource_mut::<Tools>()
            .unwrap();
        func(&mut *tools);
        self
    }
    fn devtools_with_settings<F: FnMut(&mut Settings)>(
        &mut self,
        mut func: F,
    ) -> &mut App {
        let mut settings = self.world.get_resource_mut::<Settings>().unwrap();
        func(&mut *settings);
        self
    }
    fn devtools_with_diagnostics<F: FnMut(&mut DiagnosticPanel)>(
        &mut self,
        mut func: F,
    ) -> &mut App {
        let mut diagnostics = self.world.get_resource_mut::<DiagnosticPanel>()
            .unwrap();
        func(&mut *diagnostics);
        self
    }
}
