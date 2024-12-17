extern crate proc_macro;

use log_duration::log_duration_impl;
use proc_macro::TokenStream;

pub(crate) mod log_duration;

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    log_duration_impl(args, item)
}

