use smart_string::SmartString;
use crate::toml_parser::parsed::{Component, ComponentRef, EntityProto, EntityProtoRef, Event, EventRef, System, SystemRef};

pub enum EcsThingRef {
    Component(ComponentRef),
    Event(EventRef),
    EntityProto(EntityProtoRef),
    System(SystemRef)
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

pub struct ModulePath<'a> {
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
}