//! # `rightpad-str` crate.
//! Obviously the second best crate (after `leftpad-str`) for the Rust programming language. The crate serves the only goal of implementing `leftpad` function:
//!
//! ```rs
//! pub fn rightpad(input: &str, width: usize, padding_char: char) -> String;
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/abs0luty/rightpad/main/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/abs0luty/rightpad/main/logo.png"
)]
#![cfg_attr(not(test), forbid(clippy::unwrap_used))]
#![warn(missing_docs, clippy::dbg_macro)]
#![deny(
    // rustc lint groups https://doc.rust-lang.org/rustc/lints/groups.html
    warnings,
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused,
    // rustc allowed-by-default lints https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    // rustdoc lints https://doc.rust-lang.org/rustdoc/lints.html
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    // clippy categories https://doc.rust-lang.org/clippy/
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::option_if_let_else
)]

/// Pads the given string on the right with a specified character up to a specified width.
///
/// # Arguments
///
/// * `input` - The string to be padded.
/// * `width` - The desired width of the resulting padded string.
/// * `padding_char` - The character used for padding.
///
/// # Examples
///
/// ```
/// use rightpad_str::rightpad;
///
/// assert_eq!(rightpad("hello", 8, '*'), "hello***");
/// assert_eq!(rightpad("rust", 6, ' '), "rust  ");
/// ```
#[must_use]
pub fn rightpad(input: &str, width: usize, padding_char: char) -> String {
    input
        .chars()
        .chain(std::iter::repeat(padding_char))
        .take(width)
        .collect::<String>()
}
