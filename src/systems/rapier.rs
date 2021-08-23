use bevy::prelude::*;
#[cfg(feature = "rapier2d")]
use bevy_rapier2d::physics::RapierConfiguration;
#[cfg(feature = "rapier3d")]
use bevy_rapier3d::physics::RapierConfiguration;

use crate::DevToolsSettings;

pub fn rapier_settings(settings: Res<DevToolsSettings>, mut conf: ResMut<RapierConfiguration>) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    if let Some(setting) = settings.get_key(&["rapier"]) {
        if let Some(children) = setting.get_group() {
            for child in children {
                if let Some(value) = child.value.as_bool() {
                    if child.name == "query_pipeline_active" {
                        if conf.query_pipeline_active != value {
                            conf.query_pipeline_active = value;
                        }
                    }
                    if child.name == "physics_pipeline_active" {
                        if conf.physics_pipeline_active != value {
                            conf.physics_pipeline_active = value;
                        }
                    }
                }
            }
        } else {
            warn!("Settings key rapier was not a group");
        }
    } else {
        warn!("Settings key rapier was not found");
    }
}

pub fn initial_rapier_settings(
    mut settings: ResMut<DevToolsSettings>,
    conf: ResMut<RapierConfiguration>,
) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    if let Some(setting) = settings.get_key_mut(&["rapier"]) {
        if let Some(group) = setting.get_group_mut() {
            for child in group {
                if let Some(value) = child.value.as_bool_mut() {
                    if child.name == "query_pipeline_active" {
                        *value = conf.query_pipeline_active;
                    }
                    if child.name == "physics_pipeline_active" {
                        *value = conf.physics_pipeline_active;
                    }
                }
            }
        } else {
            warn!("Settings key rapier was not a group");
        }
    } else {
        warn!("Settings key rapier was not found");
    }
}
