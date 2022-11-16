use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

static DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static OUT_FILE_PATH: &'static str = "out.txt";

pub type ErrBox = Box<dyn std::error::Error>;

// Singles - page 26

static SINGLES_LEFT: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "f" => "F",
    "z" => "Z",
    "s" => "S",
    "k" => "K",
    "t" => "T",
    "p" => "P",
    "v" => "V",
    "w" => "W",
    "l" => "R",
};

static SINGLES_CENTER: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "j" => "J",
    "e" => "E",
    "i" => "I",
    "a" => "A",
    "u" => "U",
};

static SINGLES_RIGHT: phf::Map<&'static str, &'static str> = phf::phf_map! {
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
};

// Left-hand combinations used only to initiate a word part
static PREFICES: phf::Map<&'static str, &'static str> = phf::phf_map! {
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


};

// Combinations That typically reside in the middle of a word part, but can
// appear at the end.
static INFICES: phf::Map<&'static str, &'static str> = phf::phf_map! {
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

    // page 42
    "ł" => "LJ",

};

// Right-hand combinations, typically terminating a word part.
static SUFFICES: phf::Map<&'static str, &'static str> = phf::phf_map! {
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
};

static SHORTCUTS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 27 - 3 introductory Shortcuts
    "ty" => "TY",
    "to" => "TO",
    "wy" => "VY",
    "vy" => "VY",

    // page 39
    "my" => "KPY",
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
