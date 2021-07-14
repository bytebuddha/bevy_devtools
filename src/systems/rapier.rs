use bevy::prelude::*;
#[cfg(feature = "rapier3d")]
use bevy_rapier3d::physics::RapierConfiguration;
#[cfg(feature = "rapier2d")]
use bevy_rapier3d::physics::RapierConfiguration;

use crate::{DevToolsSettings, SettingValue};

pub fn rapier_settings(settings: Res<DevToolsSettings>, mut conf: ResMut<RapierConfiguration>) {
    for setting in settings.0.iter() {
        if setting.name == "rapier" {
            for child in setting.children().unwrap() {
                if let SettingValue::Bool(value) = child.value {
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
        }
    }
}
