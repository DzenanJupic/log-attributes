extern crate proc_macro;

use proc_macro::TokenStream;

use config::Config;
use fn_processor::FnProcessor;

mod fn_processor;
mod fmt_args;
mod config;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}

fn log_with_config(input: syn::ItemFn, config: Result<Config, syn::Error>) -> TokenStream {
    let res = config
        .map(|config| FnProcessor::new(input, config))
        .and_then(FnProcessor::parse_fmt_args)
        .map(FnProcessor::insert_log_statements)
        .map(FnProcessor::into_tokens);

    match res {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn log_with_level(args: TokenStream, input: TokenStream, log_level: log::Level) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    log_with_config(
        input,
        Config::with_log_level(args.into_iter(), log_level),
    )
}

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    log_with_config(
        input,
        Config::new(args.into_iter()),
    )
}

#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    log_with_level(args, input, log::Level::Trace)
}

#[proc_macro_attribute]
pub fn debug(args: TokenStream, input: TokenStream) -> TokenStream {
    log_with_level(args, input, log::Level::Debug)
}

#[proc_macro_attribute]
pub fn info(args: TokenStream, input: TokenStream) -> TokenStream {
    log_with_level(args, input, log::Level::Info)
}

#[proc_macro_attribute]
pub fn warn(args: TokenStream, input: TokenStream) -> TokenStream {
    log_with_level(args, input, log::Level::Warn)
}

#[proc_macro_attribute]
pub fn error(args: TokenStream, input: TokenStream) -> TokenStream {
    log_with_level(args, input, log::Level::Error)
}
