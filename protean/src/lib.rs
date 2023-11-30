use std::{fs, path::Path};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

mod conversions;

mod sources;
use sources::{home, pkhex};

mod generators;
pub use generators::*;

pub(crate) mod gsc;
pub(crate) mod rgby;
pub(crate) mod rse_frlg;

static LANGUAGE_CODES: [&str; 9] = [
    "ja", "en", "fr", "it", "de", "es", "ko", "zh-Hans", "zh-Hant",
];

fn build_names_table(names: &[String]) -> TokenStream {
    let names: TokenStream = names.iter().map(|name| quote!(#name,)).collect();

    quote!(NamesData {
        names: [
            #names
        ],
    })
}

fn load_names(source_dir: &Path, data_set: &str) -> impl Iterator<Item = Vec<String>> {
    let names = LANGUAGE_CODES.map(|language| load_names_file(source_dir, data_set, language));

    let count = names[0].len();

    // Strings are loaded in lists of lines grouped by language. Transpose to
    // create lists of language entries grouped by line number.
    let mut names: Vec<_> = names.into_iter().map(|vec| vec.into_iter()).collect();
    (0..count).map(move |_| names.iter_mut().map(|iter| iter.next().unwrap()).collect())
}

fn load_names_file(source_dir: &Path, data_set: &str, language: &str) -> Vec<String> {
    let path = home::text_file_path(source_dir, data_set, language);
    let string = fs::read_to_string(path).expect("Unable to read names file");

    string
        .split_terminator(&['\r', '\n'])
        .map(String::from)
        .collect()
}

fn sanitize_variant_name(name: &str) -> Ident {
    let sanitized = name
        .replace(
            [
                ' ', '-', '\'', '.', '\u{2019}', ',', ':', '(', ')', '&', '/',
            ],
            "",
        )
        .replace('é', "e")
        .replace('♀', "Female")
        .replace('♂', "Male")
        .replace('★', "Star")
        .replace('%', "Percent")
        .replace("Clue?", "ClueQuestionMark")
        .replace("10000000", "TenMillion");

    let sanitized = if sanitized.starts_with("50") {
        sanitized.replace("50", "Fifty")
    } else if sanitized.starts_with("10") {
        sanitized.replace("10", "Ten")
    } else {
        sanitized
    };

    Ident::new(&sanitized, Span::call_site())
}
