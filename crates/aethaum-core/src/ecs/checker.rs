use thiserror::Error;
use crate::ecs::checker::context::{ModuleCheckContext, ModuleCheckTree};
use crate::ecs::checker::cross_module::CrossModuleCheckError;
use crate::ecs::checker::in_module::InModuleCheckError;
use crate::ecs::checker::type_checker::TypeCheckError;

mod context;
mod type_checker;
mod in_module;
mod cross_module;
#[derive(Debug,Error)]
pub enum CheckerError {
    #[error("Type Error: {0}")]
    Type(TypeCheckError, CheckStage),
    #[error("In Module Check Error: {0}")]
    InModule(InModuleCheckError, CheckStage),
    #[error("Cross Module Check Error: {0}")]
    CrossModule(CrossModuleCheckError, CheckStage),
}

pub struct EcsChecker {
    check_stage: CheckStage,
    check_tree: ModuleCheckTree
}

#[derive(Debug)]
pub enum CheckStage {
    Type,
    InModule,
    CrossModule
}
impl EcsChecker {
    pub fn new() -> Self {
        Self {
            check_stage: CheckStage::Type,
            check_tree: ModuleCheckTree::new(),
        }
    }
    pub fn run_checks(&self) -> Result<(), CheckerError> {
        // 按阶段执行检查
        // 1. 类型检查
        // 2. 模块内检查
        // 3. 模块间检查
        todo!()
    }
}