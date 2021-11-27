extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{ItemFn, Meta, NestedMeta, Token};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

struct FnProcessor {
    func: ItemFn,
    config: Config,
    fmt_args: FmtArgs,
}

impl FnProcessor {
    fn new(func: ItemFn, config: Config) -> Self {
        Self { func, config, fmt_args: FmtArgs::default() }
    }

    fn parse_fmt_args(mut self) -> Result<Self, syn::Error> {
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

    fn insert_log_statements(mut self) -> Self {
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

    fn into_tokens(self) -> TokenStream2 {
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

#[derive(Default, Debug)]
struct FmtArgs {
    args: Punctuated<FmtArg, Token![,]>,
    takes_fn_name: bool,
    takes_return_value: bool,
}

#[derive(Debug)]
struct FmtArg(Ident);

impl FmtArg {
    fn from_str(s: &str) -> Self {
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

#[derive(Debug)]
struct Config {
    log_level: log::Level,
    fmt_string: syn::LitStr,
}

impl Config {
    fn new(mut args: impl Iterator<Item=NestedMeta>) -> Result<Self, syn::Error> {
        let first_arg = args
            .next()
            .ok_or_else(|| syn::Error::new(
                Span::call_site(),
                "No arguments supplied\n\
                help: log requires a log level and a fmt string\n\
                example: #[log(info, \"{fn} returned {return}\")]"
            ))?;

        let log_level = match first_arg {
            NestedMeta::Meta(Meta::Path(ref p)) if p.get_ident().is_some() => {
                let level = p.get_ident().unwrap().to_string();
                log::Level::from_str(&level)
            },
            m => return Err(syn::Error::new(
                m.span(),
                "Expected log level\n\
                note: The log level should be provided as an identifier without quotes\n\
                example: #[log(info, \"{fn} returned {return}\")]",
            )),
        };

        let log_level = match log_level {
            Ok(level) => level,
            Err(err) => return Err(syn::Error::new(
                first_arg.span(),
                format_args!(
                    "Failed to parse log level\n\
                    error: {}\n\
                    note: The log level should be provided as an identifier without quotes\n\
                    example: #[log(info, \"{{fn}} returned {{return}}\")]",
                    err
                )
            )),
        };

        Self::with_log_level(args, log_level)
    }

    fn with_log_level(mut args: impl Iterator<Item=NestedMeta>, log_level: log::Level) -> Result<Self, syn::Error> {
        let fmt_string = args
            .next()
            .ok_or_else(|| syn::Error::new(
                Span::call_site(),
                "No format string supplied\n\
                help: log requires a fmt string that describes the log output\n\
                example: #[log(info, \"{fn} returned {return}\")]"
            ))?;

        let fmt_string = match fmt_string {
            NestedMeta::Lit(syn::Lit::Str(s)) => s,
            m => return Err(syn::Error::new(
                m.span(),
                "Expected a format string literal\n\
                example: #[log(info, \"{fn} returned {return}\")]",
            )),
        };

        if let Some(arg) = args.next() {
            return Err(syn::Error::new(
                arg.span(),
                "Unexpected argument"
            ));
        }

        Ok(Self { log_level, fmt_string })
    }
}

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}


#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as ItemFn);

    let res = Config::new(args.into_iter())
        .map(|config| FnProcessor::new(input, config))
        .and_then(FnProcessor::parse_fmt_args)
        .map(FnProcessor::insert_log_statements)
        .map(FnProcessor::into_tokens);

    match res {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
