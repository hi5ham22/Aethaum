use quote::quote;
use crate::code_generator::TranspileError;
use crate::toml_parser::parsed::{Component, Describable, EntityProto, Event, Field, System, SystemQuery};
use proc_macro2::{Span, TokenStream};
use syn::Ident;

pub trait Transpile {
    fn transpile(&self) -> Result<TokenStream, TranspileError>;
    fn transpile_into(&self, output: &mut TokenStream) -> Result<(), TranspileError> {
        output.extend(self.transpile()?);
        Ok(())
    }
}
fn transpile_fields<T, FieldIter>(fields: FieldIter) -> impl Iterator<Item = TokenStream>
where
    T: Field,
    FieldIter: IntoIterator<Item = T>,
{
    fields.into_iter().map(|field| {
        let field_name = field.name_as_rust_ident();
        let field_type = field.type_as_rust_ident();
        quote! {
            pub #field_name: #field_type,
        }
    })
}
fn transpile_descriptions<T: Describable>(to_transpile: &T, name: &str) -> TokenStream {
    let struct_desc = to_transpile.description()
        .map(|d| {
            quote! { #d }
        })
        .unwrap_or_else(|| quote! { "" });

    let field_desc_impl = if let Some(fields) = to_transpile.field_description() {
        let field_matches = fields.map(|(field_name, desc)| {
            quote! {
                stringify!(#field_name) => #desc,
            }
        }).collect::<Vec<_>>();

        if !field_matches.is_empty() {
            quote! {
                    match field_name {
                        #(#field_matches)*
                        _ => "",
                    }
                }
        } else {
            quote! { "" }
        }
    } else {
        quote! { "" }
    };

    let name = Ident::new(name, Span::call_site());

    quote! {
        impl Describe for #name {
            fn description(&self) -> &'static str {
                #struct_desc
            }

            fn describe_field(&self, field_name: &str) -> &'static str {
                #field_desc_impl
            }
        }
    }
}
impl Transpile for Component {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let name = Ident::new(self.name.as_str(), Span::call_site());
        let fields = if let Some(fields) = &self.fields {
            transpile_fields(fields).collect()
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
        //生成Describe trait
        let description_impl = transpile_descriptions(self,self.name.as_str());

        Ok(quote! {
            #[derive(Component)]
            pub struct #name {
                #(#fields)*
            }

            #default_impl

            #description_impl
        })
    }
}
impl Transpile for Event {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let name = Ident::new(self.name.as_str(), Span::call_site());
        let fields = if let Some( fields) = self.fields.as_ref() {
            transpile_fields(fields).collect()
        } else {
            vec![]
        };
        let description_impl = transpile_descriptions(self, self.name.as_str());

        Ok(
            quote! {
                #[derive(Event)]
                pub struct #name {
                    #(#fields)*
                }

                #description_impl
            }
        )
    }
}
impl Transpile for EntityProto {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let name = Ident::new(self.name.as_str(), Span::call_site());
        let bundle_name = Ident::new(&format!("{}Bundle", self.name), Span::call_site());
        let spawn_system_name = Ident::new(&format!("spawn_{}_system", self.name.to_lowercase()), Span::call_site());

        // 生成 Bundle 字段
        let bundle_fields = self.components.iter().map(|component_ref| {
            let component_name = Ident::new(component_ref.name.as_str(), Span::call_site());
            quote! {
                pub #component_name: #component_name,
            }
        }).collect::<Vec<_>>();

        // 生成描述实现
        let description_impl = transpile_descriptions(self, self.name.as_str());

        Ok(quote! {
            #[derive(Bundle, Default)]
            pub struct #bundle_name {
                #(#bundle_fields)*
            }

            pub struct #name;

            impl #name {
                pub fn bundle() -> #bundle_name {
                    #bundle_name::default()
                }

                pub fn spawn(commands: &mut Commands) -> Entity {
                    commands.spawn(Self::bundle()).id()
                }
            }

            // 为这个原型生成对应的处理系统
            pub fn #spawn_system_name(
                mut events: EventReader<AethaumSpawnEntity>,
                mut commands: Commands,
            ) {
                for event in events.read() {
                    if event.prototype_name == stringify!(#name) {
                        let entity = #name::spawn(&mut commands);
                        if let Some(response) = &event.entity_response {
                            let _ = response.send(entity);
                        }
                    }
                }
            }

            #description_impl
        })
    }
}
impl Transpile for SystemQuery {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let mut filters = {
            let mut filters = Vec::new();

            // 处理 With 过滤器（包含的组件）
            if let Some(include_components) = self.component_constraint.get_include() {
                for component_ref in include_components {
                    let component_name = Ident::new(&component_ref.as_path_str(), Span::call_site());
                    filters.push(quote! { With<#component_name> });
                }
            }

            // 处理 Without 过滤器（排除的组件）
            if let Some(exclude_components) = self.component_constraint.get_exclude() {
                for component_ref in exclude_components {
                    let component_name = Ident::new(&component_ref.as_path_str(), Span::call_site());
                    filters.push(quote! { Without<#component_name> });
                }
            }
            filters
        };
        match filters.len() {
            0 => Ok(quote! { Query<Entity> }),
            1 => {
                let filter = filters.pop().unwrap();
                Ok(quote! { Query<Entity, #filter> })
            },
            _ => {
                Ok(quote! {
                    Query<Entity, (#(#filters),*)>
                })
            }
        }
    }
}
impl Transpile for System {
    fn transpile(&self) -> Result<TokenStream, TranspileError> {
        let name = Ident::new(self.normal.name.as_str(), Span::call_site());
        let queries = {
            self.queries.iter()
                .map(|query| {
                    query.transpile().unwrap() //ROBUST: the Transpile for Query will always succeed
                })
                .collect::<Vec<_>>()
        };
        todo!("transpile System")
    }
}


#[cfg(test)]
mod tests {
    use smart_string::SmartString;
    use crate::code_generator::utils::format_rust_code;
    use crate::toml_parser::parsed::{AethaumType, ComponentField, ComponentRef, EventField, PrimitiveType};
    use super::*;
    #[test]
    fn test_transpile_component() {
        let component = Component {
            name: SmartString::from("TestComponent".to_string()),
            description: Some(SmartString::from("This is a test component".to_string())),
            fields: Some(vec![
                ComponentField {
                    name: SmartString::from("test_field".to_string()),
                    type_spec: AethaumType::Primitive(PrimitiveType::Bool),
                    default_value: Some(toml::Value::Boolean(true)),
                    description: Some(SmartString::from("This is a test field".to_string())),
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
    #[test]
    fn test_transpile_event() {
        let event = Event {
            name: SmartString::from("click"),
            description: Some("Click event".into()),
            fields: Option::from(vec![
                EventField {
                    name: SmartString::from("target"),
                    description: Some("The element that was clicked".into()),
                    type_spec: AethaumType::Primitive(PrimitiveType::Str),
                },
                EventField {
                    name: SmartString::from("value"),
                    description: None,
                    type_spec: AethaumType::Primitive(PrimitiveType::Int),
                },
            ]),
        };
        let transpiled = event.transpile().unwrap();
        println!("{}", transpiled);
        let transpiled = format_rust_code(transpiled).unwrap();
        println!("{}", transpiled);
        let parsed_result = syn::parse_str::<syn::File>(&transpiled);
        assert!(parsed_result.is_ok(), "Generated code has syntax errors: {:?}", parsed_result.err());
    }
    #[test]
    fn test_transpile_entity_protos() {
        let event = EntityProto {
            name: "TestEntity".into(),
            description: Some("This is a test entity".into()),
            components: vec![
                ComponentRef::new(None::<&str>, "position"),
                ComponentRef::new(Some("TestComponent"), "test_component")
            ]
        };
        let transpiled = event.transpile().unwrap();
        println!("{}", transpiled);
        let transpiled = format_rust_code(transpiled).unwrap();
        println!("{}", transpiled);
        let parsed_result = syn::parse_str::<syn::File>(&transpiled);
        assert!(parsed_result.is_ok(), "Generated code has syntax errors: {:?}", parsed_result.err());
    }
}
