use bevy::prelude::*;

mod value;
pub use self::value::SettingValue;

mod setting;
pub use self::setting::DevToolsSetting;

#[derive(Clone)]
pub struct DevToolsSettings(pub Vec<DevToolsSetting>);

impl DevToolsSettings {
    pub fn get_key(&self, keys: &[&str]) -> Option<&DevToolsSetting> {
        let mut current_index = 0;
        for setting in &self.0 {
            if setting.name == keys[current_index] {
                let mut current_setting = setting;
                loop {
                    current_index += 1;
                    if current_index >= keys.len() {
                        return Some(current_setting);
                    } else if let Some(setting) =
                        current_setting.get_named_child(keys[current_index])
                    {
                        current_setting = setting;
                        continue;
                    } else {
                        warn!("Setting has no child Named: {}", keys[current_index]);
                    }
                }
            }
        }
        None
    }

    pub fn get_key_mut(&mut self, keys: &[&str]) -> Option<&mut DevToolsSetting> {
        for item in self.0.iter_mut() {
            if item.name == keys[0] {
                if keys.len() == 1 {
                    return Some(item);
                } else {
                    let mut current: Option<&mut DevToolsSetting> = Some(item);
                    for key in &keys[1..] {
                        if let Some(setting) = current {
                            current = setting.get_named_child_mut(key);
                        } else {
                            unreachable!()
                        }
                    }
                    return current;
                }
            }
        }
        None
    }
}

impl Default for DevToolsSettings {
    fn default() -> DevToolsSettings {
        DevToolsSettings(vec![
            DevToolsSetting::new_labeled("devtools", "DevTools").set_value_group(vec![
                DevToolsSetting::new_labeled("enabled", "Enabled"),
                DevToolsSetting::new_labeled("always-visible", "Always Visible"),
                DevToolsSetting::new_labeled("settings", "Settings").set_value_group(vec![
                    DevToolsSetting::new_labeled("show-hidden", "Show hidden"),
                ]),
                DevToolsSetting::new_labeled("gui", "Gui").set_value_group(vec![
                    DevToolsSetting::new_labeled("widgets-hover", "Show widgets on hover"),
                    DevToolsSetting::new_labeled(
                        "widgets-taller",
                        "Show widgets that make their parent taller.",
                    ),
                    DevToolsSetting::new_labeled(
                        "widgets-wider",
                        "Show widgets that make their parent wider.",
                    ),
                    DevToolsSetting::new_labeled("show-resize", "Show Resize"),
                ]),
                DevToolsSetting::new_labeled("world", "World").set_value_group(vec![
                    DevToolsSetting::new_labeled("despawnable", "Despawnable Entities"),
                    DevToolsSetting::new_labeled("sort", "Sort Components"),
                ]),
                DevToolsSetting::new_labeled("tools", "Tools")
                    .set_hidden(true)
                    .set_value_group(vec![
                        DevToolsSetting::new_labeled("save-scene", "Save Scene")
                            .set_value_string("world.scn.ron"),
                        #[cfg(feature = "debugdump")]
                        DevToolsSetting::new_labeled("dot-render-graph", "Render Graph")
                            .set_value_string("render-graph.dot"),
                    ]),
            ]),
            #[cfg(feature = "puffin")]
            DevToolsSetting::new_labeled("puffin", "Profiler")
                .set_value_group(vec![DevToolsSetting::new_labeled("enabled", "Enabled")]),
            #[cfg(feature = "rapier3d")]
            DevToolsSetting::new_labeled("rapier", "Rapier").set_value_group(vec![
                DevToolsSetting::new_labeled("physics_pipeline_active", "Physics Pipeline Active")
                    .set_value_bool(true),
                DevToolsSetting::new_labeled("query_pipeline_active", "Query Pipeline Active")
                    .set_value_bool(true),
            ]),
        ])
    }
}
