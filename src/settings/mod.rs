use bevy::prelude::*;

mod value;
pub use self::value::SettingValue;

mod setting;
pub use self::setting::Setting;

#[derive(Clone)]
pub struct Settings(pub Vec<Setting>);

impl Settings {
    pub fn get_key(&self, keys: &[&str]) -> Option<&Setting> {
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

    pub fn get_key_mut(&mut self, keys: &[&str]) -> Option<&mut Setting> {
        for item in self.0.iter_mut() {
            if item.name == keys[0] {
                if keys.len() == 1 {
                    return Some(item);
                } else {
                    let mut current: Option<&mut Setting> = Some(item);
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

    pub fn toggle_enabled(&mut self) {
        if let Some(setting) = self.get_key_mut(&["devtools", "enabled"]) {
            if let Some(b) = setting.value.as_bool_mut() {
                *b = !*b;
            } else {
                error!("Setting `devtools -> enabled` is not a bool");
            }
        } else {
            error!("Setting `devtools -> enabled` is was not found");
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings(vec![Setting::new_labeled("devtools", "DevTools")
            .set_value_group(vec![
                Setting::new_labeled("enabled", "Enabled"),
                Setting::new_labeled("active_panel", "Active Panel")
                    .set_hidden(true)
                    .set_value_integer(0),
                Setting::new_labeled("settings", "Settings").set_value_group(vec![
                    Setting::new_labeled("show-hidden", "Show hidden"),
                ]),
                Setting::new_labeled("gui", "Gui").set_value_group(vec![
                    Setting::new_labeled("widgets-hover", "Show widgets on hover"),
                    Setting::new_labeled(
                        "widgets-taller",
                        "Show widgets that make their parent taller.",
                    ),
                    Setting::new_labeled(
                        "widgets-wider",
                        "Show widgets that make their parent wider.",
                    ),
                    Setting::new_labeled("show-resize", "Show Resize"),
                ]),
                Setting::new_labeled("world", "World").set_value_group(vec![
                    Setting::new_labeled("despawnable", "Despawnable Entities"),
                    Setting::new_labeled("sort", "Sort Components"),
                ]),
                Setting::new_labeled("tools", "Tools")
                    .set_hidden(true)
                    .set_value_group(vec![
                        Setting::new_labeled("save-scene", "Save Scene")
                            .set_value_string("world.scn.ron"),
                        #[cfg(feature = "debugdump")]
                        Setting::new_labeled("dot-render-graph", "Render Graph")
                            .set_value_string("render-graph.dot"),
                    ]),
            ])])
    }
}
