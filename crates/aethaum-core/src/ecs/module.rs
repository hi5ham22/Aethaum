use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use smart_string::SmartString;
use thiserror::Error;
use crate::toml_parser::parsed::{Component, ComponentRef, EntityProto, EntityProtoRef, Event, EventRef, System, SystemRef, World};
use crate::toml_parser::raw::{RawComponent, RawEntityProto, RawEvent, RawSystem};

#[derive(Debug)]
#[derive(Clone)]
pub enum EcsThingRef {
    Component(ComponentRef),
    Event(EventRef),
    EntityProto(EntityProtoRef),
    System(SystemRef)
}
impl EcsThingRef {
    pub fn as_error_str(&self) -> String {
        match self {
            EcsThingRef::Component(component_ref) => {
                format!("Component \"{}\"", component_ref)
            },
            EcsThingRef::Event(event_ref) => {
                format!("Event \"{}\"", event_ref)
            },
            EcsThingRef::EntityProto(entity_proto_ref) => {
                format!("Entity Proto \"{}\"", entity_proto_ref)
            },
            EcsThingRef::System(system_ref) => {
                format!("System \"{}\"", system_ref)
            }
        }
    }
    pub fn module_name(&self) -> &str {
        match self {
            EcsThingRef::Component(component_ref) => {
                component_ref.module_name.as_ref().map(|s| s.as_str()).unwrap_or_default()
            },
            EcsThingRef::Event(event_ref) => {
                event_ref.module_name.as_ref().map(|s| s.as_str()).unwrap_or_default()
            },
            EcsThingRef::EntityProto(entity_proto_ref) => {
                entity_proto_ref.module_name.as_ref().map(|s| s.as_str()).unwrap_or_default()
            }
            EcsThingRef::System(system_ref) => {
                system_ref.module_name.as_ref().map(|s| s.as_str()).unwrap_or_default()
            }
        }
    }
    pub fn name(&self) -> &str {
        match self {
            EcsThingRef::Component(component_ref) => {
                component_ref.name.as_str()
            }
            EcsThingRef::Event(event_ref) => {
                event_ref.name.as_str()
            }
            EcsThingRef::EntityProto(entity_proto_ref) => {
                entity_proto_ref.name.as_str()
            }
            EcsThingRef::System(system_ref) => {
                system_ref.name.as_str()
            }
        }
    }
}
impl std::fmt::Display for EcsThingRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcsThingRef::Component(ref_name) => write!(f, "{}", ref_name),
            EcsThingRef::Event(ref_name) => write!(f, "{}", ref_name),
            EcsThingRef::EntityProto(ref_name) => write!(f, "{}", ref_name),
            EcsThingRef::System(ref_name) => write!(f, "{}", ref_name)
        }
    }
}
impl From<ComponentRef> for EcsThingRef {
    fn from(component_ref: ComponentRef) -> Self {
        EcsThingRef::Component(component_ref)
    }
}
impl From<EventRef> for EcsThingRef {
    fn from(event_ref: EventRef) -> Self {
        EcsThingRef::Event(event_ref)
    }
}
impl From<EntityProtoRef> for EcsThingRef {
    fn from(entity_proto_ref: EntityProtoRef) -> Self {
        EcsThingRef::EntityProto(entity_proto_ref)
    }
}
impl From<SystemRef> for EcsThingRef {
    fn from(system_ref: SystemRef) -> Self {
        EcsThingRef::System(system_ref)
    }
}
pub struct  ModulePath<'a> {
    pub module_name: &'a SmartString,
    pub thing_name: &'a EcsThingRef
}


pub struct EcsModule {
    pub name: SmartString,
    pub components: Option<Vec<Component>>,
    pub events: Option<Vec<Event>>,
    pub entity_protos: Option<Vec<EntityProto>>,
    pub systems: Option<Vec<System>>,
}
impl EcsModule {
    pub fn new_empty(name: SmartString) -> Self {
        Self {
            name,
            components: None,
            events: None,
            entity_protos: None,
            systems: None,
        }
    }
    pub fn with_components(mut self, components: Vec<Component>) -> Self {
        self.components = Some(components);
        self
    }
    pub fn with_events(mut self, events: Vec<Event>) -> Self {
        self.events = Some(events);
        self
    }
    pub fn with_entity_protos(mut self, entity_protos: Vec<EntityProto>) -> Self {
        self.entity_protos = Some(entity_protos);
        self
    }
    pub fn with_systems(mut self, systems: Vec<System>) -> Self {
        self.systems = Some(systems);
        self
    }
    pub fn with_option_components(mut self, components: Option<Vec<Component>>) -> Self {
        match components {
            Some(components) => self.with_components(components),
            None => self
        }
    }
    pub fn with_option_events(mut self, events: Option<Vec<Event>>) -> Self {
        match events {
            Some(events) => self.with_events(events),
            None => self
        }
    }
    pub fn with_option_entity_protos(mut self, entity_protos: Option<Vec<EntityProto>>) -> Self {
        match entity_protos {
            Some(entity_protos) => self.with_entity_protos(entity_protos),
            None => self
        }
    }
    pub fn with_option_systems(mut self, systems: Option<Vec<System>>) -> Self {
        match systems {
            Some(systems) => self.with_systems(systems),
            None => self
        }
    }
}
pub struct EcsModuleTree {
    tree: HashMap<SmartString, EcsModule>
}
impl EcsModuleTree {
    pub fn new_empty() -> Self {
        Self {
            tree: HashMap::new()
        }
    }
    pub fn with_modules(mut self, modules: Vec<EcsModule>) -> Self {
        for module in modules {
            self.insert_module(module);
        }
        self
    }
    pub fn get_module(&self, module_name: &str) -> Option<&EcsModule> {
        self.tree.get(module_name)
    }
    pub fn get_module_mut(&mut self, module_name: &str) -> Option<&mut EcsModule> {
        self.tree.get_mut(module_name)
    }
    pub fn insert_module(&mut self, module: EcsModule) {
        self.tree.insert(module.name.clone(), module);
    }
    pub fn get_modules(&self) -> Vec<&EcsModule> {
        self.tree.values().collect()
    }
}
pub struct AethaumProject {
    pub root: PathBuf,
    pub world: World,
    pub module_tree: EcsModuleTree
}

impl AethaumProject {
    pub fn new(root: PathBuf,world: World, module_tree: EcsModuleTree) -> Self {
        Self {
            root,
            world,
            module_tree
        }
    }
}
