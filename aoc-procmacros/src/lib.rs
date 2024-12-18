extern crate proc_macro;

use aoc_profile::aoc_profile_impl;
use proc_macro::TokenStream;

pub(crate) mod aoc_profile;

#[proc_macro_attribute]
pub fn aoc_profile(args: TokenStream, item: TokenStream) -> TokenStream {
    aoc_profile_impl(args, item)
}

