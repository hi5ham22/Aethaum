use smart_string::SmartString;
use thiserror::Error;
use crate::ecs::checker::context::{ModuleCheckContext, ModuleCheckTree};
use crate::ecs::module::{EcsThingRef, ModulePath};
use crate::toml_parser::parsed::{ComponentRef, EntityProto, EntityProtoRef, EventRef, SystemEventHandler, SystemQuery, SystemRef};

#[derive(Debug,Error)]
pub enum CrossModuleCheckError {
    #[error("Module '{module_name} has multiple definition.'")]
    MultiDefinition {
        module_name: SmartString,
    },
    #[error("'{}' is not defined in module '{}'", thing_ref.as_error_str(), thing_ref.module_name())]
    RefNotFound {
        thing_ref: EcsThingRef,
    },
    #[error("Module '{module_name}' is not found.")]
    ModuleNotFound {
        module_name: SmartString,
    },
    #[error("Unexpected Module name missing in {thing_ref}")]
    UnexpectedModuleNameMissing {
        thing_ref: EcsThingRef,
    },
    #[error("Multiple errors occurred during checking:\n{}",
        .errors.iter().map(|e| format!("  - {}", e)).collect::<Vec<_>>().join("\n"))]
    Multiple {
        errors: Vec<CrossModuleCheckError>,
    }

}
impl CrossModuleCheckError {
    pub fn raise_multi_definition(module_name: SmartString) -> Self {
        Self::MultiDefinition { module_name }
    }
    pub fn raise_ref_not_found(thing_ref: EcsThingRef) -> Self {
        Self::RefNotFound { thing_ref }
    }
    pub fn raise_module_not_found(module_name: SmartString) -> Self {
        Self::ModuleNotFound { module_name }
    }
    pub fn raise_unexpected_module_name_missing(thing_ref: EcsThingRef) -> Self {
        Self::UnexpectedModuleNameMissing { thing_ref }
    }
    pub fn raise_multiple(errors: Vec<CrossModuleCheckError>) -> Self {
        Self::Multiple { errors }
    }
}

pub struct CrossModuleChecker;

impl CrossModuleChecker {
    ///尝试注册一个模块，如果已经注册过，则返回错误(命名冲突检查)
    pub fn register_module(name: &str, context: ModuleCheckContext, tree: &mut ModuleCheckTree) -> Result<(), CrossModuleCheckError> {
        if tree.get_tree().contains_key(name) {
            return Err(
                CrossModuleCheckError::raise_multi_definition(SmartString::from(name))
            )
        }
        tree.get_tree_mut().insert(String::from(name), context);
        Ok(())
    }
    ///检查跨模块引用，应当在所有module都被注册完后调用
    pub fn check_cross_module_ref(thing_ref: &EcsThingRef, tree: &ModuleCheckTree) -> Result<(), CrossModuleCheckError> {
        if thing_ref.module_name().is_empty() {
            return Err(
                CrossModuleCheckError::raise_unexpected_module_name_missing(thing_ref.clone())
            )
        }

        let module_context = tree.get_module_context(thing_ref.module_name())
            .ok_or_else(|| CrossModuleCheckError::raise_module_not_found(SmartString::from(thing_ref.module_name())))?;

        match thing_ref {
            EcsThingRef::Component(component_ref) => {
                if !module_context.defined_components.contains(component_ref) {
                    return Err(
                        CrossModuleCheckError::raise_ref_not_found(EcsThingRef::Component(component_ref.clone()))
                    )
                }
            }
            EcsThingRef::Event(event_ref) => {
                if !module_context.defined_events.contains(event_ref) {
                    return Err(
                        CrossModuleCheckError::raise_ref_not_found(EcsThingRef::Event(event_ref.clone()))
                    )
                }
            }
            EcsThingRef::EntityProto(entity_proto_ref) => {
                if !module_context.defined_entity_protos.contains(entity_proto_ref) {
                    return Err(
                        CrossModuleCheckError::raise_ref_not_found(EcsThingRef::EntityProto(entity_proto_ref.clone()))
                    )
                }
            }
            EcsThingRef::System(system_ref) => {
                if !module_context.defined_systems.contains(system_ref) {
                    return Err(
                        CrossModuleCheckError::raise_ref_not_found(EcsThingRef::System(system_ref.clone()))
                    )
                }
            }
        }

        Ok(())
    }
}
pub trait CrossModuleCheckable {
    fn check_cross_module<'a ,RefIter>(thing_refs: RefIter, module_context: &ModuleCheckTree) -> Result<(), CrossModuleCheckError>
    where
        RefIter: Iterator<Item = &'a EcsThingRef>,
    {
        let mut errors = Vec::new();
        for thing_ref in thing_refs {
            match CrossModuleChecker::check_cross_module_ref(thing_ref, module_context) {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(CrossModuleCheckError::raise_multiple(errors))
        }
    }
}
impl CrossModuleCheckable for SystemQuery {}
impl CrossModuleCheckable for SystemEventHandler {}

impl CrossModuleCheckable for EntityProto {}