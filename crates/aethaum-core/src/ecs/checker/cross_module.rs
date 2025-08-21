use smart_string::SmartString;
use crate::ecs::checker::context::{ModuleCheckContext, ModuleCheckTree};
use crate::ecs::module::{EcsThingRef, ModulePath};

pub struct CrossModuleChecker;
impl CrossModuleChecker {
    ///尝试注册一个模块，如果已经注册过，则返回错误(命名冲突检查)
    pub fn register_module(name: &SmartString, context: ModuleCheckContext, tree: &mut ModuleCheckTree) -> anyhow::Result<()> {
        if tree.get_tree().contains_key(name) {
            anyhow::bail!("Module {} is already registered", name);
        }
        tree.get_tree_mut().insert(name.clone(), context);
        Ok(())
    }
    ///检查跨模块引用，应当在所有module都被注册完后调用
    pub fn check_cross_module_ref(ref_name: &ModulePath, tree: &ModuleCheckTree) -> anyhow::Result<()> {
        let module_context = tree.get_module_context(ref_name.module_name)
            .ok_or_else(|| anyhow::anyhow!("Module {} is not registered", ref_name.module_name))?;
        match ref_name.thing_name {
            EcsThingRef::Component(comp_ref) => {
                if !module_context.defined_components.contains(comp_ref) {
                    anyhow::bail!("Component \"{}\" is not defined in module \"{}\"", ref_name.thing_name, ref_name.module_name);
                }
            }
            EcsThingRef::Event(event_ref) => {
                if !module_context.defined_events.contains(event_ref) {
                    anyhow::bail!("Event \"{}\" is not defined in module \"{}\"", ref_name.thing_name, ref_name.module_name);
                }
            }
            EcsThingRef::EntityProto(entity_proto_ref) => {
                if !module_context.defined_entity_protos.contains(entity_proto_ref) {
                    anyhow::bail!("Entity proto \"{}\" is not defined in module \"{}\"", ref_name.thing_name, ref_name.module_name);
                }
            }
            EcsThingRef::System(system_ref) => {
                if !module_context.defined_systems.contains(system_ref) {
                    anyhow::bail!("System \"{}\" is not defined in module \"{}\"", ref_name.thing_name, ref_name.module_name);
                }
            }
        }
        Ok(())
    }
}
pub trait CrossModuleCheckable {
    fn check_cross_module(&self, module_context: &ModuleCheckContext) -> anyhow::Result<()> {
        Ok(())
    }
}