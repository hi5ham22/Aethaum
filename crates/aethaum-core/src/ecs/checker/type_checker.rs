use thiserror::Error;
use crate::ecs::module::EcsModule;
use crate::toml_parser::parsed::{AethaumType, ComponentField, EventField, PrimitiveType};

#[derive(Debug,Error)]
pub enum TypeCheckError {
    #[error("Type mismatch: expected {0}, got {1}")]
    TypeMismatch(AethaumType, toml::Value),
    #[error("Multiple errors occurred during checking:\n{}",
        .errors.iter().map(|e| format!("  - {}", e)).collect::<Vec<_>>().join("\n"))]
    Multiple {
        errors: Vec<TypeCheckError>,
    }
}
impl TypeCheckError {
    pub fn raise_type_mismatch(expected: AethaumType, got: toml::Value) -> Self {
        Self::TypeMismatch(expected, got)
    }
    pub fn raise_multiple(errors: Vec<Self>) -> Self {
        Self::Multiple { errors }
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
impl TypeCheckable for EcsModule {
    fn check_type(&self) -> Result<(), TypeCheckError> {
        let mut errors = Vec::new();
        if let Some(ref components) = self.components {
            for component in components {
                if let Some(fields) = &component.fields {
                    for field in fields {
                        if let Err(e) = field.check_type() {
                            errors.push(e);
                        }
                    }
                }
            }
        };
        if !errors.is_empty() {
            return Err(TypeCheckError::raise_multiple(errors));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::toml_parser::parsed::Component;
    use crate::toml_parser::raw::{RawComponentFile, RawTomlCodeFile};
    use super::*;
    #[test]
    fn test_type_check_passed() {
        let toml_file = r#"
            #在一个toml文件中可以定义多个组件，normal字段不会参与转译，作为注释和元信息提供
            [normal]
            tags = ["combat", "stats"]
            description = "战斗相关组件"

            # 健康组件
            [[components]]
            name = "Health"
            description = "实体健康值"

            [[components.fields]]
            name = "value"
            type = "float"
            default = 100.0
            description = "当前健康值"

            [[components.fields]]
            name = "max_value"
            type = "float"
            default = 100.0
            description = "最大健康值"

            # 位置组件
            [[components]]
            name = "Position"
            description = "实体位置"

            [[components.fields]]
            name = "x"
            type = "float"
            default = 0.0
            description = "X坐标"

            [[components.fields]]
            name = "y"
            type = "float"
            default = 0.0
            description = "Y坐标"
        "#;
        let components = toml::from_str::<RawComponentFile>(toml_file).unwrap().into_pieces();
        let components = components.into_iter()
            .map(|c| c.into())
            .collect::<Vec<Component>>();
        for comp in components {
            for fields in comp.fields.unwrap() {
                fields.check_type().unwrap();
            }
        }
    }
    #[test]
    fn test_type_check_error() {
        let toml_file = r#"
            #在一个toml文件中可以定义多个组件，normal字段不会参与转译，作为注释和元信息提供
            [normal]
            tags = ["combat", "stats"]
            description = "战斗相关组件"

            # 健康组件
            [[components]]
            name = "Health"
            description = "实体健康值"

            [[components.fields]]
            name = "value"
            type = "float"
            default = "a"
            description = "当前健康值"

            [[components.fields]]
            name = "max_value"
            type = "float"
            default = 100.0
            description = "最大健康值"

            # 位置组件
            [[components]]
            name = "Position"
            description = "实体位置"

            [[components.fields]]
            name = "x"
            type = "float"
            default = "a"
            description = "X坐标"

            [[components.fields]]
            name = "y"
            type = "float"
            default = 0.0
            description = "Y坐标"
        "#;
        let components = toml::from_str::<RawComponentFile>(toml_file).unwrap().into_pieces();
        let components = components.into_iter()
            .map(|c| c.into())
            .collect::<Vec<Component>>();
        let mut errors = Vec::new();
        for comp in components {
            for fields in comp.fields.unwrap() {
                if let Err(e) = fields.check_type() {
                    errors.push(e);
                }
            }
        }
        assert_eq!(errors.len(), 2);
    }
}
