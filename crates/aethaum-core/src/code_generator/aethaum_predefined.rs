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