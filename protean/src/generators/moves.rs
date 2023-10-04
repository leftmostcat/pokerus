use std::{io, path::Path};

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::{build_names_table, load_names, sanitize_variant_name, sources::raw::rgby};

use super::write_generated_file;

pub fn generate_moves_data(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    let names = load_names(source_dir, "wazaname");
    let moves: Vec<_> = names
        .enumerate()
        .skip(1)
        .map(|(index, names)| {
            let english_name = &names[1];

            let move_name = rename_z_moves(index, english_name);
            let variant = sanitize_variant_name(&move_name);
            let names_data = build_names_table(&names);
            let index = Literal::usize_unsuffixed(index);

            Move {
                index,
                variant,
                names_data,
            }
        })
        .collect();

    let (variants, (data, from)): (TokenStream, (TokenStream, TokenStream)) = moves
        .iter()
        .map(|move_value| {
            let move_name = &move_value.variant;
            let names_data = &move_value.names_data;
            let index = &move_value.index;

            let variant = quote!(#move_name,);

            let data = quote!(Self::#move_name => &MoveData {
                names: #names_data,
            },);

            let from = quote!(#index => Self::#move_name,);

            (variant, (data, from))
        })
        .unzip();

    generate_moves_data_rgby(source_dir, dest_dir, &moves).unwrap();

    let tokens = quote!(
        use common::Error;

        use crate::{MoveData, NamesData};

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Move {
            #variants
        }

        impl Move {
            pub(crate) fn move_data(&self) -> &'static MoveData {
                match self {
                    #data
                }
            }
        }

        impl TryFrom<u16> for Move {
            type Error = Error;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                let value = match value {
                    #from
                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }
        }
    );

    write_generated_file(&dest_dir.join("moves.rs"), tokens)
}

struct Move {
    index: Literal,
    variant: Ident,
    names_data: TokenStream,
}

fn generate_moves_data_rgby(source_dir: &Path, dest_dir: &Path, moves: &[Move]) -> io::Result<()> {
    let variants: Vec<_> = moves.iter().map(|move_value| &move_value.variant).collect();

    let rgby_data = rgby::read_moves(source_dir);
    let move_tokens: TokenStream = rgby_data
        .into_iter()
        .map(|move_data| {
            let name = &variants[move_data.idx as usize - 1];
            let base_pp = Literal::u8_unsuffixed(move_data.base_pp);
            let move_type = Ident::new(move_data.move_type(), Span::call_site());
            let power = Literal::u8_unsuffixed(move_data.power);
            let accuracy = Literal::u8_unsuffixed(move_data.accuracy());

            quote!(Move::#name => &MoveDataRGBY {
                max_pp: #base_pp,
                move_type: Type::#move_type,
                power: #power,
                accuracy: #accuracy,
            },)
        })
        .collect();

    let tokens = quote!(
        use crate::{Move, MoveDataRGBY, Type};

        use common::Error;

        impl Move {
            pub(crate) fn move_data_for_rgby(&self) -> Result<&'static MoveDataRGBY, Error> {
                let value = match self {
                    #move_tokens

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }
        }
    );

    write_generated_file(&dest_dir.join("move_data_rgby.rs"), tokens)
}

fn rename_z_moves(index: usize, name: &str) -> String {
    static MIN_DUPLICATE_NAME_INDEX: usize = 623;
    static MAX_DUPLICATE_NAME_INDEX: usize = 658;
    if index >= MIN_DUPLICATE_NAME_INDEX && index <= MAX_DUPLICATE_NAME_INDEX {
        // Some Z-moves share the same name but differ in physical
        // vs. special. To prevent duplicate identifiers, give them
        // special handling.
        let category = if index % 2 == 0 {
            "Physical"
        } else {
            "Special"
        };

        format!("{name}{category}")
    } else {
        String::from(name)
    }
}
