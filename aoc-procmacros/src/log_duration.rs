use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { sig, vis, block, attrs, } = parse_macro_input!(input as ItemFn);

    let statements = block.stmts;
    let function_identifier = sig.ident.clone();

    quote!(
        #(#attrs)*

        #vis #sig {
            let __start = std::time::Instant::now();

            let __result = {
                #(#statements)*
            };

            if std::env::var("PROFILING").is_ok() {
                const __white: &str = aoc_utils::color::WHITE;
                const __brown: &str = aoc_utils::color::BROWN;
                const __reset: &str = aoc_utils::color::RESET;
                println!("{__brown}{}::{__white}{}{__brown}() took {__white}{}.{:06}{__brown}s{__reset}", 
                    module_path!(),
                    stringify!(#function_identifier), 
                    __start.elapsed().as_secs(),
                    __start.elapsed().as_micros() % 1_000_000,
                );
            }

            return __result;
        }
    ).into()
}

