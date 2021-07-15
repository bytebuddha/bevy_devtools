use bevy::prelude::*;
#[cfg(feature = "rapier3d")]
use bevy_rapier3d::physics::RapierConfiguration;
#[cfg(feature = "rapier2d")]
use bevy_rapier3d::physics::RapierConfiguration;

use crate::{DevToolsSettings, SettingValue};

pub fn rapier_settings(settings: Res<DevToolsSettings>, mut conf: ResMut<RapierConfiguration>) {
    if let Some(setting) = settings.named("rapier") {
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
            if let SettingValue::Float(ref float) = child.value {
                if child.name == "scale" {
                    if conf.scale != *float {
                        conf.scale = *float;
                    }
                }
            }
        }
    }
}

pub fn initial_rapier_settings(mut settings: ResMut<DevToolsSettings>, conf: ResMut<RapierConfiguration>) {
    if let Some(setting) = settings.named_mut("rapier") {
        for child in setting.children_mut().unwrap() {
            if let SettingValue::Bool(ref mut value) = child.value {
                if child.name == "query_pipeline_active" {
                    *value = conf.query_pipeline_active;
                }
                if child.name == "physics_pipeline_active" {
                    *value = conf.physics_pipeline_active;
                }
            }
            if let SettingValue::Float(ref mut float) = child.value {
                if child.name == "scale" {
                    *float = conf.scale;
                }
            }
        }
    }
}
