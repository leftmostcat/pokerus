use std::{fs, io, iter::Peekable, path::Path};

use proc_macro2::{Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};

use crate::{
    build_names_table, load_names,
    pkhex::{
        gsc::PersonalInfoGSC,
        personal_info_path,
        rgby::{PersonalInfoGen1 as _, PersonalInfoRGBY},
        PersonalInfo, PersonalInfoRevised as _, PersonalTable,
    },
    rgby, sanitize_variant_name,
};

mod forms;
use self::forms::{build_forms_data, Form};

use super::write_generated_file;

struct Tables {
    table_rgb: PersonalTable<PersonalInfoRGBY>,
    table_y: PersonalTable<PersonalInfoRGBY>,
    table_gs: PersonalTable<PersonalInfoGSC>,
    table_c: PersonalTable<PersonalInfoGSC>,
}

struct Species<'a> {
    national_dex_id: u16,
    names: Vec<String>,
    forms: Vec<Form>,
    rgby_index: Option<u8>,
    rgb: Option<&'a PersonalInfoRGBY>,
    y: Option<&'a PersonalInfoRGBY>,
    gs: Option<&'a PersonalInfoGSC>,
    c: Option<&'a PersonalInfoGSC>,
}

impl<'a> Species<'a> {
    fn new(
        national_dex_id: u16,
        names: Vec<String>,
        forms: Vec<Form>,
        context: &'a Tables,
    ) -> Self {
        let index = national_dex_id as usize;
        let rgby_index = rgby::national_dex_id_to_gen_1_idx(national_dex_id);

        let rgb_table = context.table_rgb.info_blocks.get(index);
        let y_table = context.table_y.info_blocks.get(index);

        assert_eq!(rgby_index.is_some(), rgb_table.is_some());
        assert_eq!(rgb_table.is_some(), y_table.is_some());

        Self {
            national_dex_id,
            names,
            forms,
            rgby_index,
            rgb: rgb_table,
            y: y_table,
            gs: context.table_gs.info_blocks.get(index),
            c: context.table_c.info_blocks.get(index),
        }
    }

    fn english_name(&self) -> &str {
        &self.names[1]
    }

    fn variant(&self) -> Ident {
        sanitize_variant_name(self.english_name())
    }

    fn names_table(&self) -> TokenStream {
        build_names_table(&self.names)
    }

    fn matcher_with_pattern<T: ToTokens>(&self, pattern: T) -> TokenStream {
        let variant = self.variant();
        quote!(#variant(#pattern))
    }

    fn form_enum_name(&self) -> Ident {
        let variant = self.variant();
        format_ident!("{variant}Form")
    }

    fn matcher_for_form<T: std::fmt::Display>(&self, form: T) -> TokenStream {
        let form_enum = self.form_enum_name();
        let form = format_ident!("{form}");
        self.matcher_with_pattern(quote!(#form_enum::#form))
    }

    fn species_data_rgb(&self) -> Option<TokenStream> {
        self.rgb.map(|table| species_data_rgby(self, table))
    }

    fn species_data_y(&self) -> Option<TokenStream> {
        self.y.map(|table| species_data_rgby(self, table))
    }

    fn species_data_gs(&self) -> Option<TokenStream> {
        self.gs.map(|table| species_data_gsc(self, table))
    }

    fn species_data_c(&self) -> Option<TokenStream> {
        self.c.map(|table| species_data_gsc(self, table))
    }

    fn species_data(&self, data_set: &str) -> Option<TokenStream> {
        match data_set {
            "rb" => self.species_data_rgb(),
            "y" => self.species_data_y(),
            "gs" => self.species_data_gs(),
            "c" => self.species_data_c(),

            _ => panic!("Invalid data set name"),
        }
    }
}

pub fn generate_species_data(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    let names: Vec<_> = load_names(source_dir, "monsname").collect();

    let tables = Tables {
        table_rgb: load_personal_table(source_dir, "rb"),
        table_y: load_personal_table(source_dir, "y"),
        table_gs: load_personal_table(source_dir, "gs"),
        table_c: load_personal_table(source_dir, "c"),
    };

    let forms = build_forms_data(source_dir, names.len() - 1);

    let all_species: Vec<_> = names
        .into_iter()
        .enumerate()
        .skip(1)
        .zip(forms)
        .map(|((index, names), forms)| {
            if index > u16::MAX as usize {
                // If we hit 65,536 Pokémon species, we'll need to bump the
                // int type for national dex ID, but that's likely a long time
                // away from the writing of this comment, if it ever happens.
                panic!("There are too many species! I can't fit them all in a `u16` anymore!");
            }

            Species::new(index as u16, names, forms, &tables)
        })
        .collect();

    generate_species_data_rgby(&all_species, dest_dir, "rb").unwrap();
    generate_species_data_rgby(&all_species, dest_dir, "y").unwrap();
    generate_species_data_gsc(&all_species, dest_dir, "gs").unwrap();
    generate_species_data_gsc(&all_species, dest_dir, "c").unwrap();

    let variants: TokenStream = all_species
        .iter()
        .map(|species| {
            let variant = species.matcher_with_pattern(species.form_enum_name());

            quote!(#variant,)
        })
        .collect();

    let species_data: TokenStream = all_species
        .iter()
        .map(|species| {
            let names_data = species.names_table();

            let data_matcher = species.matcher_with_pattern(Punct::new('_', Spacing::Joint));
            let national_dex_id = Literal::u16_unsuffixed(species.national_dex_id);
            quote!(Self::#data_matcher => &SpeciesData {
                national_dex_id: #national_dex_id,
                names: #names_data,
            },)
        })
        .collect();

    let forms: TokenStream = all_species
        .iter()
        .map(|species| {
            let (variants, data): (TokenStream, TokenStream) = species
                .forms
                .iter()
                .map(|form| {
                    let variant = &form.variant;
                    let names_data = build_names_table(&form.names);

                    (
                        quote!(#variant,),
                        quote!(Self::#variant => &FormData {
                            names: #names_data,
                        },),
                    )
                })
                .unzip();

            let form_enum = species.form_enum_name();
            quote!(
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub enum #form_enum {
                    #variants
                }

                impl #form_enum {
                    pub(crate) fn data(self) -> &'static FormData {
                        match self {
                            #data
                        }
                    }
                }
            )
        })
        .collect();

    let form_data: TokenStream = all_species
        .iter()
        .map(|species| {
            let data_matcher = species.matcher_with_pattern(format_ident!("form"));
            quote!(Self::#data_matcher => form.data(),)
        })
        .collect();

    let rgby_matches: TokenStream = all_species
        .iter()
        .take_while(|species| species.national_dex_id <= 151)
        .flat_map(|species| {
            species.rgby_index.map(|idx| {
                let literal = Literal::u8_unsuffixed(idx);
                let base_form = species.matcher_for_form("Base");

                quote!(#literal => Self::#base_form,)
            })
        })
        .collect();

    let gsc_matches: TokenStream = all_species.iter().take_while(|species| species.national_dex_id <= 251).flat_map(|species| {
        let species_literal = Literal::u16_unsuffixed(species.national_dex_id);
        let species_variant = species.variant();
        let form_enum_name = species.form_enum_name();

        species.forms.iter().flat_map(|form| {
            let form_literal = Literal::u8_unsuffixed(form.gsc_index?);
            let form_variant = &form.variant;

            Some(quote!((#species_literal, #form_literal) => Self::#species_variant(#form_enum_name::#form_variant),))
        }).collect::<TokenStream>()
    }).collect();

    let tokens = quote!(
        #![allow(clippy::enum_variant_names)]

        use common::Error;

        use crate::{FormData, NamesData, SpeciesData};

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Species {
            #variants
        }

        impl Species {
            pub(crate) fn try_from_gen_1_species_id(idx: u8) -> Result<Self, Error> {
                let value = match idx {
                    #rgby_matches

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }

            pub(crate) fn try_from_gen_2_species_and_form_id(species_idx: u8, form_idx: u8) -> Result<Self, Error> {
                let value = match (species_idx, form_idx) {
                    #gsc_matches

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(value)
            }

            pub(crate) fn species_data(&self) -> &'static SpeciesData {
                match self {
                    #species_data
                }
            }

            pub(crate) fn form_data(&self) -> &'static FormData {
                match self {
                    #form_data
                }
            }
        }

        #forms
    );

    write_generated_file(&dest_dir.join("species.rs"), tokens)
}

fn generate_species_data_rgby(
    all_species: &[Species],
    out_dir: &Path,
    data_set: &str,
) -> io::Result<()> {
    let species_data: TokenStream = all_species
        .iter()
        .filter_map(|species| species.species_data(data_set))
        .collect();

    let fn_name = format_ident!("species_data_{data_set}");
    let tokens = quote!(
        use super::*;
        use crate::{ExperienceGrowthRate, Species, SpeciesDataRGBY, Type};

        use common::Error;

        impl Species {
            pub(crate) fn #fn_name(&self) -> Result<&'static SpeciesDataRGBY, Error> {
                let data = match self {
                    #species_data
                    _ => return Err(Error::invalid_argument()),
                };

                Ok(data)
            }
        }
    );

    let filename = format!("species_data_{data_set}.rs");
    write_generated_file(&out_dir.join(filename), tokens)
}

fn species_data_rgby(species: &Species, table: &PersonalInfoRGBY) -> TokenStream {
    let base_hp = Literal::u8_unsuffixed(table.base_hp());
    let base_atk = Literal::u8_unsuffixed(table.base_atk());
    let base_def = Literal::u8_unsuffixed(table.base_def());
    let base_spc = Literal::u8_unsuffixed(table.base_spc());
    let base_spe = Literal::u8_unsuffixed(table.base_spe());
    let primary_type = format_ident!("{}", table.primary_type());
    let secondary_type = format_ident!("{}", table.secondary_type());
    let experience_growth_rate = format_ident!("{}", table.experience_growth_rate());

    let pattern = {
        let matcher = species.matcher_for_form("Base");

        quote!(Species::#matcher)
    };

    quote!(#pattern => &SpeciesDataRGBY {
        base_hp: #base_hp,
        base_atk: #base_atk,
        base_def: #base_def,
        base_spc: #base_spc,
        base_spe: #base_spe,
        primary_type: Type::#primary_type,
        secondary_type: Type::#secondary_type,
        experience_growth_rate: ExperienceGrowthRate::#experience_growth_rate,
    },)
}

fn generate_species_data_gsc(
    all_species: &[Species],
    out_dir: &Path,
    data_set: &str,
) -> io::Result<()> {
    let species_data: TokenStream = all_species
        .iter()
        .filter_map(|species| species.species_data(data_set))
        .collect();

    let fn_name = format_ident!("species_data_{data_set}");
    let tokens = quote!(
        use super::*;
        use crate::{ExperienceGrowthRate, GenderRatio, Species, SpeciesDataGSC, Type};

        use common::Error;

        impl Species {
            pub(crate) fn #fn_name(&self) -> Result<&'static SpeciesDataGSC, Error> {
                let data = match self {
                    #species_data

                    _ => return Err(Error::invalid_argument()),
                };

                Ok(data)
            }
        }
    );

    let filename = format!("species_data_{data_set}.rs");
    write_generated_file(&out_dir.join(filename), tokens)
}

fn species_data_gsc(species: &Species, table: &PersonalInfoGSC) -> TokenStream {
    let base_hp = Literal::u8_unsuffixed(table.base_hp());
    let base_atk = Literal::u8_unsuffixed(table.base_atk());
    let base_def = Literal::u8_unsuffixed(table.base_def());
    let base_spa = Literal::u8_unsuffixed(table.base_spa());
    let base_spd = Literal::u8_unsuffixed(table.base_spd());
    let base_spe = Literal::u8_unsuffixed(table.base_spe());
    let primary_type = format_ident!("{}", table.primary_type());
    let secondary_type = format_ident!("{}", table.secondary_type());
    let experience_growth_rate = format_ident!("{}", table.experience_growth_rate());
    let gender_ratio = format_ident!("{}", table.gender_ratio());

    let pattern = if species.national_dex_id == 201 {
        let variants = ('A'..='Z').map(|form| {
            let matcher = species.matcher_for_form(form);

            quote!(Species::#matcher)
        });

        BadIntersperse {
            iter: variants.peekable(),
            next_from_iter: true,
        }
        .collect()
    } else {
        let matcher = species.matcher_for_form("Base");

        quote!(Species::#matcher)
    };

    quote!(#pattern => &SpeciesDataGSC {
        base_hp: #base_hp,
        base_atk: #base_atk,
        base_def: #base_def,
        base_spa: #base_spa,
        base_spd: #base_spd,
        base_spe: #base_spe,
        primary_type: Type::#primary_type,
        secondary_type: Type::#secondary_type,
        experience_growth_rate: ExperienceGrowthRate::#experience_growth_rate,
        gender_ratio: GenderRatio::#gender_ratio,
    },)
}

fn load_personal_table<I: PersonalInfo>(source_dir: &Path, data_set: &str) -> PersonalTable<I> {
    let table = personal_info_path(source_dir, data_set);
    let table = fs::read(table).unwrap();

    PersonalTable::new(table)
}

struct BadIntersperse<I: Iterator<Item = TokenStream>> {
    iter: Peekable<I>,
    next_from_iter: bool,
}

impl<I: Iterator<Item = TokenStream>> Iterator for BadIntersperse<I> {
    type Item = TokenStream;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_from_iter {
            self.next_from_iter = false;

            self.iter.next()
        } else if self.iter.peek().is_some() {
            self.next_from_iter = true;

            Some(TokenTree::Punct(Punct::new('|', Spacing::Alone)).into())
        } else {
            None
        }
    }
}
