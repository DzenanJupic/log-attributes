use std::str::FromStr;

use proc_macro2::Span;
use syn::{Meta, NestedMeta};
use syn::spanned::Spanned;

pub struct Config {
    pub log_level: log::Level,
    pub fmt_string: syn::LitStr,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item=NestedMeta>) -> Result<Self, syn::Error> {
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

    pub fn with_log_level(mut args: impl Iterator<Item=NestedMeta>, log_level: log::Level) -> Result<Self, syn::Error> {
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
