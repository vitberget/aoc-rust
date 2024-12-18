use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn token_stream_has_ident(token_stream: TokenStream, name: &str) -> bool {
    token_stream.into_iter().any(|token| 
        if let TokenTree::Ident(ident_name) = token {
            ident_name.to_string() == name
        } else {
            false
        }
    )
}

pub(crate) fn aoc_profile_impl(proc_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { sig, vis, block, attrs, } = parse_macro_input!(input as ItemFn);

    let statements = block.stmts;
    let function_identifier = sig.ident.clone();

    let counter = token_stream_has_ident(proc_args, "counter");

    let q = if counter {
        quote!(
            #(#attrs)*

            #vis #sig {
                static __AOC_PROFILING: std::sync::LazyLock<bool> = std::sync::LazyLock::new(||
                    std::env::var("AOC_PROFILING").is_ok()
                );

                if *__AOC_PROFILING {
                    static __COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                    let __counter = __COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                    const __WHITE: &str = aoc_utils::color::WHITE;
                    const __BROWN: &str = aoc_utils::color::BROWN;
                    const __RESET: &str = aoc_utils::color::RESET;
                    const __GRAY: &str = aoc_utils::color::LIGHT_GRAY;

                    let __start = std::time::Instant::now();

                    let __result = { #(#statements)* };

                    let __elapsed = __start.elapsed();
                    let __micros = __elapsed.as_micros();

                    println!("{__BROWN}{}::{__WHITE}{}{__BROWN}() took {__WHITE}{}{__RESET}.{__WHITE}{:03} {:03}{__BROWN} sec{__WHITE} {__counter}{__RESET}", 
                        module_path!(),
                        stringify!(#function_identifier), 
                        __elapsed.as_secs(),
                        (__micros / 1_000) % 1_000,
                        __micros % 1_000,
                    );

                    __result

                } else {
                    { #(#statements)* }
                }
            }
        )
    } else {
        quote!(
            #(#attrs)*

            #vis #sig {
                static __AOC_PROFILING: std::sync::LazyLock<bool> = std::sync::LazyLock::new(||
                    std::env::var("AOC_PROFILING").is_ok()
                );

                if *__AOC_PROFILING {
                    const __WHITE: &str = aoc_utils::color::WHITE;
                    const __BROWN: &str = aoc_utils::color::BROWN;
                    const __RESET: &str = aoc_utils::color::RESET;
                    const __GRAY: &str = aoc_utils::color::LIGHT_GRAY;

                    let __start = std::time::Instant::now();

                    let __result = { #(#statements)* };

                    let __elapsed = __start.elapsed();
                    let __micros = __elapsed.as_micros();

                    println!("{__BROWN}{}::{__WHITE}{}{__BROWN}() took {__WHITE}{}{__RESET}.{__WHITE}{:03} {:03}{__BROWN} sec{__RESET}", 
                        module_path!(),
                        stringify!(#function_identifier), 
                        __elapsed.as_secs(),
                        (__micros / 1_000) % 1_000,
                        __micros % 1_000,
                    );

                    __result

                } else {
                    { #(#statements)* }
                }
            }
        )

    };

    q.into()
}
