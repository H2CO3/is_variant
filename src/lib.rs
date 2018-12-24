#![crate_type = "proc-macro"]

#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
extern crate heck;

mod generics;
mod error;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{ DeriveInput, Data, Ident, Fields };
use heck::SnakeCase;
use error::{ Error, Result };
use generics::*;

/// The top-level entry point of this proc-macro. Only here to be exported
/// and to handle `Result::Err` return values by `panic!()`ing.
#[proc_macro_derive(IsVariant, attributes(is_variant))]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    impl_is_variant(input).unwrap_or_else(|error| panic!("{}", error))
}

/// Implements `is_XXX` methods for an `enum`.
fn impl_is_variant(input: TokenStream) -> Result<TokenStream> {
    let parsed_ast: DeriveInput = syn::parse(input)?;
    let ty = parsed_ast.ident;
    let enum_ast = match parsed_ast.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new("only `enum`s have variants to check")),
    };
    let Components {
        impl_params,
        type_params,
        where_bounds,
    } = parsed_ast.generics.into_pruned_components();

    let functions = enum_ast.variants.into_iter().map(|variant| {
        let variant_name = variant.ident;
        let fn_name_str = format!("is_{}", variant_name).to_snake_case();
        let fn_name = Ident::new(&fn_name_str, Span::call_site());
        let ignore_fields = match variant.fields {
            Fields::Unit => Default::default(),
            Fields::Named(_) => quote!({ .. }),
            Fields::Unnamed(_) => quote!{ (..) },
        };

        quote! {
            pub fn #fn_name(&self) -> bool {
                if let &#ty::#variant_name #ignore_fields = self {
                    true
                } else {
                    false
                }
            }
        }
    });

    let generated = quote! {
        impl<#impl_params> #ty<#type_params> #where_bounds {
            #(#functions)*
        }
    };

    Ok(generated.into())
}
