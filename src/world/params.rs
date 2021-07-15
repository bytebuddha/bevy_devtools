use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::render::render_graph::base::MainPass;
use bevy_inspector_egui::options::EntityAttributes;

use crate::DevToolsSettings;

use std::any::TypeId;

/// Resource which controls the way the world inspector is shown.
#[derive(Debug, Clone)]
pub struct WorldInspectorParams {
    /// these components will be ignored
    pub ignore_components: HashSet<TypeId>,
    /// these components will be read only
    pub read_only_components: HashSet<TypeId>,
    /// Whether to sort the components alphabetically
    pub sort_components: bool,
    /// Whether entities can be despawned
    pub despawnable_entities: bool
}

impl WorldInspectorParams {
    pub fn apply_settings<'a, 'b>(params: &'a mut WorldInspectorParams, settings: &'b DevToolsSettings) {
        if let Some(setting) = settings.named("devtools") {
            if let Some(child) = setting.named_child("world") {
                for child in child.children().unwrap() {
                    if let Some(value) = child.value.as_bool() {
                        if child.name == "despawnable" && params.despawnable_entities != value {
                            params.despawnable_entities = value;
                        }
                        if child.name == "sort" && params.sort_components != value {
                            params.sort_components = value;
                        }

                    }
                }
            }
        }
    }
}

impl WorldInspectorParams {
    fn empty() -> Self {
        WorldInspectorParams {
            ignore_components: HashSet::default(),
            read_only_components: HashSet::default(),
            sort_components: false,
            despawnable_entities: false
        }
    }

    pub(crate) fn should_ignore_component(&self, type_id: TypeId) -> bool {
        self.ignore_components.contains(&type_id)
    }

    pub(crate) fn is_read_only(&self, type_id: TypeId) -> bool {
        self.read_only_components.contains(&type_id)
    }

    pub(crate) fn entity_options(&self) -> EntityAttributes {
        EntityAttributes {
            despawnable: self.despawnable_entities,
        }
    }
}

impl Default for WorldInspectorParams {
    fn default() -> Self {
        let mut params = WorldInspectorParams::empty();

        params.ignore_components = [
            TypeId::of::<Name>(),
            TypeId::of::<Children>(),
            TypeId::of::<Parent>(),
            TypeId::of::<PreviousParent>(),
            TypeId::of::<MainPass>(),
            TypeId::of::<Draw>(),
            TypeId::of::<RenderPipelines>(),
        ]
        .iter()
        .copied()
        .collect();
        params.read_only_components = [TypeId::of::<GlobalTransform>()].iter().copied().collect();

        #[cfg(feature = "rapier3d")]
        {
            params
                .ignore_components
                .insert(TypeId::of::<bevy_rapier3d::prelude::RigidBodyIds>());
            params
                .ignore_components
                .insert(TypeId::of::<bevy_rapier3d::prelude::ColliderBroadPhaseData>());
        }
        #[cfg(feature = "rapier2d")]
        {
            params
                .ignore_components
                .insert(TypeId::of::<bevy_rapier2d::prelude::RigidBodyIds>());
            params
                .ignore_components
                .insert(TypeId::of::<bevy_rapier2d::prelude::ColliderBroadPhaseData>());
        }

        params
    }
}
