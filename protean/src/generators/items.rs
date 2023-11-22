use std::{io, path::Path};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{build_names_table, load_names, sanitize_variant_name};

use super::write_generated_file;

pub fn generate_items_data(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    let names = load_names(source_dir, "itemname");

    let (variants, data): (TokenStream, TokenStream) = names
        .chain(REMOVED_ITEMS.map(|names| names.into_iter().map(String::from).collect()))
        .enumerate()
        .skip(1)
        .filter_map(|(index, names)| {
            let english_name = &names[1];

            if english_name == "???"
                || english_name.is_empty()
                || (0x026a..=0x026c).contains(&index)
                || (0x02b2..=0x02b6).contains(&index)
                || index == 0x0438
                || index == 0x0647
                || index == 0x0656
                || (0x06e5..=0x06e9).contains(&index)
                || index == 0x0870
            {
                return None;
            }

            let variant = {
                let variant = sanitize_variant_name(english_name);

                if index == 0x0079 {
                    format_ident!("PokemonBox")
                } else if index == 0x026d || index == 0x0281 || index == 0x02e3 {
                    format_ident!("{variant}Male")
                } else if index == 0x0272 || index == 0x02ca || index == 0x02e4 {
                    format_ident!("{variant}Female")
                } else if index == 0x0274 || index == 0x03af || index == 0x03b0 || index == 0x0636 {
                    format_ident!("{variant}NotFused")
                } else if index == 0x0275 || index == 0x03b1 || index == 0x03b2 || index == 0x0637 {
                    format_ident!("{variant}Fused")
                } else if index == 0x01c2 {
                    format_ident!("{variant}Green")
                } else if index == 0x02c9 {
                    format_ident!("{variant}Yellow")
                } else if index == 0x027c {
                    format_ident!("{variant}Curtis")
                } else if index == 0x027d {
                    format_ident!("{variant}Yancy")
                } else if index == 0x01c8 || index == 0x01db || index == 0x01dc {
                    format_ident!("{variant}Johto")
                } else if index == 0x02d3 || index == 0x02de || index == 0x02e0 {
                    format_ident!("{variant}Hoenn")
                } else if index == 0x01cb || index == 0x01cf || index == 0x01d3 {
                    format_ident!("{variant}Sinnoh")
                } else if index == 0x0368 || index == 0x0369 || index == 0x036b || index == 0x036c {
                    format_ident!("{variant}Kanto")
                } else if index == 0x02d9 {
                    format_ident!("{variant}FirstForm")
                } else if index == 0x02ef {
                    format_ident!("{variant}SecondForm")
                } else if index == 0x0303 {
                    format_ident!("{variant}ThirdForm")
                } else if index == 0x0304 {
                    format_ident!("{variant}FinalForm")
                } else if (0x0308..=0x31a).contains(&index)
                    || (0x031e..=0x0326).contains(&index)
                    || index == 0x0344
                    || (0x0399..=0x039e).contains(&index)
                {
                    format_ident!("{variant}Held")
                } else if (0x0327..=0x0342).contains(&index) || (0x039f..=0x03a4).contains(&index) {
                    format_ident!("{variant}Bag")
                } else if (0x0668..=0x066c).contains(&index)
                    || (0x066e..=0x0672).contains(&index)
                    || (0x0674..=0x0687).contains(&index)
                    || (0x0689..=0x068d).contains(&index)
                    || (0x06c1..=0x06c3).contains(&index)
                    || (0x06d7..=0x06d9).contains(&index)
                    || (0x06f7..=0x06f8).contains(&index)
                {
                    format_ident!("{variant}Recipe")
                } else if (0x06ae..=0x06b1).contains(&index)
                    || index == 0x0650
                    || index == 0x0699
                    || index == 0x06d4
                    || index == 0x06e3
                {
                    format_ident!("{variant}Hisui")
                } else if index == 0x037e {
                    format_ident!("{variant}Pikachu")
                } else if index == 0x037f {
                    format_ident!("{variant}Eevee")
                } else if index == 0x0439 {
                    format_ident!("{variant}LandMode")
                } else if index == 0x04f2 {
                    format_ident!("{variant}WaterMode")
                } else if index == 0x0631 {
                    format_ident!("{variant}SparklingWhite")
                } else if index == 0x0632 {
                    format_ident!("{variant}GlisteningBlack")
                } else if index == 0x0909
                    || index == 0x090e
                    || index == 0x0913
                    || index == 0x0919
                    || index == 0x0972
                {
                    // XXX: Verify that these are in the right order.
                    format_ident!("{variant}Naranja")
                } else if index == 0x090a
                    || index == 0x090f
                    || index == 0x0914
                    || index == 0x091a
                    || index == 0x0973
                {
                    format_ident!("{variant}Uva")
                } else {
                    variant
                }
            };

            let names_data = build_names_table(&names);

            Some((
                quote!(#variant,),
                quote!(Item::#variant => &ItemData {
                    names: #names_data,
                },),
            ))
        })
        .unzip();

    let tokens = quote!(
        use crate::{ItemData, NamesData};

        #[derive(Clone, Copy, Debug)]
        pub enum Item {
            #variants
        }

        impl Item {
            pub(crate) fn data(&self) -> &'static ItemData {
                match self {
                    #data
                }
            }
        }
    );

    write_generated_file(&dest_dir.join("items.rs"), tokens)
}

const REMOVED_ITEMS: [[&str; 9]; 9] = [
    [
        "ひきかえけん",
        "Bike Voucher",
        "Bon Commande",
        "Buono Bici",
        "Rad-Coupon",
        "Bono Bici",
        "",
        "",
        "",
    ],
    [
        "スぺシャルアップ",
        "X Special",
        "Special +",
        "Special X",
        "X-Spezial",
        "Especial X",
        "스페셜 업",
        "",
        "",
    ],
    [
        "ひでんマシン０１",
        "HM01",
        "CS01",
        "MN01",
        "VM01",
        "MO01",
        "비전머신01",
        "秘传学习器０１",
        "秘傳學習器０１",
    ],
    [
        "ひでんマシン０２",
        "HM02",
        "CS02",
        "MN02",
        "VM02",
        "MO02",
        "비전머신02",
        "秘传学习器０２",
        "秘傳學習器０２",
    ],
    [
        "ひでんマシン０３",
        "HM03",
        "CS03",
        "MN03",
        "VM03",
        "MO03",
        "비전머신03",
        "秘传学习器０３",
        "秘傳學習器０３",
    ],
    [
        "ひでんマシン０４",
        "HM04",
        "CS04",
        "MN04",
        "VM04",
        "MO04",
        "비전머신04",
        "秘传学习器０４",
        "秘傳學習器０４",
    ],
    [
        "ひでんマシン０５",
        "HM05",
        "CS05",
        "MN05",
        "VM05",
        "MO05",
        "비전머신05",
        "秘传学习器０５",
        "秘傳學習器０５",
    ],
    [
        "ひでんマシン０６",
        "HM06",
        "CS06",
        "MN06",
        "VM06",
        "MO06",
        "비전머신06",
        "秘传学习器０６",
        "秘傳學習器０６",
    ],
    [
        "ひでんマシン０８",
        "HM08",
        "CS08",
        "MN08",
        "VM08",
        "MO08",
        "비전머신08",
        "秘传学习器０８",
        "秘傳學習器０８",
    ],
];
