use bevy::app::AppBuilder;
use bevy::input::keyboard::KeyCode;

use crate::{
    DevTool, DevToolsDiagnostics, DevToolsLocation, DevToolsSetting, DevToolsSettings,
    DevToolsState, DevToolsTab, DevToolsTabs, DevToolsTools, DiagnosticGroup,
};

pub trait DevToolsExt {
    fn devtools_enabled(&mut self) -> &mut AppBuilder;
    fn devtools_active_tab(&mut self, _: usize) -> &mut AppBuilder;
    fn devtools_toggle_key(&mut self, _: KeyCode) -> &mut AppBuilder;
    fn devtools_location(&mut self, _: DevToolsLocation) -> &mut AppBuilder;
    #[cfg(feature = "puffin")]
    fn devtools_profiler_key(&mut self, _: KeyCode) -> &mut AppBuilder;

    fn devtools_tab(&mut self, tab: DevToolsTab) -> &mut AppBuilder;
    fn devtools_tool(&mut self, tool: DevTool) -> &mut AppBuilder;
    fn devtools_setting(&mut self, setting: DevToolsSetting) -> &mut AppBuilder;
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder;

    fn devtools_with_tabs<F: FnMut(&mut DevToolsTabs)>(&mut self, func: F) -> &mut AppBuilder;
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
    fn devtools_active_tab(&mut self, tab: usize) -> &mut AppBuilder {
        self.world_mut()
            .get_resource_mut::<DevToolsState>()
            .unwrap()
            .active_tab = tab;
        self
    }
    fn devtools_toggle_key(&mut self, key: KeyCode) -> &mut AppBuilder {
        self.world_mut()
            .get_resource_mut::<DevToolsState>()
            .unwrap()
            .toggle_key = key;
        self
    }
    #[cfg(feature = "puffin")]
    fn devtools_profiler_key(&mut self, key: KeyCode) -> &mut AppBuilder {
        self.world_mut()
            .get_resource_mut::<DevToolsState>()
            .unwrap()
            .profiler_key = key;
        self
    }
    fn devtools_location(&mut self, location: DevToolsLocation) -> &mut AppBuilder {
        self.world_mut()
            .get_resource_mut::<DevToolsState>()
            .unwrap()
            .location = location;
        self
    }
    fn devtools_tab(&mut self, tab: DevToolsTab) -> &mut AppBuilder {
        let mut tabs = self.world_mut().get_resource_mut::<DevToolsTabs>().unwrap();
        tabs.0.push(tab);
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

    fn devtools_with_tabs<F: FnMut(&mut DevToolsTabs)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut tabs = self.world_mut().get_resource_mut::<DevToolsTabs>().unwrap();
        func(&mut *tabs);
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
