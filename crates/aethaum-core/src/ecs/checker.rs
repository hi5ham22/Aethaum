use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::ecs::checker::context::{ModuleCheckContext, ModuleCheckTree};
use crate::ecs::checker::cross_module::{CrossModuleCheckError, CrossModuleCheckable};
use crate::ecs::checker::in_module::{InModuleCheckError, InModuleCheckable};
use crate::ecs::checker::type_checker::{TypeCheckError, TypeCheckable};
use crate::ecs::module::{AethaumProject, EcsModule, EcsThingRef};

mod context;
mod type_checker;
mod in_module;
mod cross_module;

#[derive(Debug)]
pub enum CheckStage {
    Type,
    InModule,
    CrossModule
}

#[derive(Debug, Error)]
pub enum CheckerError {
    #[error("Type Error: {0}")]
    Type(#[from] TypeCheckError),
    #[error("Unfiltered In Module Check Error: {0}")]
    InModule(InModuleCheckError, Box<ModuleCheckContext>),
    #[error("In Module Check Error: {0}")]
    FilteredInModule(InModuleCheckError),
    #[error("Cross Module Check Error: {0}")]
    CrossModule(#[from] CrossModuleCheckError),
    #[error("Multiple errors occurred during checking:\n{}",
        .errors.iter().map(|e| format!("  - {}", e)).collect::<Vec<_>>().join("\n"))]
    Multiple {
        errors: Vec<CheckerError>,
    }
}
impl CheckerError {
    pub fn raise_multiple(errors: Vec<CheckerError>) -> Self {
        Self::Multiple {
            errors,
        }
    }
}
pub struct CheckedEcs {
    modules: Vec<EcsModule>,
}
impl<I> From<I> for CheckedEcs
where
    I: Iterator<Item = EcsModule>
{
    fn from(modules: I) -> Self {
        Self {
            modules: modules.collect(),
        }
    }
}

pub struct SingleEcsModuleChecker;


impl SingleEcsModuleChecker {
    pub fn run_checks(module: &EcsModule, project_root: PathBuf) -> Result<ModuleCheckContext, CheckerError> {
        // 按阶段执行检查
        // 1. 类型检查
        // 2. 模块内检查
        let mut module_check_context = ModuleCheckContext::new(module.name.clone(), project_root);
        module.check_type()?;
        let in_module_check_res = module.check_in_module(&mut module_check_context);
        if let Err(err) = in_module_check_res {
            return Err(CheckerError::InModule(err, Box::new(module_check_context)));
        }
        Ok(module_check_context)
    }
}
pub struct CrossEcsModuleChecker;
impl CrossEcsModuleChecker {
    pub fn run_checks<'a>(ref_to_check: impl IntoIterator<Item = &'a EcsThingRef>, module_tree: &ModuleCheckTree) -> Result<(), CrossModuleCheckError> {
        //3.模块间检查
        let mut errors = Vec::new();
        for thing_ref in ref_to_check {
            match EcsThingRef::check_cross_module(thing_ref, module_tree) {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.pop().unwrap())
        }else {
            Err(CrossModuleCheckError::raise_multiple(errors))
        }
    }
}
pub struct AethaumChecker;
impl AethaumChecker {
    pub fn run_check(project: AethaumProject) -> Result<AethaumProject, CheckerError> {
        let mut errors = Vec::new();
        let mut module_contexts = Vec::new();
        for module in project.module_tree.get_modules() {
            match SingleEcsModuleChecker::run_checks(module,project.root.clone()) {
                Ok(module_context) => module_contexts.push(module_context),
                Err(err) => errors.push(err),
            }
        }
        let (filtered_module_contexts, propagated_checks) = Self::extract_propagated_check(errors)?;
        module_contexts.extend(filtered_module_contexts);
        let module_tree = ModuleCheckTree::new().with_module_contexts(module_contexts);
        CrossEcsModuleChecker::run_checks(propagated_checks.iter(), &module_tree)?;
        Ok(project)
    }
    fn extract_propagated_check(errors: Vec<CheckerError>) -> Result<(Vec<ModuleCheckContext>, Vec<EcsThingRef>), CheckerError> {
        let mut true_errors = Vec::new();
        let mut module_contexts = Vec::new();
        let mut propagated_checks = Vec::new();
        for error in errors {
            match error {
                CheckerError::InModule(err, module_context) => {
                    module_contexts.push(*module_context);
                    match err {
                        InModuleCheckError::PropagateToCrossCheck {thing_ref} => {
                            propagated_checks.push(thing_ref);

                        },
                        InModuleCheckError::Multiple { errors} => {
                            for err in errors {
                                match err {
                                    InModuleCheckError::PropagateToCrossCheck {thing_ref} => {
                                        propagated_checks.push(thing_ref);
                                    },
                                    _ => true_errors.push(CheckerError::FilteredInModule(err)),
                                }
                            }
                        },
                        _ => true_errors.push(CheckerError::FilteredInModule(err)),
                    }
                }
                _ => true_errors.push(error),
            }
        }
        if true_errors.is_empty() {
            Ok((module_contexts, propagated_checks))
        } else if true_errors.len() == 1 {
            Err(true_errors.pop().unwrap())
        } else {
            Err(CheckerError::raise_multiple(true_errors))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::ecs::loader::ProjectLoader;
    use super::*;
    #[test]
    fn test_project_checker() {
        let project = ProjectLoader::new(r#"D:\Aethaum\test_project"#.into()).load().unwrap();
        AethaumChecker::run_check(project).unwrap();
    }
}