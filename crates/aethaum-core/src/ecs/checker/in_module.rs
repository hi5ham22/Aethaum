use smart_string::SmartString;
use crate::ecs::checker::context::{ECSThing, ModuleCheckContext, ModuleCheckTree};
use crate::ecs::module::ModulePath;
use crate::toml_parser::parsed::{ComponentRef, EntityProtoRef, EventRef, SystemRef};

pub struct InModuleChecker;
impl InModuleChecker {
    ///尝试注册一个组件（事件，实体原型，系统），如果已经注册过则返回错误
    pub fn try_register(name: SmartString, thing_type: ECSThing, module_context: &mut ModuleCheckContext) -> anyhow::Result<()> {
        match thing_type {
            ECSThing::Component => {
                let component_ref = ComponentRef::new(name.as_str());
                if module_context.defined_components.contains(&component_ref) {
                    anyhow::bail!("Component {} is already defined", name);
                }
                module_context.defined_components.insert(component_ref);
                Ok(())
            },
            ECSThing::Event => {
                let event_ref = EventRef::new(name.as_str());
                if module_context.defined_events.contains(&event_ref) {
                    anyhow::bail!("Event {} is already defined", name);
                }
                module_context.defined_events.insert(event_ref);
                Ok(())
            },
            ECSThing::EntityProto => {
                let entity_proto_ref = EntityProtoRef::new(name.as_str());
                if module_context.defined_entity_protos.contains(&entity_proto_ref) {
                    anyhow::bail!("EntityProto {} is already defined", name);
                }
                module_context.defined_entity_protos.insert(entity_proto_ref);
                Ok(())
            }
            ECSThing::System => {
                let system_ref = SystemRef::new(name.as_str());
                if module_context.defined_systems.contains(&system_ref) {
                    anyhow::bail!("System {} is already defined", name);
                }
                module_context.defined_systems.insert(system_ref);
                Ok(())
            }
        }
    }
    ///检查模块内引用,应当等待所有组件，事件，实体原型，系统都被注册完后调用
    pub fn check_in_module_ref(ref_name: &ModulePath, tree: &ModuleCheckContext) {
        todo!("the parameter list should be considered again")
    }
}
pub trait InModuleCheckable {
    fn check_in_module(&self, module_context: &mut ModuleCheckContext) -> anyhow::Result<()> {
        Ok(())
    }
}