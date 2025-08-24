use quote::quote;
use crate::code_generator::TranspileError;
use crate::toml_parser::parsed::Component;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

pub trait Transpile {
    fn transpile(&self) -> Result<TokenStream, TranspileError>;
    fn transpile_into(&self, output: &mut TokenStream) -> Result<(), TranspileError> {
        output.extend(self.transpile()?);
        Ok(())
    }
}
impl Transpile for Component {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let name = Ident::new(self.name.as_str(), Span::call_site());
        let fields = if let Some(fields) = &self.fields {
            fields.iter().map(|field| {
                let field_name = Ident::new(field.name.as_str(), Span::call_site());
                let field_type = field.type_spec.to_rust_type();
                quote! {
                    pub #field_name: #field_type,
                }
            }).collect::<Vec<_>>()
        } else {
            vec![]
        };
        // 生成 Default 实现（如果有默认值的话）
        let default_impl = if let Some(fields) = &self.fields {
            if fields.iter().any(|f| f.default_value.is_some()) {
                let default_fields = fields.iter().map(|field| {
                    let field_name = Ident::new(field.name.as_str(), Span::call_site());
                    if let Some(default_value) = &field.default_value {
                        // 将 TOML 值转换为 Rust 字面量
                        let default_literal = match default_value {
                            toml::Value::Boolean(b) => quote! { #b },
                            toml::Value::Integer(i) => quote! { #i },
                            toml::Value::Float(f) => quote! { #f },
                            toml::Value::String(s) => quote! { #s },
                            // 其他类型需要进一步处理
                            _ => quote! { Default::default() },
                        };
                        quote! { #field_name: #default_literal }
                    } else {
                        quote! { #field_name: Default::default() }
                    }
                }).collect::<Vec<_>>();

                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            Self {
                                #(#default_fields),*
                            }
                        }
                    }
                }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        };

        Ok(quote! {
            #[derive(Component)]
            pub struct #name {
                #(#fields)*
            }

            #default_impl
        })
    }
}
#[cfg(test)]
mod tests {
    use smart_string::SmartString;
    use crate::code_generator::utils::format_rust_code;
    use crate::toml_parser::parsed::{AethaumType, ComponentField, PrimitiveType};
    use super::*;
    #[test]
    fn test_transpile_component() {
        let component = Component {
            name: SmartString::from("TestComponent".to_string()),
            description: None,
            fields: Some(vec![
                ComponentField {
                    name: SmartString::from("test_field".to_string()),
                    type_spec: AethaumType::Primitive(PrimitiveType::Bool),
                    default_value: Some(toml::Value::Boolean(true)),
                    description: None,
                },
                ComponentField {
                    name: SmartString::from("test_field2".to_string()),
                    type_spec: AethaumType::Primitive(PrimitiveType::Int),
                    default_value: None,
                    description: None,
                },
            ]),
        };
        let transpiled = component.transpile().unwrap();
        let transpiled = format_rust_code(transpiled).unwrap();
        println!("{}", transpiled);
        let parsed_result = syn::parse_str::<syn::File>(&transpiled);
        assert!(parsed_result.is_ok(), "Generated code has syntax errors: {:?}", parsed_result.err());
    }
}