use std::{io, iter, path::Path};

use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

use crate::{build_names_table, gsc, load_names, rgby, sanitize_variant_name};

use super::write_generated_file;

pub fn generate_types_data(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    let names = load_names(source_dir, "typename")
        .collect::<Vec<_>>()
        .into_iter();
    let names = names
        .clone()
        .take(9)
        .chain(iter::once(Vec::from_iter(
            QUESTION_MARK_NAMES.into_iter().map(String::from),
        )))
        .chain(names.skip(9));

    let types: Vec<_> = names
        .enumerate()
        .map(|(index, names)| {
            let english_name = &names[1];

            let variant = match index {
                9 => format_ident!("QuestionMark"),
                _ => sanitize_variant_name(english_name),
            };
            let names_data = build_names_table(&names);

            Type {
                index: index as u8,
                variant,
                names_data,
            }
        })
        .collect();

    // Build the actual token streams for each type of data we want to generate.
    // There's a lot of repeat iteration here, but the return data type gets
    // complicated otherwise.
    let variants: TokenStream = types
        .iter()
        .map(|type_value| {
            let variant = &type_value.variant;
            quote!(#variant,)
        })
        .collect();

    let data: TokenStream = types
        .iter()
        .map(|type_value| {
            let variant = &type_value.variant;
            let names_data = &type_value.names_data;

            quote!(Self::#variant => &TypeData {
                names: #names_data,
            },)
        })
        .collect();

    let (from_gen1, to_gen1): (TokenStream, TokenStream) = types
        .iter()
        .flat_map(|type_value| {
            rgby::type_id_to_gen_1_idx(type_value.index).map(|index| {
                let variant = &type_value.variant;
                let index = Literal::u8_unsuffixed(index);

                (
                    quote!(#index => Self::#variant,),
                    quote!(Self::#variant => #index,),
                )
            })
        })
        .unzip();

    let (from_gen2, to_gen2): (TokenStream, TokenStream) = types
        .iter()
        .flat_map(|type_value| {
            gsc::type_id_to_gen_2_idx(type_value.index).map(|index| {
                let variant = &type_value.variant;
                let index = Literal::u8_unsuffixed(index);

                (
                    quote!(#index => Self::#variant,),
                    quote!(Self::#variant => #index,),
                )
            })
        })
        .unzip();

    // Build generated file contents.
    let tokens = quote!(
        use crate::{NamesData, TypeData};

        use common::Error;

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Type {
            #variants
        }

        impl Type {
            pub(crate) fn data(&self) -> &'static TypeData {
                match self {
                    #data
                }
            }

            pub(crate) fn try_from_gen_1_id(value: u8) -> Result<Self, Error> {
                let value = match value {
                    #from_gen1

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }

            pub(crate) fn try_into_gen_1_id(&self) -> Result<u8, Error> {
                let value = match self {
                    #to_gen1

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }

            pub(crate) fn try_from_gen_2_id(value: u8) -> Result<Self, Error> {
                let value = match value {
                    #from_gen2

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }

            pub(crate) fn try_into_gen_2_id(&self) -> Result<u8, Error> {
                let value = match self {
                    #to_gen2

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }
        }
    );

    write_generated_file(&dest_dir.join("types.rs"), tokens)
}

struct Type {
    index: u8,
    variant: Ident,
    names_data: TokenStream,
}

static QUESTION_MARK_NAMES: [&str; 9] =
    ["？？？", "???", "???", "???", "???", "(?)", "???", "", ""];
