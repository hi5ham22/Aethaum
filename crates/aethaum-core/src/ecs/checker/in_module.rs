use thiserror::Error;
use crate::ecs::checker::context::{ModuleCheckContext};
use crate::ecs::module::{EcsThingRef};
use crate::toml_parser::parsed::{ComponentRef, EntityProto, EntityProtoRef, EventRef, SystemEventHandler, SystemQuery, SystemRef};

#[derive(Debug,Error)]
pub enum InModuleCheckError {
    #[error("'{}' is already defined.",thing_ref.as_error_str())]
    AlreadyDefined {
        thing_ref: EcsThingRef,
    },
    #[error("'{}' is defined in external module '{}'",thing_ref.as_error_str(), thing_ref.module_name())]
    DefineExternal {
        thing_ref: EcsThingRef,
    },
    #[error("'{}' is not defined in current module.",thing_ref.as_error_str())]
    NotDefined {
        thing_ref: EcsThingRef,
    },
    #[error("'{thing_ref}' should be checked in Cross Module Check stage.")]
    PropagateToCrossCheck {
        thing_ref: EcsThingRef,
    },
    #[error("Multiple errors occurred during checking:\n{}",
        .errors.iter().map(|e| format!("  - {}", e)).collect::<Vec<_>>().join("\n"))]
    Multiple {
        errors: Vec<InModuleCheckError>,
    }
}
impl InModuleCheckError {
    pub fn raise_already_defined(thing_ref: EcsThingRef) -> Self {
        Self::AlreadyDefined { thing_ref }
    }
    pub fn raise_define_external(thing_ref: EcsThingRef) -> Self {
        Self::DefineExternal { thing_ref }
    }
    pub fn raise_not_defined(thing_ref: EcsThingRef) -> Self {
        Self::NotDefined { thing_ref }
    }
    pub fn raise_propagate_to_cross_check(thing_ref: EcsThingRef) -> Self {
        Self::PropagateToCrossCheck { thing_ref }
    }
    pub fn raise_multiple(errors: Vec<Self>) -> Self {
        Self::Multiple { errors }
    }
    pub fn need_cross_module_check(&self) -> bool {
        matches!(self, InModuleCheckError::PropagateToCrossCheck { .. })
    }
}

pub struct InModuleChecker;
impl InModuleChecker {
    ///尝试注册一个组件（事件，实体原型，系统），如果已经注册过则返回错误
    pub fn try_register(thing: EcsThingRef, module_context: &mut ModuleCheckContext) -> Result<(),InModuleCheckError> {
        match thing {
            EcsThingRef::Component(component_ref) => {
                if component_ref.module_name.is_some() && component_ref.module_name != Some(module_context.name.clone()) {
                    //TODO: remove the clone
                    return Err(
                        InModuleCheckError::raise_define_external(EcsThingRef::Component(component_ref))
                    )
                }
                if module_context.defined_components.contains(&component_ref) {
                    return Err(
                        InModuleCheckError::raise_already_defined(EcsThingRef::Component(component_ref))
                    )
                }
                module_context.defined_components.insert(component_ref);
                Ok(())
            },
            EcsThingRef::Event(event_ref) => {
                if event_ref.module_name.is_some() && event_ref.module_name.as_ref().unwrap() != &module_context.name {
                    return Err(
                        InModuleCheckError::raise_define_external(EcsThingRef::Event(event_ref))
                    );
                }
                if module_context.defined_events.contains(&event_ref) {
                    return Err(
                        InModuleCheckError::raise_already_defined(EcsThingRef::Event(event_ref))
                    )
                }
                module_context.defined_events.insert(event_ref);
                Ok(())
            },
            EcsThingRef::EntityProto(entity_proto_ref) => {
                if entity_proto_ref.module_name.is_some() {
                    return Err(
                        InModuleCheckError::raise_define_external(EcsThingRef::EntityProto(entity_proto_ref))
                    );
                }
                if module_context.defined_entity_protos.contains(&entity_proto_ref) {
                    return Err(
                        InModuleCheckError::raise_already_defined(EcsThingRef::EntityProto(entity_proto_ref))
                    )
                }
                module_context.defined_entity_protos.insert(entity_proto_ref);
                Ok(())
            },
            EcsThingRef::System(system_ref) => {
                if system_ref.module_name.is_some() {
                    return Err(
                        InModuleCheckError::raise_define_external(EcsThingRef::System(system_ref))
                    );
                }
                if module_context.defined_systems.contains(&system_ref) {
                    return Err(
                        InModuleCheckError::raise_already_defined(EcsThingRef::System(system_ref))
                    )
                }
                module_context.defined_systems.insert(system_ref);
                Ok(())
            }
        }
    }
    ///检查模块内引用,应当等待所有组件，事件，实体原型，系统都被注册完后调用
    pub fn check_in_module_component_ref(component_ref: &ComponentRef, module_context: &ModuleCheckContext) -> Result<(), InModuleCheckError> {
        if component_ref.module_name.is_some() && component_ref.module_name.as_ref() != Some(&module_context.name) {
            return Err(
                InModuleCheckError::raise_propagate_to_cross_check(EcsThingRef::Component(component_ref.clone()))
            );
        }
        if !module_context.defined_components.contains(component_ref) {
            return Err(
                InModuleCheckError::raise_not_defined(EcsThingRef::Component(component_ref.clone()))
            )
        }
        Ok(())
    }
    pub fn check_in_module_event_ref(event_ref: &EventRef, module_context: &ModuleCheckContext) -> Result<(), InModuleCheckError> {
        if event_ref.module_name.is_some() && event_ref.module_name.as_ref() != Some(&module_context.name) {
            return Err(
                InModuleCheckError::raise_propagate_to_cross_check(EcsThingRef::Event(event_ref.clone()))
            );
        }
        if !module_context.defined_events.contains(event_ref) {
            return Err(
                InModuleCheckError::raise_not_defined(EcsThingRef::Event(event_ref.clone()))
            )
        }
        Ok(())
    }
    pub fn check_in_module_entity_proto_ref(entity_proto_ref: &EntityProtoRef, module_context: &ModuleCheckContext) -> Result<(), InModuleCheckError> {
        if entity_proto_ref.module_name.is_some() && entity_proto_ref.module_name.as_ref() != Some(&module_context.name) {
            return Err(
                InModuleCheckError::raise_propagate_to_cross_check(EcsThingRef::EntityProto(entity_proto_ref.clone()))
            );
        }
        if !module_context.defined_entity_protos.contains(entity_proto_ref) {
            return Err(
                InModuleCheckError::raise_not_defined(EcsThingRef::EntityProto(entity_proto_ref.clone()))
            );
        }
        Ok(())
    }
    pub fn check_in_module_system_ref(system_ref: &SystemRef, module_context: &ModuleCheckContext) -> Result<(), InModuleCheckError> {
        if system_ref.module_name.is_some() && system_ref.module_name.as_ref() != Some(&module_context.name) {
            return Err(
                InModuleCheckError::raise_propagate_to_cross_check(EcsThingRef::System(system_ref.clone()))
            );
        }
        if !module_context.defined_systems.contains(system_ref) {
            return Err(
                InModuleCheckError::raise_not_defined(EcsThingRef::System(system_ref.clone()))
            )
        }
        Ok(())
    }
}
pub trait InModuleCheckable {
    fn check_in_module(&self, module_context: &mut ModuleCheckContext) -> Result<(), InModuleCheckError> {
        Ok(())
    }
}
impl InModuleCheckable for SystemQuery {
    fn check_in_module(&self, module_context: &mut ModuleCheckContext) -> Result<(), InModuleCheckError> {
        let mut errors = Vec::new();

        for component_ref in self.component_constraint.chained_iter() {
            if let Err(e) = InModuleChecker::check_in_module_component_ref(component_ref, module_context) {
                errors.push(e);
            }
        }

        if !errors.is_empty() {
            return Err(InModuleCheckError::raise_multiple(errors));
        }

        Ok(())
    }
}
impl InModuleCheckable for SystemEventHandler {
    fn check_in_module(&self, module_context: &mut ModuleCheckContext) -> Result<(), InModuleCheckError> {
        InModuleChecker::check_in_module_event_ref(&self.watch_for, module_context)
    }
}
impl InModuleCheckable for EntityProto {
    fn check_in_module(&self, module_context: &mut ModuleCheckContext) -> Result<(), InModuleCheckError> {
        let mut errors = Vec::new();

        for component_ref in self.components.iter() {
            if let Err(e) = InModuleChecker::check_in_module_component_ref(component_ref, module_context) {
                errors.push(e);
            }
        }

        if !errors.is_empty() {
            return Err(InModuleCheckError::raise_multiple(errors));
        }

        Ok(())
    }
}