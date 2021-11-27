use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::ItemFn;
use syn::parse::{Parse, Parser};
use syn::spanned::Spanned;

use crate::config::Config;
use crate::fmt_args::{FmtArg, FmtArgs};

pub struct FnProcessor {
    func: ItemFn,
    config: Config,
    fmt_args: FmtArgs,
}

impl FnProcessor {
    pub fn new(func: ItemFn, config: Config) -> Self {
        Self { func, config, fmt_args: FmtArgs::default() }
    }

    pub fn parse_fmt_args(mut self) -> Result<Self, syn::Error> {
        let mut in_arg = false;
        let mut skip = false;
        let mut arg = String::new();

        let fmt_string = self.config.fmt_string.value();
        let mut chars = fmt_string.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                '{' if chars.peek() == Some(&'{') => skip = true,
                '{' if skip => {},
                '{' if in_arg => { /* we let the compiler throw the error */ },
                '{' => in_arg = true,

                '}' if chars.peek() == Some(&'}') => {},
                '}' if !in_arg => { /* we let the compiler throw the error */ },
                '}' => {
                    in_arg = false;
                    skip = false;
                    match &*std::mem::take(&mut arg) {
                        "fn" => self.fmt_args.takes_fn_name = true,
                        "return" => self.fmt_args.takes_return_value = true,
                        arg => self.fmt_args.args.push(FmtArg::from_str(arg)),
                    }
                }

                ':' if in_arg => skip = true,

                char if in_arg && !skip => arg.push(char),
                _ => {},
            }
        }

        Ok(self)
    }

    pub fn insert_log_statements(mut self) -> Self {
        let body = self.func.block;
        let ret_ident = Ident::new("__FUNC_RET__", body.span());

        let log_level = syn::Ident::new(
            &capitalize(self.config.log_level.as_str()),
            Span::call_site(),
        );
        let fmt_string = &self.config.fmt_string;
        let fn_arg = self.fmt_args.takes_fn_name.then(|| {
            let fn_name = self.func.sig.ident.to_string();
            quote! { fn = #fn_name, }
        });
        let ret_arg = self.fmt_args.takes_return_value.then(|| {
            quote! { return = #ret_ident, }
        });
        let fmt_args = &self.fmt_args.args;

        let body = quote! {{
            #[allow(non_snake_case, used_underscore_binding)]
            let #ret_ident = #body;

            ::log::log!(
                ::log::Level::#log_level,
                #fmt_string,
                #fn_arg
                #ret_arg
                #fmt_args
            );

            #ret_ident
        }};

        self.func.block = Box::new(syn::Block::parse.parse(body.into()).unwrap());
        self
    }

    pub fn into_tokens(self) -> TokenStream2 {
        self.func.into_token_stream()
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    let first = match c.next() {
        None => return String::new(),
        Some(c) => c,
    };

    first
        .to_uppercase()
        .chain(
            c
                .map(char::to_lowercase)
                .flatten()
        )
        .collect::<String>()
}
