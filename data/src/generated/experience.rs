// GENERATED CONTENT; DO NOT EDIT. See the `protean` crate.
use common::Error;
pub(crate) const fn min_experience_for_level_erratic(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 15,
        3 => 52,
        4 => 122,
        5 => 237,
        6 => 406,
        7 => 637,
        8 => 942,
        9 => 1326,
        10 => 1800,
        11 => 2369,
        12 => 3041,
        13 => 3822,
        14 => 4719,
        15 => 5737,
        16 => 6881,
        17 => 8155,
        18 => 9564,
        19 => 11111,
        20 => 12800,
        21 => 14632,
        22 => 16610,
        23 => 18737,
        24 => 21012,
        25 => 23437,
        26 => 26012,
        27 => 28737,
        28 => 31610,
        29 => 34632,
        30 => 37800,
        31 => 41111,
        32 => 44564,
        33 => 48155,
        34 => 51881,
        35 => 55737,
        36 => 59719,
        37 => 63822,
        38 => 68041,
        39 => 72369,
        40 => 76800,
        41 => 81326,
        42 => 85942,
        43 => 90637,
        44 => 95406,
        45 => 100237,
        46 => 105122,
        47 => 110052,
        48 => 115015,
        49 => 120001,
        50 => 125000,
        51 => 131324,
        52 => 137795,
        53 => 144410,
        54 => 151165,
        55 => 158056,
        56 => 165079,
        57 => 172229,
        58 => 179503,
        59 => 186894,
        60 => 194400,
        61 => 202013,
        62 => 209728,
        63 => 217540,
        64 => 225443,
        65 => 233431,
        66 => 241496,
        67 => 249633,
        68 => 257834,
        69 => 267406,
        70 => 276458,
        71 => 286328,
        72 => 296358,
        73 => 305767,
        74 => 316074,
        75 => 326531,
        76 => 336255,
        77 => 346965,
        78 => 357812,
        79 => 367807,
        80 => 378880,
        81 => 390077,
        82 => 400293,
        83 => 411686,
        84 => 423190,
        85 => 433572,
        86 => 445239,
        87 => 457001,
        88 => 467489,
        89 => 479378,
        90 => 491346,
        91 => 501878,
        92 => 513934,
        93 => 526049,
        94 => 536557,
        95 => 548720,
        96 => 560922,
        97 => 571333,
        98 => 583539,
        99 => 591882,
        100 => 600000,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_erratic(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=14 => 1,
        15..=51 => 2,
        52..=121 => 3,
        122..=236 => 4,
        237..=405 => 5,
        406..=636 => 6,
        637..=941 => 7,
        942..=1325 => 8,
        1326..=1799 => 9,
        1800..=2368 => 10,
        2369..=3040 => 11,
        3041..=3821 => 12,
        3822..=4718 => 13,
        4719..=5736 => 14,
        5737..=6880 => 15,
        6881..=8154 => 16,
        8155..=9563 => 17,
        9564..=11110 => 18,
        11111..=12799 => 19,
        12800..=14631 => 20,
        14632..=16609 => 21,
        16610..=18736 => 22,
        18737..=21011 => 23,
        21012..=23436 => 24,
        23437..=26011 => 25,
        26012..=28736 => 26,
        28737..=31609 => 27,
        31610..=34631 => 28,
        34632..=37799 => 29,
        37800..=41110 => 30,
        41111..=44563 => 31,
        44564..=48154 => 32,
        48155..=51880 => 33,
        51881..=55736 => 34,
        55737..=59718 => 35,
        59719..=63821 => 36,
        63822..=68040 => 37,
        68041..=72368 => 38,
        72369..=76799 => 39,
        76800..=81325 => 40,
        81326..=85941 => 41,
        85942..=90636 => 42,
        90637..=95405 => 43,
        95406..=100236 => 44,
        100237..=105121 => 45,
        105122..=110051 => 46,
        110052..=115014 => 47,
        115015..=120000 => 48,
        120001..=124999 => 49,
        125000..=131323 => 50,
        131324..=137794 => 51,
        137795..=144409 => 52,
        144410..=151164 => 53,
        151165..=158055 => 54,
        158056..=165078 => 55,
        165079..=172228 => 56,
        172229..=179502 => 57,
        179503..=186893 => 58,
        186894..=194399 => 59,
        194400..=202012 => 60,
        202013..=209727 => 61,
        209728..=217539 => 62,
        217540..=225442 => 63,
        225443..=233430 => 64,
        233431..=241495 => 65,
        241496..=249632 => 66,
        249633..=257833 => 67,
        257834..=267405 => 68,
        267406..=276457 => 69,
        276458..=286327 => 70,
        286328..=296357 => 71,
        296358..=305766 => 72,
        305767..=316073 => 73,
        316074..=326530 => 74,
        326531..=336254 => 75,
        336255..=346964 => 76,
        346965..=357811 => 77,
        357812..=367806 => 78,
        367807..=378879 => 79,
        378880..=390076 => 80,
        390077..=400292 => 81,
        400293..=411685 => 82,
        411686..=423189 => 83,
        423190..=433571 => 84,
        433572..=445238 => 85,
        445239..=457000 => 86,
        457001..=467488 => 87,
        467489..=479377 => 88,
        479378..=491345 => 89,
        491346..=501877 => 90,
        501878..=513933 => 91,
        513934..=526048 => 92,
        526049..=536556 => 93,
        536557..=548719 => 94,
        548720..=560921 => 95,
        560922..=571332 => 96,
        571333..=583538 => 97,
        583539..=591881 => 98,
        591882..=599999 => 99,
        600000 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
pub(crate) const fn min_experience_for_level_fast(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 6,
        3 => 21,
        4 => 51,
        5 => 100,
        6 => 172,
        7 => 274,
        8 => 409,
        9 => 583,
        10 => 800,
        11 => 1064,
        12 => 1382,
        13 => 1757,
        14 => 2195,
        15 => 2700,
        16 => 3276,
        17 => 3930,
        18 => 4665,
        19 => 5487,
        20 => 6400,
        21 => 7408,
        22 => 8518,
        23 => 9733,
        24 => 11059,
        25 => 12500,
        26 => 14060,
        27 => 15746,
        28 => 17561,
        29 => 19511,
        30 => 21600,
        31 => 23832,
        32 => 26214,
        33 => 28749,
        34 => 31443,
        35 => 34300,
        36 => 37324,
        37 => 40522,
        38 => 43897,
        39 => 47455,
        40 => 51200,
        41 => 55136,
        42 => 59270,
        43 => 63605,
        44 => 68147,
        45 => 72900,
        46 => 77868,
        47 => 83058,
        48 => 88473,
        49 => 94119,
        50 => 100000,
        51 => 106120,
        52 => 112486,
        53 => 119101,
        54 => 125971,
        55 => 133100,
        56 => 140492,
        57 => 148154,
        58 => 156089,
        59 => 164303,
        60 => 172800,
        61 => 181584,
        62 => 190662,
        63 => 200037,
        64 => 209715,
        65 => 219700,
        66 => 229996,
        67 => 240610,
        68 => 251545,
        69 => 262807,
        70 => 274400,
        71 => 286328,
        72 => 298598,
        73 => 311213,
        74 => 324179,
        75 => 337500,
        76 => 351180,
        77 => 365226,
        78 => 379641,
        79 => 394431,
        80 => 409600,
        81 => 425152,
        82 => 441094,
        83 => 457429,
        84 => 474163,
        85 => 491300,
        86 => 508844,
        87 => 526802,
        88 => 545177,
        89 => 563975,
        90 => 583200,
        91 => 602856,
        92 => 622950,
        93 => 643485,
        94 => 664467,
        95 => 685900,
        96 => 707788,
        97 => 730138,
        98 => 752953,
        99 => 776239,
        100 => 800000,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_fast(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=5 => 1,
        6..=20 => 2,
        21..=50 => 3,
        51..=99 => 4,
        100..=171 => 5,
        172..=273 => 6,
        274..=408 => 7,
        409..=582 => 8,
        583..=799 => 9,
        800..=1063 => 10,
        1064..=1381 => 11,
        1382..=1756 => 12,
        1757..=2194 => 13,
        2195..=2699 => 14,
        2700..=3275 => 15,
        3276..=3929 => 16,
        3930..=4664 => 17,
        4665..=5486 => 18,
        5487..=6399 => 19,
        6400..=7407 => 20,
        7408..=8517 => 21,
        8518..=9732 => 22,
        9733..=11058 => 23,
        11059..=12499 => 24,
        12500..=14059 => 25,
        14060..=15745 => 26,
        15746..=17560 => 27,
        17561..=19510 => 28,
        19511..=21599 => 29,
        21600..=23831 => 30,
        23832..=26213 => 31,
        26214..=28748 => 32,
        28749..=31442 => 33,
        31443..=34299 => 34,
        34300..=37323 => 35,
        37324..=40521 => 36,
        40522..=43896 => 37,
        43897..=47454 => 38,
        47455..=51199 => 39,
        51200..=55135 => 40,
        55136..=59269 => 41,
        59270..=63604 => 42,
        63605..=68146 => 43,
        68147..=72899 => 44,
        72900..=77867 => 45,
        77868..=83057 => 46,
        83058..=88472 => 47,
        88473..=94118 => 48,
        94119..=99999 => 49,
        100000..=106119 => 50,
        106120..=112485 => 51,
        112486..=119100 => 52,
        119101..=125970 => 53,
        125971..=133099 => 54,
        133100..=140491 => 55,
        140492..=148153 => 56,
        148154..=156088 => 57,
        156089..=164302 => 58,
        164303..=172799 => 59,
        172800..=181583 => 60,
        181584..=190661 => 61,
        190662..=200036 => 62,
        200037..=209714 => 63,
        209715..=219699 => 64,
        219700..=229995 => 65,
        229996..=240609 => 66,
        240610..=251544 => 67,
        251545..=262806 => 68,
        262807..=274399 => 69,
        274400..=286327 => 70,
        286328..=298597 => 71,
        298598..=311212 => 72,
        311213..=324178 => 73,
        324179..=337499 => 74,
        337500..=351179 => 75,
        351180..=365225 => 76,
        365226..=379640 => 77,
        379641..=394430 => 78,
        394431..=409599 => 79,
        409600..=425151 => 80,
        425152..=441093 => 81,
        441094..=457428 => 82,
        457429..=474162 => 83,
        474163..=491299 => 84,
        491300..=508843 => 85,
        508844..=526801 => 86,
        526802..=545176 => 87,
        545177..=563974 => 88,
        563975..=583199 => 89,
        583200..=602855 => 90,
        602856..=622949 => 91,
        622950..=643484 => 92,
        643485..=664466 => 93,
        664467..=685899 => 94,
        685900..=707787 => 95,
        707788..=730137 => 96,
        730138..=752952 => 97,
        752953..=776238 => 98,
        776239..=799999 => 99,
        800000 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
pub(crate) const fn min_experience_for_level_medium_fast(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 8,
        3 => 27,
        4 => 64,
        5 => 125,
        6 => 216,
        7 => 343,
        8 => 512,
        9 => 729,
        10 => 1000,
        11 => 1331,
        12 => 1728,
        13 => 2197,
        14 => 2744,
        15 => 3375,
        16 => 4096,
        17 => 4913,
        18 => 5832,
        19 => 6859,
        20 => 8000,
        21 => 9261,
        22 => 10648,
        23 => 12167,
        24 => 13824,
        25 => 15625,
        26 => 17576,
        27 => 19683,
        28 => 21952,
        29 => 24389,
        30 => 27000,
        31 => 29791,
        32 => 32768,
        33 => 35937,
        34 => 39304,
        35 => 42875,
        36 => 46656,
        37 => 50653,
        38 => 54872,
        39 => 59319,
        40 => 64000,
        41 => 68921,
        42 => 74088,
        43 => 79507,
        44 => 85184,
        45 => 91125,
        46 => 97336,
        47 => 103823,
        48 => 110592,
        49 => 117649,
        50 => 125000,
        51 => 132651,
        52 => 140608,
        53 => 148877,
        54 => 157464,
        55 => 166375,
        56 => 175616,
        57 => 185193,
        58 => 195112,
        59 => 205379,
        60 => 216000,
        61 => 226981,
        62 => 238328,
        63 => 250047,
        64 => 262144,
        65 => 274625,
        66 => 287496,
        67 => 300763,
        68 => 314432,
        69 => 328509,
        70 => 343000,
        71 => 357911,
        72 => 373248,
        73 => 389017,
        74 => 405224,
        75 => 421875,
        76 => 438976,
        77 => 456533,
        78 => 474552,
        79 => 493039,
        80 => 512000,
        81 => 531441,
        82 => 551368,
        83 => 571787,
        84 => 592704,
        85 => 614125,
        86 => 636056,
        87 => 658503,
        88 => 681472,
        89 => 704969,
        90 => 729000,
        91 => 753571,
        92 => 778688,
        93 => 804357,
        94 => 830584,
        95 => 857375,
        96 => 884736,
        97 => 912673,
        98 => 941192,
        99 => 970299,
        100 => 1000000,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_medium_fast(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=7 => 1,
        8..=26 => 2,
        27..=63 => 3,
        64..=124 => 4,
        125..=215 => 5,
        216..=342 => 6,
        343..=511 => 7,
        512..=728 => 8,
        729..=999 => 9,
        1000..=1330 => 10,
        1331..=1727 => 11,
        1728..=2196 => 12,
        2197..=2743 => 13,
        2744..=3374 => 14,
        3375..=4095 => 15,
        4096..=4912 => 16,
        4913..=5831 => 17,
        5832..=6858 => 18,
        6859..=7999 => 19,
        8000..=9260 => 20,
        9261..=10647 => 21,
        10648..=12166 => 22,
        12167..=13823 => 23,
        13824..=15624 => 24,
        15625..=17575 => 25,
        17576..=19682 => 26,
        19683..=21951 => 27,
        21952..=24388 => 28,
        24389..=26999 => 29,
        27000..=29790 => 30,
        29791..=32767 => 31,
        32768..=35936 => 32,
        35937..=39303 => 33,
        39304..=42874 => 34,
        42875..=46655 => 35,
        46656..=50652 => 36,
        50653..=54871 => 37,
        54872..=59318 => 38,
        59319..=63999 => 39,
        64000..=68920 => 40,
        68921..=74087 => 41,
        74088..=79506 => 42,
        79507..=85183 => 43,
        85184..=91124 => 44,
        91125..=97335 => 45,
        97336..=103822 => 46,
        103823..=110591 => 47,
        110592..=117648 => 48,
        117649..=124999 => 49,
        125000..=132650 => 50,
        132651..=140607 => 51,
        140608..=148876 => 52,
        148877..=157463 => 53,
        157464..=166374 => 54,
        166375..=175615 => 55,
        175616..=185192 => 56,
        185193..=195111 => 57,
        195112..=205378 => 58,
        205379..=215999 => 59,
        216000..=226980 => 60,
        226981..=238327 => 61,
        238328..=250046 => 62,
        250047..=262143 => 63,
        262144..=274624 => 64,
        274625..=287495 => 65,
        287496..=300762 => 66,
        300763..=314431 => 67,
        314432..=328508 => 68,
        328509..=342999 => 69,
        343000..=357910 => 70,
        357911..=373247 => 71,
        373248..=389016 => 72,
        389017..=405223 => 73,
        405224..=421874 => 74,
        421875..=438975 => 75,
        438976..=456532 => 76,
        456533..=474551 => 77,
        474552..=493038 => 78,
        493039..=511999 => 79,
        512000..=531440 => 80,
        531441..=551367 => 81,
        551368..=571786 => 82,
        571787..=592703 => 83,
        592704..=614124 => 84,
        614125..=636055 => 85,
        636056..=658502 => 86,
        658503..=681471 => 87,
        681472..=704968 => 88,
        704969..=728999 => 89,
        729000..=753570 => 90,
        753571..=778687 => 91,
        778688..=804356 => 92,
        804357..=830583 => 93,
        830584..=857374 => 94,
        857375..=884735 => 95,
        884736..=912672 => 96,
        912673..=941191 => 97,
        941192..=970298 => 98,
        970299..=999999 => 99,
        1000000 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
pub(crate) const fn min_experience_for_level_medium_slow(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 9,
        3 => 57,
        4 => 96,
        5 => 135,
        6 => 179,
        7 => 236,
        8 => 314,
        9 => 419,
        10 => 560,
        11 => 742,
        12 => 973,
        13 => 1261,
        14 => 1612,
        15 => 2035,
        16 => 2535,
        17 => 3120,
        18 => 3798,
        19 => 4575,
        20 => 5460,
        21 => 6458,
        22 => 7577,
        23 => 8825,
        24 => 10208,
        25 => 11735,
        26 => 13411,
        27 => 15244,
        28 => 17242,
        29 => 19411,
        30 => 21760,
        31 => 24294,
        32 => 27021,
        33 => 29949,
        34 => 33084,
        35 => 36435,
        36 => 40007,
        37 => 43808,
        38 => 47846,
        39 => 52127,
        40 => 56660,
        41 => 61450,
        42 => 66505,
        43 => 71833,
        44 => 77440,
        45 => 83335,
        46 => 89523,
        47 => 96012,
        48 => 102810,
        49 => 109923,
        50 => 117360,
        51 => 125126,
        52 => 133229,
        53 => 141677,
        54 => 150476,
        55 => 159635,
        56 => 169159,
        57 => 179056,
        58 => 189334,
        59 => 199999,
        60 => 211060,
        61 => 222522,
        62 => 234393,
        63 => 246681,
        64 => 259392,
        65 => 272535,
        66 => 286115,
        67 => 300140,
        68 => 314618,
        69 => 329555,
        70 => 344960,
        71 => 360838,
        72 => 377197,
        73 => 394045,
        74 => 411388,
        75 => 429235,
        76 => 447591,
        77 => 466464,
        78 => 485862,
        79 => 505791,
        80 => 526260,
        81 => 547274,
        82 => 568841,
        83 => 590969,
        84 => 613664,
        85 => 636935,
        86 => 660787,
        87 => 685228,
        88 => 710266,
        89 => 735907,
        90 => 762160,
        91 => 789030,
        92 => 816525,
        93 => 844653,
        94 => 873420,
        95 => 902835,
        96 => 932903,
        97 => 963632,
        98 => 995030,
        99 => 1027103,
        100 => 1059860,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_medium_slow(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=8 => 1,
        9..=56 => 2,
        57..=95 => 3,
        96..=134 => 4,
        135..=178 => 5,
        179..=235 => 6,
        236..=313 => 7,
        314..=418 => 8,
        419..=559 => 9,
        560..=741 => 10,
        742..=972 => 11,
        973..=1260 => 12,
        1261..=1611 => 13,
        1612..=2034 => 14,
        2035..=2534 => 15,
        2535..=3119 => 16,
        3120..=3797 => 17,
        3798..=4574 => 18,
        4575..=5459 => 19,
        5460..=6457 => 20,
        6458..=7576 => 21,
        7577..=8824 => 22,
        8825..=10207 => 23,
        10208..=11734 => 24,
        11735..=13410 => 25,
        13411..=15243 => 26,
        15244..=17241 => 27,
        17242..=19410 => 28,
        19411..=21759 => 29,
        21760..=24293 => 30,
        24294..=27020 => 31,
        27021..=29948 => 32,
        29949..=33083 => 33,
        33084..=36434 => 34,
        36435..=40006 => 35,
        40007..=43807 => 36,
        43808..=47845 => 37,
        47846..=52126 => 38,
        52127..=56659 => 39,
        56660..=61449 => 40,
        61450..=66504 => 41,
        66505..=71832 => 42,
        71833..=77439 => 43,
        77440..=83334 => 44,
        83335..=89522 => 45,
        89523..=96011 => 46,
        96012..=102809 => 47,
        102810..=109922 => 48,
        109923..=117359 => 49,
        117360..=125125 => 50,
        125126..=133228 => 51,
        133229..=141676 => 52,
        141677..=150475 => 53,
        150476..=159634 => 54,
        159635..=169158 => 55,
        169159..=179055 => 56,
        179056..=189333 => 57,
        189334..=199998 => 58,
        199999..=211059 => 59,
        211060..=222521 => 60,
        222522..=234392 => 61,
        234393..=246680 => 62,
        246681..=259391 => 63,
        259392..=272534 => 64,
        272535..=286114 => 65,
        286115..=300139 => 66,
        300140..=314617 => 67,
        314618..=329554 => 68,
        329555..=344959 => 69,
        344960..=360837 => 70,
        360838..=377196 => 71,
        377197..=394044 => 72,
        394045..=411387 => 73,
        411388..=429234 => 74,
        429235..=447590 => 75,
        447591..=466463 => 76,
        466464..=485861 => 77,
        485862..=505790 => 78,
        505791..=526259 => 79,
        526260..=547273 => 80,
        547274..=568840 => 81,
        568841..=590968 => 82,
        590969..=613663 => 83,
        613664..=636934 => 84,
        636935..=660786 => 85,
        660787..=685227 => 86,
        685228..=710265 => 87,
        710266..=735906 => 88,
        735907..=762159 => 89,
        762160..=789029 => 90,
        789030..=816524 => 91,
        816525..=844652 => 92,
        844653..=873419 => 93,
        873420..=902834 => 94,
        902835..=932902 => 95,
        932903..=963631 => 96,
        963632..=995029 => 97,
        995030..=1027102 => 98,
        1027103..=1059859 => 99,
        1059860 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
pub(crate) const fn min_experience_for_level_slow(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 10,
        3 => 33,
        4 => 80,
        5 => 156,
        6 => 270,
        7 => 428,
        8 => 640,
        9 => 911,
        10 => 1250,
        11 => 1663,
        12 => 2160,
        13 => 2746,
        14 => 3430,
        15 => 4218,
        16 => 5120,
        17 => 6141,
        18 => 7290,
        19 => 8573,
        20 => 10000,
        21 => 11576,
        22 => 13310,
        23 => 15208,
        24 => 17280,
        25 => 19531,
        26 => 21970,
        27 => 24603,
        28 => 27440,
        29 => 30486,
        30 => 33750,
        31 => 37238,
        32 => 40960,
        33 => 44921,
        34 => 49130,
        35 => 53593,
        36 => 58320,
        37 => 63316,
        38 => 68590,
        39 => 74148,
        40 => 80000,
        41 => 86151,
        42 => 92610,
        43 => 99383,
        44 => 106480,
        45 => 113906,
        46 => 121670,
        47 => 129778,
        48 => 138240,
        49 => 147061,
        50 => 156250,
        51 => 165813,
        52 => 175760,
        53 => 186096,
        54 => 196830,
        55 => 207968,
        56 => 219520,
        57 => 231491,
        58 => 243890,
        59 => 256723,
        60 => 270000,
        61 => 283726,
        62 => 297910,
        63 => 312558,
        64 => 327680,
        65 => 343281,
        66 => 359370,
        67 => 375953,
        68 => 393040,
        69 => 410636,
        70 => 428750,
        71 => 447388,
        72 => 466560,
        73 => 486271,
        74 => 506530,
        75 => 527343,
        76 => 548720,
        77 => 570666,
        78 => 593190,
        79 => 616298,
        80 => 640000,
        81 => 664301,
        82 => 689210,
        83 => 714733,
        84 => 740880,
        85 => 767656,
        86 => 795070,
        87 => 823128,
        88 => 851840,
        89 => 881211,
        90 => 911250,
        91 => 941963,
        92 => 973360,
        93 => 1005446,
        94 => 1038230,
        95 => 1071718,
        96 => 1105920,
        97 => 1140841,
        98 => 1176490,
        99 => 1212873,
        100 => 1250000,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_slow(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=9 => 1,
        10..=32 => 2,
        33..=79 => 3,
        80..=155 => 4,
        156..=269 => 5,
        270..=427 => 6,
        428..=639 => 7,
        640..=910 => 8,
        911..=1249 => 9,
        1250..=1662 => 10,
        1663..=2159 => 11,
        2160..=2745 => 12,
        2746..=3429 => 13,
        3430..=4217 => 14,
        4218..=5119 => 15,
        5120..=6140 => 16,
        6141..=7289 => 17,
        7290..=8572 => 18,
        8573..=9999 => 19,
        10000..=11575 => 20,
        11576..=13309 => 21,
        13310..=15207 => 22,
        15208..=17279 => 23,
        17280..=19530 => 24,
        19531..=21969 => 25,
        21970..=24602 => 26,
        24603..=27439 => 27,
        27440..=30485 => 28,
        30486..=33749 => 29,
        33750..=37237 => 30,
        37238..=40959 => 31,
        40960..=44920 => 32,
        44921..=49129 => 33,
        49130..=53592 => 34,
        53593..=58319 => 35,
        58320..=63315 => 36,
        63316..=68589 => 37,
        68590..=74147 => 38,
        74148..=79999 => 39,
        80000..=86150 => 40,
        86151..=92609 => 41,
        92610..=99382 => 42,
        99383..=106479 => 43,
        106480..=113905 => 44,
        113906..=121669 => 45,
        121670..=129777 => 46,
        129778..=138239 => 47,
        138240..=147060 => 48,
        147061..=156249 => 49,
        156250..=165812 => 50,
        165813..=175759 => 51,
        175760..=186095 => 52,
        186096..=196829 => 53,
        196830..=207967 => 54,
        207968..=219519 => 55,
        219520..=231490 => 56,
        231491..=243889 => 57,
        243890..=256722 => 58,
        256723..=269999 => 59,
        270000..=283725 => 60,
        283726..=297909 => 61,
        297910..=312557 => 62,
        312558..=327679 => 63,
        327680..=343280 => 64,
        343281..=359369 => 65,
        359370..=375952 => 66,
        375953..=393039 => 67,
        393040..=410635 => 68,
        410636..=428749 => 69,
        428750..=447387 => 70,
        447388..=466559 => 71,
        466560..=486270 => 72,
        486271..=506529 => 73,
        506530..=527342 => 74,
        527343..=548719 => 75,
        548720..=570665 => 76,
        570666..=593189 => 77,
        593190..=616297 => 78,
        616298..=639999 => 79,
        640000..=664300 => 80,
        664301..=689209 => 81,
        689210..=714732 => 82,
        714733..=740879 => 83,
        740880..=767655 => 84,
        767656..=795069 => 85,
        795070..=823127 => 86,
        823128..=851839 => 87,
        851840..=881210 => 88,
        881211..=911249 => 89,
        911250..=941962 => 90,
        941963..=973359 => 91,
        973360..=1005445 => 92,
        1005446..=1038229 => 93,
        1038230..=1071717 => 94,
        1071718..=1105919 => 95,
        1105920..=1140840 => 96,
        1140841..=1176489 => 97,
        1176490..=1212872 => 98,
        1212873..=1249999 => 99,
        1250000 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
pub(crate) const fn min_experience_for_level_fluctuating(level: u8) -> Result<u32, Error> {
    let experience = match level {
        1 => 0,
        2 => 4,
        3 => 13,
        4 => 32,
        5 => 65,
        6 => 112,
        7 => 178,
        8 => 276,
        9 => 393,
        10 => 540,
        11 => 745,
        12 => 967,
        13 => 1230,
        14 => 1591,
        15 => 1957,
        16 => 2457,
        17 => 3046,
        18 => 3732,
        19 => 4526,
        20 => 5440,
        21 => 6482,
        22 => 7666,
        23 => 9003,
        24 => 10506,
        25 => 12187,
        26 => 14060,
        27 => 16140,
        28 => 18439,
        29 => 20974,
        30 => 23760,
        31 => 26811,
        32 => 30146,
        33 => 33780,
        34 => 37731,
        35 => 42017,
        36 => 46656,
        37 => 50653,
        38 => 55969,
        39 => 60505,
        40 => 66560,
        41 => 71677,
        42 => 78533,
        43 => 84277,
        44 => 91998,
        45 => 98415,
        46 => 107069,
        47 => 114205,
        48 => 123863,
        49 => 131766,
        50 => 142500,
        51 => 151222,
        52 => 163105,
        53 => 172697,
        54 => 185807,
        55 => 196322,
        56 => 210739,
        57 => 222231,
        58 => 238036,
        59 => 250562,
        60 => 267840,
        61 => 281456,
        62 => 300293,
        63 => 315059,
        64 => 335544,
        65 => 351520,
        66 => 373744,
        67 => 390991,
        68 => 415050,
        69 => 433631,
        70 => 459620,
        71 => 479600,
        72 => 507617,
        73 => 529063,
        74 => 559209,
        75 => 582187,
        76 => 614566,
        77 => 639146,
        78 => 673863,
        79 => 700115,
        80 => 737280,
        81 => 765275,
        82 => 804997,
        83 => 834809,
        84 => 877201,
        85 => 908905,
        86 => 954084,
        87 => 987754,
        88 => 1035837,
        89 => 1071552,
        90 => 1122660,
        91 => 1160499,
        92 => 1214753,
        93 => 1254796,
        94 => 1312322,
        95 => 1354652,
        96 => 1415577,
        97 => 1460276,
        98 => 1524731,
        99 => 1571884,
        100 => 1640000,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(experience)
}
pub(crate) const fn calculate_level_fluctuating(experience: u32) -> Result<u8, Error> {
    let level = match experience {
        0..=3 => 1,
        4..=12 => 2,
        13..=31 => 3,
        32..=64 => 4,
        65..=111 => 5,
        112..=177 => 6,
        178..=275 => 7,
        276..=392 => 8,
        393..=539 => 9,
        540..=744 => 10,
        745..=966 => 11,
        967..=1229 => 12,
        1230..=1590 => 13,
        1591..=1956 => 14,
        1957..=2456 => 15,
        2457..=3045 => 16,
        3046..=3731 => 17,
        3732..=4525 => 18,
        4526..=5439 => 19,
        5440..=6481 => 20,
        6482..=7665 => 21,
        7666..=9002 => 22,
        9003..=10505 => 23,
        10506..=12186 => 24,
        12187..=14059 => 25,
        14060..=16139 => 26,
        16140..=18438 => 27,
        18439..=20973 => 28,
        20974..=23759 => 29,
        23760..=26810 => 30,
        26811..=30145 => 31,
        30146..=33779 => 32,
        33780..=37730 => 33,
        37731..=42016 => 34,
        42017..=46655 => 35,
        46656..=50652 => 36,
        50653..=55968 => 37,
        55969..=60504 => 38,
        60505..=66559 => 39,
        66560..=71676 => 40,
        71677..=78532 => 41,
        78533..=84276 => 42,
        84277..=91997 => 43,
        91998..=98414 => 44,
        98415..=107068 => 45,
        107069..=114204 => 46,
        114205..=123862 => 47,
        123863..=131765 => 48,
        131766..=142499 => 49,
        142500..=151221 => 50,
        151222..=163104 => 51,
        163105..=172696 => 52,
        172697..=185806 => 53,
        185807..=196321 => 54,
        196322..=210738 => 55,
        210739..=222230 => 56,
        222231..=238035 => 57,
        238036..=250561 => 58,
        250562..=267839 => 59,
        267840..=281455 => 60,
        281456..=300292 => 61,
        300293..=315058 => 62,
        315059..=335543 => 63,
        335544..=351519 => 64,
        351520..=373743 => 65,
        373744..=390990 => 66,
        390991..=415049 => 67,
        415050..=433630 => 68,
        433631..=459619 => 69,
        459620..=479599 => 70,
        479600..=507616 => 71,
        507617..=529062 => 72,
        529063..=559208 => 73,
        559209..=582186 => 74,
        582187..=614565 => 75,
        614566..=639145 => 76,
        639146..=673862 => 77,
        673863..=700114 => 78,
        700115..=737279 => 79,
        737280..=765274 => 80,
        765275..=804996 => 81,
        804997..=834808 => 82,
        834809..=877200 => 83,
        877201..=908904 => 84,
        908905..=954083 => 85,
        954084..=987753 => 86,
        987754..=1035836 => 87,
        1035837..=1071551 => 88,
        1071552..=1122659 => 89,
        1122660..=1160498 => 90,
        1160499..=1214752 => 91,
        1214753..=1254795 => 92,
        1254796..=1312321 => 93,
        1312322..=1354651 => 94,
        1354652..=1415576 => 95,
        1415577..=1460275 => 96,
        1460276..=1524730 => 97,
        1524731..=1571883 => 98,
        1571884..=1639999 => 99,
        1640000 => 100,
        _ => return Err(Error::invalid_argument()),
    };
    Ok(level)
}
