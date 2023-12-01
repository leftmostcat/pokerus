use std::{io, path::Path};

use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

use crate::{build_names_match, load_names, sanitize_variant_name};

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

            let index = Literal::usize_unsuffixed(index);
            let variant = sanitize_variant_name(english_name);
            let names_match = build_names_match(&names);

            Nature {
                index,
                variant,
                names_match,
            }
        })
        .collect();

    // Build the actual token streams for each type of data we want to generate.
    // There's a lot of repeat iteration here, but the return data type gets
    // complicated otherwise.
    let indices: Vec<_> = natures.iter().map(|nature| &nature.index).collect();
    let variants: Vec<_> = natures.iter().map(|nature| &nature.variant).collect();
    let names: Vec<_> = natures.iter().map(|nature| &nature.names_match).collect();

    // Build generated file contents.
    let tokens = quote!(
        use crate::Language;

        use common::Error;

        /// `Nature` is a property of a Pokémon which affects stat calculations
        /// as well as flavor preferences.
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Nature {
            #(#variants),*
        }

        impl Nature {
            /// Gets the name of this `Nature` in the given language.
            pub fn name(&self, language: Language) -> &'static str {
                match self {
                    #(Self::#variants => #names,)*
                }
            }
        }

        impl TryFrom<u8> for Nature {
            type Error = Error;

            fn try_from(value: u8) -> Result<Self, Error> {
                let value = match value {
                    #(#indices => Self::#variants,)*

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }
        }

        impl From<Nature> for u8 {
            fn from(value: Nature) -> Self {
                match value {
                    #(Nature::#variants => #indices,)*
                }
            }
        }
    );

    write_generated_file(&dest_dir.join("natures.rs"), tokens)
}

struct Nature {
    index: Literal,
    variant: Ident,
    names_match: TokenStream,
}
