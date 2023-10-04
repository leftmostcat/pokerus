use std::{iter, path::Path};

use proc_macro2::Ident;

use crate::{load_names, sanitize_variant_name};

pub(crate) struct Form {
    pub(crate) variant: Ident,
    pub(crate) names: Vec<String>,
    pub(crate) gsc_index: Option<u8>,
}

impl Form {
    fn from_names(names: Vec<String>, gsc_index: Option<u8>) -> Self {
        let variant = String::from(match names[1].as_str() {
            "" => "Base",
            name if name.starts_with("Mega") => {
                if name.ends_with('X') {
                    "MegaX"
                } else if name.ends_with('Y') {
                    "MegaY"
                } else {
                    "Mega"
                }
            }
            name => name,
        });

        Self::from_variant_and_names(&variant, names, gsc_index)
    }

    fn from_variant_and_names(variant: &str, names: Vec<String>, gsc_index: Option<u8>) -> Self {
        Self {
            variant: sanitize_variant_name(variant),
            names,
            gsc_index,
        }
    }
}

pub(super) fn build_forms_data(source_dir: &Path, species_count: usize) -> Vec<Vec<Form>> {
    let form_names: Vec<_> = load_names(source_dir, "zkn_form").skip(1).collect();

    let (specifiers, pre_alt_count, alt_count) = build_form_name_specifiers(species_count);

    let (templates, form_names) = form_names.split_at(1);
    let templates = &templates[0];
    let (pre_alt_main_names, rest) = form_names.split_at(pre_alt_count);
    let (alt_names, post_alt_main_names) = rest.split_at(alt_count);
    let mut main_names = pre_alt_main_names.iter().chain(post_alt_main_names.iter());
    let mut alt_names = alt_names.iter();

    specifiers
        .into_iter()
        .enumerate()
        .map(|(index, specifier)| {
            let form_names = (0..specifier.main_forms)
                .map(|_| main_names.next().unwrap())
                .chain((0..specifier.alt_forms).map(|_| alt_names.next().unwrap()))
                .cloned()
                .collect();

            build_forms_for_species(index, form_names, templates)
        })
        .collect()
}

fn build_form_name_specifiers(species_count: usize) -> (Vec<FormNamesSpecifier>, usize, usize) {
    let (specifiers, (pre_alt_counts, alt_counts)): (Vec<_>, (Vec<_>, Vec<_>)) =
        (1..=species_count)
            .map(|index| {
                let specifier = match index {
                    // Pokémon with both Mega and Gigantamax forms: Venusaur, Blastoise
                    3 | 9 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Charizard
                    6 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3, // Two Mega evolutions and Gigantamax
                    },

                    // Gigantamax Pokémon: Butterfree, Machamp, Kingler, Lapras,
                    // Snorlax, Garbodor, Melmetal, Rillaboom, Cinderace, Inteleon,
                    // Corviknight, Orbeetle, Drednaw, Coalossal, Flapple, Appletun,
                    // Sandaconda, Centiskorch, Hatterene, Grimmsnarl, Copperajah,
                    // Duraludon
                    12 | 68 | 99 | 131 | 143 | 569 | 809 | 812 | 815 | 818 | 825 | 826 | 834
                    | 839 | 841 | 842 | 844 | 851 | 858 | 861 | 879 | 884 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Pokémon with single Mega forms: Beedrill, Pidgeot, Alakazam,
                    // Kangaskhan, Pinsir, Gyarados, Aerodactyl, Ampharos, Steelix,
                    // Scizor, Heracross, Houndoom, Tyranitar, Sceptile, Blaziken,
                    // Swampert, Gardevoir, Sableye, Mawile, Aggron, Medicham,
                    // Manectric, Sharpedo, Camerupt, Altaria, Banette, Absol, Glalie,
                    // Salamence, Metagross, Latias, Latios, Rayquaza, Lopunny,
                    // Garchomp, Lucario, Abomasnow, Gallade, Audino, Diancie
                    15 | 18 | 65 | 115 | 127 | 130 | 142 | 181 | 208 | 212 | 214 | 229 | 248
                    | 254 | 257 | 260 | 282 | 302 | 303 | 306 | 308 | 310 | 319 | 323 | 334
                    | 354 | 359 | 362 | 373 | 376 | 380 | 381 | 384 | 428 | 445 | 448 | 460
                    | 475 | 531 | 719 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Pokémon with Alolan forms: Rattata, Raichu, Sandshrew, Sandslash,
                    // Vulpix, Ninetales, Diglett, Dugtrio, Persian, Geodude, Graveler,
                    // Golem, Grimer, Muk, Exeggutor
                    19 | 26..=28 | 37 | 38 | 50 | 51 | 53 | 74..=76 | 88 | 89 | 103 => {
                        FormNamesSpecifier {
                            main_forms: 1,
                            alt_forms: 1,
                        }
                    }

                    // Pokémon with Alolan and totem forms: Raticate, Marowak
                    20 | 105 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Pikachu
                    25 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 10,
                    },

                    // Meowth
                    52 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3, // Alolan, Galarian, Gigantamax
                    },

                    // Pokémon with Hisuian forms: Growlithe, Voltorb, Typhlosion,
                    // Qwilfish, Sneasel, Samurott, Zorua, Zoroark, Braviary, Sliggoo,
                    // Goodra, Avalugg, Decidueye
                    58 | 100 | 157 | 211 | 215 | 503 | 570 | 571 | 628 | 705 | 706 | 724 => {
                        FormNamesSpecifier {
                            main_forms: 1,
                            alt_forms: 1,
                        }
                    }

                    // Pokémon with Hisuian and Lord/Lady forms: Arcanine, Electrode, Lilligant, Avalugg
                    59 | 101 | 549 | 713 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Pokémon with Galarian forms: Ponyta, Rapidash, Slowpoke,
                    // Farfetch'd, Weezing, Mr. Mime, Articuno, Zapdos, Moltres,
                    // Slowking, Corsola, Zigzagoon, Linoone, Darumaka, Yamask, Stunfisk
                    77
                    | 78
                    | 79
                    | 83
                    | 110
                    | 122
                    | 144..=146
                    | 199
                    | 222
                    | 263
                    | 264
                    | 554
                    | 562
                    | 618 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Slowbro
                    80 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2, // Mega evolution and Galarian
                    },

                    // Gengar
                    94 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2, // Mega evolution and Gigantamax
                    },

                    // Tauros
                    128 => FormNamesSpecifier {
                        main_forms: 7, // Paldean forms
                        alt_forms: 0,
                    },

                    // Eevee
                    133 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2, // Let's Go partner form and Gigantamax
                    },

                    // Mewtwo
                    150 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2, // Two Mega evolutions
                    },

                    // Wooper
                    194 => FormNamesSpecifier {
                        main_forms: 2, // Paldean form
                        alt_forms: 0,
                    },

                    // Unown
                    201 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 27,
                    },

                    // Castform
                    351 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Pokémon with Primal Reversion: Kyogre, Groudon
                    382 | 383 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Deoxys
                    386 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Burmy, Wormadam, Mothim
                    // Forms are not displayed differently for Mothim, but they are
                    // present in data.
                    412..=414 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Cherrim
                    421 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Shellos, Gastrodon
                    422 | 423 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Rotom
                    479 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 5,
                    },

                    // Dialga, Palkia, Giratina
                    483 | 484 | 487 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Shaymin
                    492 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Arceus
                    493 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 17,
                    },

                    // Pokémon with per-gender Dex entries in SwSh: Unfezant, Frillish,
                    // Jellicent
                    521 | 592 | 593 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Basculin
                    550 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Darmanitan
                    555 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Deerling, Sawsbuck
                    585 | 586 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Tornadus, Thundurus, Landorus, Eanmorus
                    641 | 642 | 645 | 905 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Kyurem
                    646 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Keldeo
                    647 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Meloetta
                    648 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Genesect
                    649 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 4,
                    },

                    // Greninja
                    658 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2, // Battle Bond form, Ash-Greninja
                    },

                    // Scatterbug, Spewpa, Vivillon
                    664..=666 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 19,
                    },

                    // Flabébé, Florges
                    669 | 671 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 4,
                    },

                    // Floette
                    // There is an additional (presently unobtainable) "Eternal Flower"
                    // form for Floette which Flabébé and Florges do not have.
                    670 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 5,
                    },

                    // Furfrou
                    676 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 9,
                    },

                    // Gendered forms with stat differences prior to Gen 9: Meowstic,
                    // Indeedee, Basculegion
                    678 | 876 | 902 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Aegislash
                    681 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Pumpkaboo, Gourgeist
                    710 | 711 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Xerneas
                    716 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Zygarde
                    718 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 4,
                    },

                    // Hoopa
                    720 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Pokémon with totem forms: Gumshoos, Vikavolt, Ribombee,
                    // Araquanid, Lurantis, Salazzle, Togedemaru, Kommo-o
                    735 | 738 | 743 | 752 | 754 | 758 | 777 | 784 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Oricorio
                    741 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Rockruff
                    744 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1, // Own Tempo form
                    },

                    // Lycanroc
                    745 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Wishiwashi
                    746 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Silvally
                    773 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 17,
                    },

                    // Minior
                    774 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 13,
                    },

                    // Mimikyu
                    778 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Necrozma
                    800 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Magearna
                    801 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Cramorant
                    845 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Toxtricity
                    849 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Sinistea, Polteageist
                    854 | 855 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Alcremie
                    869 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 9,
                    },

                    // Eiscue
                    875 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Morpeko
                    877 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Zacian, Zamazenta
                    888 | 889 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Eternatus
                    890 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Urshifu
                    892 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 3,
                    },

                    // Zarude
                    893 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Calyrex
                    898 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 2,
                    },

                    // Kleavor
                    900 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Ursaluna
                    901 => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 1,
                    },

                    // Oinkologne
                    916 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Maushold
                    925 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Squawkabilly
                    931 => FormNamesSpecifier {
                        main_forms: 4,
                        alt_forms: 0,
                    },

                    // Palafin
                    964 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Tatsugiri
                    978 => FormNamesSpecifier {
                        main_forms: 3,
                        alt_forms: 0,
                    },

                    // Dudunsparce
                    982 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Gimmighoul
                    999 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Koraidon, Miraidon
                    1007 | 1008 => FormNamesSpecifier {
                        main_forms: 5,
                        alt_forms: 0,
                    },

                    // Poltchageist, Sinistcha
                    1012 | 1013 => FormNamesSpecifier {
                        main_forms: 2,
                        alt_forms: 0,
                    },

                    // Ogerpon
                    1017 => FormNamesSpecifier {
                        main_forms: 4,
                        alt_forms: 0,
                    },

                    _ => FormNamesSpecifier {
                        main_forms: 1,
                        alt_forms: 0,
                    },
                };

                let pre_alt_count = if index <= 905 {
                    specifier.main_forms
                } else {
                    0
                };

                let alt_count = specifier.alt_forms;

                (specifier, (pre_alt_count, alt_count))
            })
            .unzip();

    (
        specifiers,
        pre_alt_counts.into_iter().sum(),
        alt_counts.into_iter().sum(),
    )
}

fn build_forms_for_species(
    index: usize,
    form_names: Vec<Vec<String>>,
    templates: &[String],
) -> Vec<Form> {
    let national_dex_id = index + 1;

    let form_names = adjust_form_names_list(national_dex_id, form_names, templates);
    form_names
        .into_iter()
        .enumerate()
        .map(|(index, names)| {
            // In most cases, we generate variant names from the English name
            // for the form, but in some cases that's insufficient and we have
            // to specify our own variant name.
            match national_dex_id {
                // Raticate, Marowak
                20 | 105 => match index {
                    0 => Form::from_names(names, Some(0)),

                    // Totem forms are not present in the Pokédex and have blank
                    // names, so specify our own variant.
                    2 => Form::from_variant_and_names("TotemForm", names, None),

                    _ => Form::from_names(names, None),
                },

                // Pikachu
                25 => match index {
                    0 => Form::from_names(names, Some(0)),

                    // The partner in LGPE is a separate form, but has no entry
                    // in the Pokédex and a blank name in the data.
                    14 => Form::from_variant_and_names("LetsGoPartner", names, None),

                    _ => Form::from_names(names, None),
                },

                // Eevee
                133 => match index {
                    0 => Form::from_names(names, Some(0)),

                    // The partner in LGPE is a separate form, but has no entry
                    // in the Pokédex and a blank name in the data.
                    1 => Form::from_variant_and_names("LetsGoPartner", names, None),

                    _ => Form::from_names(names, None),
                },

                // Pichu
                172 => match index {
                    0 => Form::from_names(names, Some(0)),

                    // While our sanitization would produce an acceptable name,
                    // we override it for capitalization purposes.
                    1 => Form::from_variant_and_names("SpikyEaredPichu", names, None),

                    _ => Form::from_names(names, None),
                },

                // Unown
                201 => {
                    // The Pokédex simply lists each Unown as "One form", so we
                    // have to make our own list.
                    let variant = [
                        "A",
                        "B",
                        "C",
                        "D",
                        "E",
                        "F",
                        "G",
                        "H",
                        "I",
                        "J",
                        "K",
                        "L",
                        "M",
                        "N",
                        "O",
                        "P",
                        "Q",
                        "R",
                        "S",
                        "T",
                        "U",
                        "V",
                        "W",
                        "X",
                        "Y",
                        "Z",
                        "ExclamationMark",
                        "QuestionMark",
                    ][index];
                    let gsc_index = if index < 26 { Some(index as u8) } else { None };

                    Form::from_variant_and_names(variant, names, gsc_index)
                }

                // Mothim
                414 => {
                    // Because Mothim's forms are visually indistinct, the forms
                    // are not listed separately in the Pokédex and the data has
                    // blank names.
                    let variant = ["PlantCloak", "SandyCloak", "TrashCloak"][index];

                    Form::from_variant_and_names(variant, names, None)
                }

                // Arceus
                493 => {
                    // All Arceus forms are simply listed as "Arceus", so we
                    // need to manually specify variants.
                    let variant = [
                        "Normal", "Fighting", "Flying", "Poison", "Ground", "Rock", "Bug", "Ghost",
                        "Steel", "Fire", "Water", "Grass", "Electric", "Psychic", "Ice", "Dragon",
                        "Dark", "Fairy",
                    ][index];

                    Form::from_variant_and_names(variant, names, None)
                }

                // Unfezant, Frillish, Jellicent
                521 | 592 | 593 => {
                    // These species have separate Pokédex entries by gender in
                    // SwSh, but the forms are unnamed.
                    let variant = ["Male", "Female"][index];

                    Form::from_variant_and_names(variant, names, None)
                }

                // Darmanitan
                555 => match index {
                    // The name of the Zen Mode form is identical for both the
                    // Unovan and Galarian forms, so distinguish it manually.
                    3 => Form::from_variant_and_names("GalarianZenMode", names, None),

                    _ => Form::from_names(names, None),
                },

                // Genesect
                649 => match index {
                    0 => Form::from_names(names, None),

                    _ => {
                        // All Genesect forms are simply listed as "Genesect",
                        // so we need to distinguish the forms with drives.
                        let variant =
                            ["DouseDrive", "ShockDrive", "BurnDrive", "ChillDrive"][index - 1];

                        Form::from_variant_and_names(variant, names, None)
                    }
                },

                // Greninja
                658 => match index {
                    // Greninja with Battle Bond is a separate form, but it is
                    // not distinct in the Pokédex and has a blank name.
                    1 => Form::from_variant_and_names("BattleBond", names, None),

                    _ => Form::from_names(names, None),
                },

                // Scatterbug, Spewpa
                664 | 665 => {
                    // These species have distinct forms which evolve into the
                    // corresponding Vivillon form, but the form names in the
                    // data are blank.
                    let variant = [
                        "IcySnowPattern",
                        "PolarPattern",
                        "TundraPattern",
                        "ContinentalPattern",
                        "GardenPattern",
                        "ElegantPattern",
                        "MeadowPattern",
                        "ModernPattern",
                        "MarinePattern",
                        "ArchipelagoPattern",
                        "HighPlainsPattern",
                        "SandstormPattern",
                        "RiverPattern",
                        "MonsoonPattern",
                        "SavannaPattern",
                        "SunPattern",
                        "OceanPattern",
                        "JunglePattern",
                        "FancyPattern",
                        "PokeBallPattern",
                    ][index];

                    Form::from_variant_and_names(variant, names, None)
                }

                // Zygarde
                718 => {
                    // There are distinct forms for Zygarde with Aura Break and
                    // Power Construct, but the names in the data are identical.
                    match index {
                        0 | 1 => {
                            let variant = format!("{}AuraBreak", &names[1]);

                            Form::from_variant_and_names(&variant, names, None)
                        }

                        2 | 3 => {
                            let variant = format!("{}PowerConstruct", &names[1]);

                            Form::from_variant_and_names(&variant, names, None)
                        }

                        _ => Form::from_names(names, None),
                    }
                }

                // Gumshoos, Vikavolt, Ribombee, Araquanid, Lurantis, Salazzle,
                // Togedemaru, Kommo-o
                735 | 738 | 743 | 752 | 754 | 758 | 777 | 784 => match index {
                    // As with Raticate and Marowak, totem forms have blank
                    // names in data.
                    1 => Form::from_variant_and_names("TotemForm", names, None),

                    _ => Form::from_names(names, None),
                },

                // Rockruff
                744 => match index {
                    // Own Tempo Rockruff is a separate form, but has a blank
                    // name.
                    1 => Form::from_variant_and_names("OwnTempo", names, None),

                    _ => Form::from_names(names, None),
                },

                // Minior
                774 => match index {
                    // Each color core has a distinct Meteor Form, but these
                    // forms are identically named.
                    0..=6 => {
                        let variant = [
                            "RedCore",
                            "OrangeCore",
                            "YellowCore",
                            "GreenCore",
                            "BlueCore",
                            "IndigoCore",
                            "VioletCore",
                        ][index];
                        let variant = format!("{name}{variant}", name = &names[1]);

                        Form::from_variant_and_names(&variant, names, None)
                    }

                    _ => Form::from_names(names, None),
                },

                // Mimikyu
                778 => match index {
                    // The totem form of Mimikyu has forms like those of the
                    // standard Mimikyu, but their names are blank in data.
                    2 => Form::from_variant_and_names("DisguisedTotemForm", names, None),
                    3 => Form::from_variant_and_names("BustedTotemForm", names, None),

                    _ => Form::from_names(names, None),
                },

                // Eternatus
                890 => match index {
                    // Eternatus has two Eternamax forms, one which represents a
                    // Gigantamax form, the other of which is used in battles
                    // when using Eternabeam. (???) Their names are identical in
                    // the data.
                    2 => {
                        let variant = format!("{}Gigantamax", &names[1]);

                        Form::from_variant_and_names(&variant, names, None)
                    }

                    _ => Form::from_names(names, None),
                },

                // Urshifu
                892 => match index {
                    // Urshifu has two distinct Gigantamax forms based on its
                    // base form, but the names are identical in data.
                    2 | 3 => {
                        let variant = ["SingleStrike", "RapidStrike"][index - 2];
                        let variant = format!("{variant}{name}", name = &names[1]);

                        Form::from_variant_and_names(&variant, names, None)
                    }

                    _ => Form::from_names(names, None),
                },

                _ => {
                    let gsc_index = if national_dex_id <= 251 && index == 0 {
                        Some(0)
                    } else {
                        None
                    };

                    Form::from_names(names, gsc_index)
                }
            }
        })
        .collect()
}

/// Adjusts the list of form names for a species based on any species-specific
/// requirements.
fn adjust_form_names_list(
    national_dex_id: usize,
    form_names: Vec<Vec<String>>,
    templates: &[String],
) -> Vec<Vec<String>> {
    match national_dex_id {
        25 => {
            // Cosplay Pikachu was present only in ORAS and its form names have
            // since been removed from the main data. Add the names back to the
            // list manually.
            let form_names = form_names.into_iter();

            form_names
                .clone()
                .take(1)
                .chain(
                    COSPLAY_PIKACHU_NAMES
                        .into_iter()
                        .map(|names| names.into_iter().map(String::from).collect())
                        .collect::<Vec<_>>(),
                )
                .chain(form_names.skip(1))
                .collect()
        }
        128 => {
            // Tauros's Paldean forms are built from two strings each using
            // language-specific templates. Rebuild the list with the templated
            // form names.
            let mut form_names = form_names.into_iter();
            let base = form_names.next().unwrap();

            iter::once(base)
                .chain((0..3).map(|_| {
                    let (first, second) = (form_names.next().unwrap(), form_names.next().unwrap());

                    // There is one template per language. Zip together with the
                    // per-language form names lists.
                    templates
                        .iter()
                        .zip(first.into_iter().zip(second))
                        .map(|(template, (first, second))| {
                            template
                                .replace("[VAR 01D4(0000)]", &first)
                                .replace("[VAR 01D4(0001)]", &second)
                        })
                        .collect()
                }))
                .collect()
        }
        172 => {
            // Spiky-eared Pichu was present only in HGSS and its form names are
            // long-since missing from the main data. Re-add them manually.
            form_names
                .into_iter()
                .chain(iter::once(
                    PICHU_NAMES.into_iter().map(String::from).collect(),
                ))
                .collect()
        }

        _ => form_names,
    }
}

// Cosplay Pikachu only appears in ORAS and its form data is no longer present,
// so handle it manually.
static COSPLAY_PIKACHU_NAMES: [[&str; 9]; 6] = [
    [
        "ハードロック・ピカチュウ",
        "Pikachu Rock Star",
        "Pikachu Rockeur",
        "Pikachu rockstar",
        "Rocker-Pikachu",
        "Pikachu Roquera",
        "하드록 피카츄",
        "",
        "",
    ],
    [
        "マダム・ピカチュウ",
        "Pikachu Belle",
        "Pikachu Lady",
        "Pikachu damigella",
        "Damen-Pikachu",
        "Pikachu Aristócrata",
        "마담 피카츄",
        "",
        "",
    ],
    [
        "アイドル・ピカチュウ",
        "Pikachu Pop Star",
        "Pikachu Star",
        "Pikachu confetto",
        "Star-Pikachu",
        "Pikachu Superstar",
        "아이돌 피카츄",
        "",
        "",
    ],
    [
        "ドクター・ピカチュウ",
        "Pikachu, Ph.D.",
        "Pikachu Docteur",
        "Pikachu scienziata",
        "Professoren-Pikachu",
        "Pikachu Erudita",
        "닥터 피카츄",
        "",
        "",
    ],
    [
        "マスク・ド・ピカチュウ",
        "Pikachu Libre",
        "Pikachu Catcheur",
        "Pikachu wrestler",
        "Wrestler-Pikachu",
        "Pikachu Enmascarada",
        "마스크드 피카츄",
        "",
        "",
    ],
    [
        "おきがえピカチュウ",
        "Cosplay Pikachu",
        "Pikachu Cosplayeur",
        "Pikachu Cosplay",
        "Cosplay-Pikachu",
        "Pikachu Coqueta",
        "옷갈아입기 피카츄",
        "",
        "",
    ],
];

// Spiky-eared Pichu only appears in HGSS and its form data is no longer
// present, so handle it manually.
static PICHU_NAMES: [&str; 9] = [
    "ギザみみピチュー",
    "Spiky-eared Pichu",
    "Pichu Troizépi",
    "Pichu Spunzorek",
    "Strubbelohr-Pichu",
    "Pichu Picoreja",
    "삐쭉귀 피츄",
    "",
    "",
];

/// `FormNamesSpecifier` maps a single species to the form names in the
/// `zkn_form` table.
///
/// At present, the file is structured such that, for forms introduced before
/// Gen 9, a single base form appears in the list for each species, with
/// alternate forms following the base forms for the species introduced before
/// Gen 9. Forms introduced in Gen 9 are inline after the species they belong
/// to, with species introduced in Gen 9 following the list of alternate forms.
struct FormNamesSpecifier {
    /// The count of forms in the "main list", including base forms for
    /// pre-Gen 9 species and all forms for Gen 9 species or species with forms
    /// forms introduced in Gen 9.
    main_forms: usize,

    /// The count of forms in the "alt list" after pre-Gen 9 base forms.
    alt_forms: usize,
}
