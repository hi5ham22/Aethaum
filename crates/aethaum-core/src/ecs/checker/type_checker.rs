use crate::toml_parser::parsed::{AethaumType, PrimitiveType};

pub struct TypeChecker;
impl TypeChecker {
    pub fn check_type_value_match(type_spec: &AethaumType, default_value: &Option<toml::Value>) -> anyhow::Result<()> {
        match type_spec {
            AethaumType::Primitive(primitive_type) => {
                match (primitive_type, default_value) {
                    (PrimitiveType::Bool, Some(toml::Value::Boolean(_))) => Ok(()),
                    (PrimitiveType::Int, Some(toml::Value::Integer(_))) => Ok(()),
                    (PrimitiveType::Float, Some(toml::Value::Float(_))) => Ok(()),
                    (PrimitiveType::Str, Some(toml::Value::String(_))) => Ok(()),
                    (_, None) => Ok(()),
                    _ => anyhow::bail!("Type mismatch: {}", type_spec),
                }
            }
            AethaumType::Custom(_) => unreachable!("Custom type not supported in this version")
        }
    }
}
pub trait TypeCheckable {
    fn check_type(&self) -> anyhow::Result<()> {
        Ok(())
    }
}