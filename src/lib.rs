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

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemFn);

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
