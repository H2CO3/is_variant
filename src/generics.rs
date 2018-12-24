//! Parse and extend generic bounds.

use syn::{
    Generics, GenericParam, WhereClause,
    punctuated::Punctuated,
    token::Comma,
};
use proc_macro2::TokenStream;
use quote::ToTokens;

/// Separated components of generic arguments, suitable for use in an `impl`.
#[derive(Debug, Clone, Default)]
pub struct Components {
    /// Generic parameters of the `impl` itself (names only).
    pub impl_params: TokenStream,
    /// Generic parameters of the type that we are `impl`ementing.
    pub type_params: TokenStream,
    /// The essence of the `where` clause.
    pub where_bounds: TokenStream,
}

/// Helper for getting the interesting parts of generics for an `impl`
/// (e.g. without the default values of the generic parameters)
#[allow(clippy::stutter)]
pub trait GenericsExt: Sized {
    /// The first return value is the `impl` generic parameter list on the left.
    /// The second one is just the list of names of type and lifetime arguments.
    /// The third one is the augmented `where` clause -- the whole point.
    fn into_pruned_components(self) -> Components;
}

impl GenericsExt for Generics {
    fn into_pruned_components(self) -> Components {
        if self.lt_token.is_none() || self.gt_token.is_none() {
            return Default::default(); // no type parameters
        }

        let self_params: Vec<_> = self.params
            .iter()
            .cloned()
            .map(|param| match param {
                GenericParam::Type(ty) => ty.ident.into_token_stream(),
                GenericParam::Lifetime(lt) => lt.lifetime.into_token_stream(),
                GenericParam::Const(cst) => cst.ident.into_token_stream(),
            })
            .collect();

        let where_clause = self.where_clause.unwrap_or(WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });

        let params_sans_defaults: Punctuated<GenericParam, Comma> = self
            .params
            .into_iter()
            .map(|param| match param {
                GenericParam::Lifetime(_) => param,
                GenericParam::Type(mut param) => {
                    param.eq_token.take();
                    param.default.take();
                    GenericParam::Type(param)
                }
                GenericParam::Const(mut param) => {
                    param.eq_token.take();
                    param.default.take();
                    GenericParam::Const(param)
                }
            })
            .collect();

        Components {
            impl_params: params_sans_defaults.into_token_stream(),
            type_params: quote!{ #(#self_params),* },
            where_bounds: where_clause.into_token_stream(),
        }
    }
}
