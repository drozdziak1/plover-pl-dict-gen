use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

static DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static OUT_FILE_PATH: &'static str = "out.txt";

pub type ErrBox = Box<dyn std::error::Error>;


// Left-hand combinations used only to initiate a word part
static LEFT_HAND_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "f" => "F",
    "z" => "Z",
    "s" => "S",
    "k" => "K",
    "t" => "T",
    "p" => "P",
    "v" => "V",
    "w" => "W",
    "l" => "R",

    // page 29 - occuring primarily at the beginning of the word, mostly in borrowings
    "au" => "EA",
    "eu" => "EU",

    // page 33
    "bi" => "PJI",
    "di" => "TJI",
    "gi" => "KJI",
    "by" => "PEIAU",
    "dy" => "TEIAU",
    "gy" => "KEIAU",

    // page 32
    "b" => "PJ",
    "d" => "TJ",
    "g" => "KJ",

    // page 34
    "sz" => "TP",
    "cz" => "PV",
    "ż" => "TPJ",
    "dż" => "PVJ",

    // page 36
    "c" => "ZS",
    "dz" => "ZSJ",
    "ch" => "KT",
    "h" => "KTJ",
    // Weird conflict with the dź sound, book mentions it as exception in PL, skipping in favor of shorter chord below
    // "dzi" => "ZSJI",
    "dzy" => "ZSEIAU",
    "hi" => "KTJI",
    "hy" => "KTEIAU",

    // page 37
    "rz" => "RJ",

    // page 42
    // NOTE(2022-11-16): I did not understand the expansion rule very well. I hope that I resolved the conflict with dzi correctly.
    "dź" => "TJ~",
    "dzi" => "TJI",
    "dzie" => "TJEI",
    "ć" => "T~",
    "ci" => "TI",
    "cie" => "TEI",
    "ś" => "S~",
    "si" => "SI",
    "sie" => "SEI",
    "ź" => "Z~",
    "zi" => "ZI",
    "zie" => "ZEI",

    // page 42
    "ł" => "LJ",

    // page 58, 59
    "gj" => "KJ",
    "pj" => "PJ",
    "sj" => "SJ",
    "tj" => "TJ",
    "wj" => "VJ",
    "zj" => "ZJ",

    "br" => "PRJ",
    "chr" => "KTR",
    "dr" => "TRJ",
    "fr" => "FR",
    "gr" => "KRJ",
    "hr" => "KTRJ",
    "kr" => "KR",
    "mr" => "KPR",

    "pr" => "PR",
    "sr" => "SR",
    "śr" => "SR", // NOTE: Duplicate!
    "szr" => "TPR",
    "tr" => "TR",
    "wr" => "VR",
    "zr" => "ZR",
    "źr" => "ZR", // NOTE: Duplicate!
    "żr" => "TPRJ",

    // page 63
    "bl" => "PLJ",
    "cl" => "ZSL",
    "chl" => "KTL",
    "dl" => "TRL",
    "fl" => "FL",
    "gl" => "KRL",
    "hl" => "KTLJ",
    "kl" => "KL",
    "ml" => "KPL",
    "pl" => "PL",
    "sl" => "SL",
    "śl" => "SL", // NOTE: Duplicate!
    "szl" => "TPL",
    "tl" => "TL",
    "vl" => "VL",
    "zl" => "ZL",
    "źl" => "ZL", // NOTE: Duplicate

    // Note: From now on, many combinations will reuse existing
    // chords, * is used to signify the R/L case in conflicts,
    // i.e. rz/ł don't use *

    // page 64
    "bł" => "PLJ",
    "cł" => "ZSL",
    "chł" => "KTL",
    "dł" => "TRL",
    "gł" => "KRL",
    "hł" => "KTLJ",
    "kł" => "KL",
    "mł" => "KPL",
    "pł" => "PL",
    "sł" => "SL",
    "szł" => "TPL",
    "tł" => "TL",
    "vł" => "VL",
    "zł" => "ZL",
    "żł" => "TPLJ",

    // page 71
    "brz" => "PRJ",
    "chrz" => "KTR",
    "drz" => "TRJ",
    "grz" => "KRJ",
    "krz" => "KR",
    "mrz" => "KPR",

    "prz" => "PR",
    "trz" => "TR",
    "wrz" => "VR",
    "zrz" => "ZR",

    // page 75, n/ń, s/ś not distinguished
    "bn" => "PLRJ",
    "cn" => "ZSLR",
    "chn" => "KTLR",
    "czn" => "PVLR",
    "dn" => "TLRJ",
    "gn" => "KLRJ",
    "hn" => "KTLRJ",
    "kn" => "KLR",
    "ln" => "VLR",
    "mn" => "KPLR",
    "pn" => "PLR",
    "sn" => "SLR",
    "śn" => "SLR",
    "szn" => "TPLR",
    "tn" => "TLR",
    "wn" => "VLR",
    "zn" => "ZLR",
    "żn" => "TPLRJ",

    "ćp" => "TP*",
    "kp" => "KP*",
    "sp" => "SP",
    "śp" => "SP",
    "szp" => "ZSP",
    "wp" => "FP",

    // page 76
    "db" => "TPJ",
    "dzb" => "ZSP",
    "gb" => "KPJ",
    "mb" => "KPJ*",
    "tb" => "TPJ*",
    "wb" => "FPJ",
    "zb" => "ZPJ",
    "żb" => "TPVJ",

    // page 81, 82,
    "cw" => "ZSV",
    "ćw" => "TV",
    "czw" => "PVR",
    "dw" => "TVJ",
    "dzw" => "ZSVJ",
    "dźw" => "TV~",
    "gw" => "KVJ",
    "hw" => "KTVJ",
    "chw" => "KTV",
    "kw" => "KV",
    "lw" => "VL*",
    "rw" => "VR*",
    "sw" => "SV",
    "św" => "SV",
    "szw" => "TPV",
    "tw" => "TV",
    "ww" => "FV",
    "zw" => "ZV",
    "źw" => "ZV",
    "żw" => "TPVJ",

    "czcz" => "PV*",
    "dżdż" => "PVJ*",
    "ndż" => "TPLRJ*",
    "scz" => "SPV",
    "szcz" => "TPV",
    "tcz" => "TPV",
    "wcz" => "FPV",
    "zdż" => "ZPV",

    "ćk" => "KT~",
    "czk" => "KPV*",
    "łk" => "KLJ*",
    "sk" => "SK",
    "szk" => "ZSK",
    "tk" => "KT*",
    "wk" => "FK",

    "cm" => "ZSKP",
    "ćm" => "KTP",
    "chm" => "KTP",
    "czm" => "KPV",
    "dm" => "KTPJ",
    "gm" => "KPJ",
    "km" => "KPV*",
    "khm" => "KTP",
    "sm" => "SKP",
    "śm" => "SKP",
    "wm" => "FKP",
    "zm" => "ZKP",
    "źm" => "ZKP",
    "żm" => "KTP",

    // page 89, 90
    "dźg" => "KT~",
    "lg" => "KLJ*",
    "łg" => "KLJ*",
    "mg" => "KPJ",
    "ng" => "KLRJ*",
    "sg" => "SKJ",
    "wg" => "FKJ",
    "zg" => "ZKJ",
    "źg" => "ZKJ",

    "zh" => "ZKTJ",

    "czch" => "KTPV*",
    "mch" => "KTP",
    "pch" => "KTP*",
    "sch" => "SKT",
    "tch" => "KTV*",
    "wch" => "FKT",

    "czt" => "TPV*",
    "ft" => "kt",
    "pt" => "TP*",
    "rt" => "TR*",
    "st" => "ST",
    "szt" => "ZST",
    "wt" => "FT",

    "chć" => "KTV",
    "chci" => "KTV",
    "czć" => "TPV",
    "czci" => "TPV",
    "kć" => "KT~",
    "kci" => "KT~",
    "pć" => "TP*",
    "pci" => "TP*",
    "ść" => "ST~",
    "ści" => "ST~",
    "wć" => "FT~",
    "wci" => "FT~",

    "gd" => "KTJ",
    "rd" => "TRJ*",
    "wd" => "FTJ",
    "zd" => "ZTJ",
    "żd" => "TPVJ",

    "gdź" => "KT",
    "wdź" => "FT",
    "zdź" => "ZTJ",
    "źdź" => "ZTJ",

    "ksz" => "KTP",
    "msz" => "KTP",
    "psz" => "KTV",
    "wsz" => "FTP",
    "zsz" => "ZTP",

    // page 96-97
    "bz" => "ZPJ*",
    // "dz" => "ZSJ",
    "gz" => "ZKJ*",
    "tz" => "ZSJ",
    "wz" => "FS",
    "zz" => "ZS*",

    "bź" => "ZPJ*",
    "bzi" => "ZPJ*",
    "gź" => "ZKJ*",
    "gzi" => "ZKJ*",
    "wź" => "FS",
    "wzi" => "FS",
    "zź" => "ZS",
    "zzi" => "ZS",

    "gż" => "KTPJ*",
    "lż" => "TPLJ*",
    "mż" => "KTPJ",
    "rż" => "TPRJ*",
    "wż" => "FTPJ",
    "zż" => "ZTP",

    "ks" => "SK*",
    "ps" => "SP*",
    "ss" => "ZS*",
    "ts" => "ZS*",
    "ws" => "FS",
    "zs" => "ZS",

    "chc" => "ZSKT*",
    "kc" => "ZSK*",
    "sc" => "ZST",
    "wc" => "FST*",

    "rdz" => "ZSRJ*",
    "wdz" => "FST",

    "pf" => "FP*",
    "sf" => "FS*",
};

// Combinations That typically reside in the middle of a word part, but can
// appear at the end.
static CENTER_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "j" => "J",
    "e" => "E",
    "i" => "I",
    "a" => "A",
    "u" => "U",

    // page 30
    "ą" => "EAU",
    "ę" => "EIA",
    "ó" => "EIU",
    "ia" => "IA",
    "ie" => "EI",
    "ią" => "JEAU",
    "ię" => "JEIA",
    "io" => "JAU",
    "ió" => "JU",

};

// Right-hand combinations, typically terminating a word part.
static RIGHT_HAND_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "c" => "C",
    "r" => "R",
    "l" => "L",
    "b" => "B",
    "s" => "S",
    "g" => "G",
    "t" => "T",
    "v" => "V",
    "w" => "V",
    "o" => "O",
    "y" => "Y",

    // Page 44
    "p" => "BW",
    "f" => "WY",
    "dz" => "CW",
    "j" => "CB",

    // Page 46
    "d" => "BT",
    "z" => "BS",
    "k" => "BG",
    "h" => "CBG",

    // Page 48
    "sz" => "SG",
    "cz" => "GW",
    "rz" => "CRB",
    "ż" => "BSG",
    "dż" => "BGW",

    // Page 50
    "m" => "CS",
    "n" => "CL",

    // Page 53
    // NOTE: Missing last column, perhaps this is where static lookup maps may be falling short.
    "dź" => "BTO",
    "dzi" => "BTW",
    "ć" => "TO",
    "ci" => "TW",
    "ś" => "SO",
    "si" => "SW",
    "ź" => "BSO",
    "zi" => "BSW",
    "ń" => "CLO",
    "ni" => "CLW",

    // Page 54
    "ł" => "LB",

};

// Oddball shortcuts and expressions with dedicated chords
static SHORTCUTS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 27 
    "ty" => "TY",
    "to" => "TO",
    "wy" => "VY",
    "vy" => "VY",

    // page 39
    "my" => "KPY",

    // page 75
    "ja" => "JA",
    "mnie" => "KPLRE",
    "mi" => "KPI",
    "mną" => "KPEIA",
    "nas" => "LRAS",
    "nam" => "LRACS",
    "nami" => "LRACSY",

    // page 90
    "gdzie" => "KTE",
    "kto" => "KTO",
};

fn main() -> Result<(), ErrBox> {
    println!("Starting...");

    let mut word_set: BTreeSet<String> = BTreeSet::new();

    let mut file_line_count = 0u32;
    let mut file_word_count = 0u64;
    let mut file_duplicate_count = 0u32;

    let mut file_non_alphanum_count = 0u32;

    for line in DICT.lines() {
        for word in line.split(", ") {
            let sanitized = word.trim().to_lowercase();

            if sanitized
                .chars()
                .any(|ch| !ch.is_alphanumeric() || ch.is_whitespace())
            {
                file_non_alphanum_count += 1;
            } else {
                if word_set.contains(&sanitized) {
                    file_duplicate_count += 1;
                } else {
                    word_set.insert(sanitized);
                }
            }

            file_word_count += 1;
        }
        file_line_count += 1;
    }

    println!(
        "Processed {} words in {} lines, discarded {} duplicates and {} non-alphanumerical and/or multi-word entries",
        file_word_count, file_line_count, file_duplicate_count, file_non_alphanum_count
    );

    let out_buf: String = word_set.into_iter().collect::<Vec<_>>().join("\n");

    let mut out_f = File::create(OUT_FILE_PATH)?;

    out_f.write_all(out_buf.as_bytes())?;

    println!("Written out to {}", OUT_FILE_PATH);

    Ok(())
}
