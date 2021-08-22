use bevy::app::AppBuilder;

use crate::{ DevToolsDiagnostics, DiagnosticGroup, DevToolsSetting, DevToolsSettings, DevTool, DevToolsTools};

pub trait DevToolsExt {
    fn devtools_tool(&mut self, tool: DevTool)  -> &mut AppBuilder;
    fn devtools_setting(&mut self, setting: DevToolsSetting) -> &mut AppBuilder;
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder;

    fn devtools_with_tools<F: FnMut(&mut DevToolsTools)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_settings<F: FnMut(&mut DevToolsSettings)>(&mut self, func: F) -> &mut AppBuilder;
    fn devtools_with_diagnostics<F: FnMut(&mut DevToolsDiagnostics)>(&mut self, func: F) -> &mut AppBuilder;
}

impl <'a>DevToolsExt for &'a mut AppBuilder {
    fn devtools_tool(&mut self, tool: DevTool) -> &mut AppBuilder {
        let mut tools = self.world_mut().get_resource_mut::<DevToolsTools>().unwrap();
        tools.0.push(tool);
        self
    }
    fn devtools_setting(&mut self, setting: DevToolsSetting) -> &mut AppBuilder {
        let mut settings = self.world_mut().get_resource_mut::<DevToolsSettings>().unwrap();
        settings.0.push(setting);
        self
    }
    fn devtools_diagnostic(&mut self, diagnostic: DiagnosticGroup) -> &mut AppBuilder {
        let mut diagnostics = self.world_mut().get_resource_mut::<DevToolsDiagnostics>().unwrap();
        diagnostics.0.push(diagnostic);
        self
    }

    fn devtools_with_tools<F: FnMut(&mut DevToolsTools)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut tools = self.world_mut().get_resource_mut::<DevToolsTools>().unwrap();
        func(&mut *tools);
        self
    }
    fn devtools_with_settings<F: FnMut(&mut DevToolsSettings)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut settings = self.world_mut().get_resource_mut::<DevToolsSettings>().unwrap();
        func(&mut *settings);
        self
    }
    fn devtools_with_diagnostics<F: FnMut(&mut DevToolsDiagnostics)>(&mut self, mut func: F) -> &mut AppBuilder {
        let mut diagnostics = self.world_mut().get_resource_mut::<DevToolsDiagnostics>().unwrap();
        func(&mut *diagnostics);
        self
    }
}
