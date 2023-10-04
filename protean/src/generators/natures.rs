use std::{io, path::Path};

use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

use crate::{build_names_table, load_names, sanitize_variant_name};

use super::write_generated_file;

pub fn generate_natures_data(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    let names = load_names(source_dir, "seikaku");

    // Gather formatted tokens related to natures. The "seikaku" text includes a
    // bogus 26th entry, so we explicitly take 25. It's not expected that the
    // number of natures will change, so hopefully this is safe enough.
    let natures: Vec<_> = names
        .take(25)
        .enumerate()
        .map(|(index, names)| {
            let english_name = &names[1];

            let variant = sanitize_variant_name(english_name);
            let names_data = build_names_table(&names);
            let index = Literal::usize_unsuffixed(index);

            Nature {
                index,
                variant,
                names_data,
            }
        })
        .collect();

    // Build the actual token streams for each type of data we want to generate.
    // There's a lot of repeat iteration here, but the return data type gets
    // complicated otherwise.
    let variants: TokenStream = natures
        .iter()
        .map(|nature| {
            let variant = &nature.variant;
            quote!(#variant,)
        })
        .collect();

    let data: TokenStream = natures
        .iter()
        .map(|nature| {
            let variant = &nature.variant;
            let names_data = &nature.names_data;

            quote!(Self::#variant => &NatureData {
                names: #names_data,
            },)
        })
        .collect();

    let (from, to): (TokenStream, TokenStream) = natures
        .iter()
        .map(|nature| {
            let variant = &nature.variant;
            let index = &nature.index;

            (
                quote!(#index => Self::#variant,),
                quote!(Nature::#variant => #index,),
            )
        })
        .unzip();

    // Build generated file contents.
    let tokens = quote!(
        use crate::{NamesData, NatureData};

        use common::Error;

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Nature {
            #variants
        }

        impl Nature {
            pub(crate) fn data(&self) -> &'static NatureData {
                match self {
                    #data
                }
            }
        }

        impl TryFrom<u8> for Nature {
            type Error = Error;

            fn try_from(value: u8) -> Result<Self, Error> {
                let value = match value {
                    #from

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }
        }

        impl From<Nature> for u8 {
            fn from(value: Nature) -> Self {
                match value {
                    #to
                }
            }
        }
    );

    write_generated_file(&dest_dir.join("natures.rs"), tokens)
}

struct Nature {
    index: Literal,
    variant: Ident,
    names_data: TokenStream,
}
