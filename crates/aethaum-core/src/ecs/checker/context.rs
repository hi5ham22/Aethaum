use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use smart_string::SmartString;
use crate::toml_parser::parsed::{AethaumType, ComponentRef, EntityProtoRef, EventRef, PrimitiveType, SystemRef};
use anyhow::Result;
use crate::ecs::module::EcsModule;

pub struct ModuleCheckTree {
    modules: HashMap<String, ModuleCheckContext> // 模块名 -> 模块上下文
    //目前，模块是扁平的结构，暂时不需要树结构
}
impl ModuleCheckTree {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    pub fn with_module_contexts(mut self, module_contexts: Vec<ModuleCheckContext>) -> Self {
        for module_context in module_contexts {
            self.modules.insert(module_context.name.as_str().into(), module_context);
        }
        self
    }
    pub fn get_module_context(&self, name: &str) -> Option<&ModuleCheckContext> {
        self.modules.get(name)
    }
    pub fn get_module_context_mut(&mut self, name: &str) -> Option<&mut ModuleCheckContext> {
        self.modules.get_mut(name)
    }
    pub fn get_tree(&self) -> &HashMap<String, ModuleCheckContext> {
        &self.modules
    }
    pub fn get_tree_mut(&mut self) -> &mut HashMap<String, ModuleCheckContext> {
        &mut self.modules
    }
}
pub enum ECSThing {
    Component,
    Event,
    EntityProto,
    System,
}

//单个模块的ECS上下文
#[derive(Debug)]
pub struct ModuleCheckContext {
    pub name: SmartString,
    pub defined_components: HashSet<ComponentRef>, //TODO: change to use one HashSet to prevent multiple identifiers
    pub defined_events: HashSet<EventRef>,
    pub defined_entity_protos: HashSet<EntityProtoRef>,
    pub defined_systems: HashSet<SystemRef>,
    pub project_root: PathBuf,
}
impl ModuleCheckContext {
    pub fn new(name: SmartString, project_root: PathBuf) -> Self {
        Self {
            name,
            defined_components: HashSet::new(),
            defined_events: HashSet::new(),
            defined_entity_protos: HashSet::new(),
            defined_systems: HashSet::new(),
            project_root,
        }
    }
}


