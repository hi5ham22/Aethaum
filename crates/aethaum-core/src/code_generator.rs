mod transpile;
mod lua_binding;
mod aethaum_predefined;
mod utils;

use bevy_ecs::event::Events;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum TranspileError {
    #[error("Error to write generated code, {0}")]
    WriteError(#[from] core::fmt::Error),
    #[error("Error to format generated code, {0}")]
    FormatError(#[from] syn::Error)
}