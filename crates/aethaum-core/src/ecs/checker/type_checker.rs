use thiserror::Error;
use crate::toml_parser::parsed::{AethaumType, ComponentField, EventField, PrimitiveType};

#[derive(Debug,Error)]
pub enum TypeCheckError {
    #[error("Type mismatch: expected {0}, got {1}")]
    TypeMismatch(AethaumType, toml::Value),
}
impl TypeCheckError {
    pub fn raise_type_mismatch(expected: AethaumType, got: toml::Value) -> Self {
        Self::TypeMismatch(expected, got)
    }
}

pub struct TypeChecker;
impl TypeChecker {
    pub fn check_type_value_match(type_spec: &AethaumType, default_value: &Option<toml::Value>) -> Result<(), TypeCheckError> {
        match type_spec {
            AethaumType::Primitive(primitive_type) => {
                match (primitive_type, default_value) {
                    (PrimitiveType::Bool, Some(toml::Value::Boolean(_))) => Ok(()),
                    (PrimitiveType::Int, Some(toml::Value::Integer(_))) => Ok(()),
                    (PrimitiveType::Float, Some(toml::Value::Float(_))) => Ok(()),
                    (PrimitiveType::Str, Some(toml::Value::String(_))) => Ok(()),
                    (_, None) => Ok(()),
                    _ => return Err(TypeCheckError::raise_type_mismatch(type_spec.clone(), default_value.as_ref().unwrap().clone())),
                    //ROBUST: None is early returned
                }
            }
            AethaumType::Custom(_) => unreachable!("Custom type not supported in this version")
        }
    }
}
pub trait TypeCheckable {
    fn check_type(&self) -> Result<(), TypeCheckError> {
        Ok(())
    }
}
impl TypeCheckable for ComponentField {
    fn check_type(&self) -> Result<(), TypeCheckError> {
        TypeChecker::check_type_value_match(&self.type_spec, &self.default_value)
    }
}