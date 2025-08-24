use proc_macro2::TokenStream;
use quote::quote;

pub fn trait_describe() -> TokenStream {
    quote! {
        pub trait Describe {
            fn describe(&self) -> &'static str {
                 ""
            }
            fn describe_field(&self, field_name: &str) -> &'static str {
                 ""
            }
        }
    }
}
//Reserved Events
pub fn event_aethaum_spawn_entity() -> TokenStream {
    quote! {
        #[derive(Event)]
        pub struct AethaumSpawnEntity {
            pub prototype_name: String,
            pub entity_response: Option<oneshot::Sender<Entity>>,
        }
    }
}