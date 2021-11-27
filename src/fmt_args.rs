use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{TokenStreamExt, ToTokens};
use syn::punctuated::Punctuated;

#[derive(Default)]
pub struct FmtArgs {
    pub args: Punctuated<FmtArg, syn::Token![,]>,
    pub takes_fn_name: bool,
    pub takes_return_value: bool,
}

pub struct FmtArg(Ident);

impl FmtArg {
    pub fn from_str(s: &str) -> Self {
        Self(Ident::new(s, Span::call_site()))
    }
}

impl ToTokens for FmtArg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append(self.0.clone());
        tokens.append(proc_macro2::Punct::new('=', proc_macro2::Spacing::Alone));
        tokens.append(self.0.clone());
    }
}
