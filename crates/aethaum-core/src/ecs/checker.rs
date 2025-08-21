use crate::ecs::checker::context::ModuleCheckContext;

mod context;
mod type_checker;
mod in_module;
mod cross_module;

pub struct EcsChecker {

}
pub enum CheckStage {
    Type,
    InModule,
    CrossModule
}