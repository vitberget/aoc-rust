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
                const __WHITE: &str = aoc_utils::color::WHITE;
                const __BROWN: &str = aoc_utils::color::BROWN;
                const __RESET: &str = aoc_utils::color::RESET;
                const __GRAY: &str = aoc_utils::color::LIGHT_GRAY;
                println!("{__BROWN}{}::{__WHITE}{}{__BROWN}() took {__WHITE}{}{__RESET}.{__WHITE}{:03} {:03}{__BROWN} sec{__RESET}", 
                    module_path!(),
                    stringify!(#function_identifier), 
                    __start.elapsed().as_secs(),
                    (__start.elapsed().as_micros() / 1_000) % 1_000,
                    __start.elapsed().as_micros() % 1_000,
                );
            }

            return __result;
        }
    ).into()
}

