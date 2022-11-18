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

    // NOTE: The right-side combinations include the singles from this
    // point, these are commented out in favor of the definitions
    // earlier in the book (they coincide anyway). Other types of
    // conflicts conflicts are explicitly indicated in comments.

    // Page 105, 106
    "zz" => "C",
    "ts" => "C",
    // "ds" => "C", // Coinflicts with native "ds"
    "dc" => "C",
    "tts" => "C",
    "tc" => "C",

    "rr" => "R",

    "ll" => "L",
    // "wl" => "L", // Conflicts with native "wl"

    "bb" => "B",

    "ss" => "S",
    // "ws" => "S", // Conflicts with native "ws"

    "gg" => "G",

    "tt" => "T",
    "th" => "T",
    "dt" => "T",
    "ght" => "T",

    "vv" => "V",

    "rc" => "CR",
    "cr" => "CR",

    "nn" => "CL",
    "lc" => "CL",
    "wn" => "CL",
    // "gn" => "CL", // Conflicts with native "gn"
    "cl" => "CL",

    "bc" => "CB",

    "mm" => "CS",
    "sc" => "CS",

    "ch" => "CG",
    "kh" => "CG",

    "ct" => "CT",
    "tst" => "CT",

    "ds" => "CW",
    "wc" => "CW",
    "cw" => "CW",

    "rl" => "RL",

    "rb" => "RB",
    "br" => "RB",

    "rs" => "RS",

    "rg" => "RG",
    "gr" => "RG",

    "rt" => "RT",
    "tr" => "RT",

    "rw" => "RW",
    "wr" => "RW",
    "rv" => "RW",
    "vr" => "RW",
    "wf" => "RW",

    "lb" => "LB",
    "bl" => "LB",

    "ls" => "LS",
    "sl" => "LS",
    "ws" => "LS",
    "vs" => "LS",
    "sk" => "LS",
    "ks" => "LS",
    "x" => "LS",
    "cs" => "LS",

    "lg" => "LG",
    "gl" => "LG",

    "lt" => "LT",
    "tl" => "LT",
    "lth" => "LT",

    "lw" => "LW",
    "wl" => "LW",

    "lv" => "LW",
    "vl" => "LW",

    "bs" => "BS",
    "sb" => "BS",

    "kk" => "BG",
    // "wk" => "BG", // Conflicts with native wk

    "dd" => "BT",
    "bt" => "BT",
    "tb" => "BT",

    "pp" => "BW",
    "bł" => "BW",
    "łb" => "BW",

    "bd" => "BO",
    "db" => "BO",

    // "sch" => "SG", // Conflicts with native "sch"
    "sh" => "SG",
    // "ch(e)" => "SG", // Conflicts with native "ch"

    "st" => "ST",
    "ft" => "ST",

    // "si" => "SW",

    // "ś" => "SO",

    "kt" => "GT",
    "gt" => "GT",
    // "ct" => "GT", // Conflicts with native "ct"
    "tk" => "GT",

    // "cz" => "GW",
    // "cs" => "GW", // Conflicts with native "cs"
    "tch" => "GW",
    "gw" => "GW",
    "wg" => "GW",
    "kv" => "GW",
    "vk" => "GW",
    "kw" => "GW",
    "wk" => "GW",

    // "ci" => "TW",
    "wt" => "TW",
    "tw" => "TW",
    "vt" => "TW",
    "tv" => "TW",

    // "ć" => "TO",

    // "f" => "WY",
    "ff" => "WY",
    "ph" => "WY",

    // page 108, 109, 110
    "rn" => "CRL",
    "nr" => "CRL",
    "ln" => "CRL",
    "nl" => "CRL",

    // "rz" => "CRB",
    // "ż" => "CRB", // Conflict with page 48?

    "rm" => "CRS",
    "mr" => "CRS",

    "rch" => "CRG",
    "chr" => "CRG",

    "trz" => "CRT",
    "rzt" => "CRT",

    "rdz" => "CRW",

    "jl" => "CLB",
    "łc" => "CLB",
    "jn" => "CLB",
    "bn" => "CLB",
    "łn" => "CLB",
    "pn" => "CLB",
    "jł" => "CLB",

    "ns" => "CLS",
    "sn" => "CLS",
    "lm" => "CLS",
    "ml" => "CLS",
    "sm" => "CLS",
    "ms" => "CLS",
    "mn" => "CLS",
    "nm" => "CLS",

    "ng" => "CLG",
    "gn" => "CLG",
    "lch" => "CLG",
    "chl" => "CLG",
    "nkh" => "CLG",

    "nt" => "CLT",
    "nth" => "CLT",
    "tn" => "CLT",
    "ctl" => "CLT",

    "ldz" => "CLW",
    // "ni" => "CLW",
    "wni" => "CLW",
    "nw" => "CLW",
    "vln" => "CLW",
    "vn" => "CLW",
    "nv" => "CLW",

    // "ń" => "CLO",
    "wń" => "CLO",

    "zm" => "CBS",
    "mz" => "CBS",
    "mp" => "CBS",
    "mpf" => "CBS",
    "mb" => "CBS",
    "nb" => "CBS",

    "jch" => "CBG",
    "jg" => "CBG",
    "bch" => "CBG",
    // "h" => "CBG",
    "ck" => "CBG",
    "tsk" => "CBG",
    "jh" => "CBG",

    "jt" => "CBT",
    "jd" => "CBT",

    "jw" => "CBW",
    "cp" => "CBW",
    "jp" => "CBW",
    "jb" => "CBW",
    "tsp" => "CBW",
    "jv" => "CBW",

    "gm" => "CSG",
    "mg" => "CSG",
    "chm" => "CSG",
    "mch" => "CSG",
    "msz" => "CSG",
    "sch" => "CSG",

    "tm" => "CST",
    "mt" => "CST",
    "mpt" => "CST",
    "mst" => "CST",
    "stm" => "CST",

    "zdz" => "CSW",
    "msi" => "CSW",

    "mś" => "CSO",
    "śm" => "CSO",

    "cht" => "CGT",

    "chw" => "CGW",
    "gdz" => "CGW",
    "chcz" => "CGW",
    "chł" => "CGW",
    "chv" => "CGW",

    "ctw" => "CTW",
    "dztw" => "CTW",
    "ctv" => "CTW",

    "rł" => "RLB",

    "rls" => "RLS",

    "ltr" => "RLT",

    // "rz" => "RBS", // r + z, conflicts with the ż sound written "rz"
    "zr" => "RBS",
    "rps" => "RBS",

    "rk" => "RBG",
    "kr" => "RBG",
    // "cr" => "RBG", // Conflicts with native "cr"

    "rd" => "RBT",
    "dr" => "RBT",
    "rpt" => "RBT",
    "ptr" => "RBT",

    "rp" => "RBW",
    "pr" => "RBW",

    "rsz" => "RSG",

    "rst" => "RST",
    "str" => "RST",
    "rts" => "RST",

    "ktr" => "RGT",
    "rkt" => "RGT",

    "rgł" => "RGW",

    "rci" => "RTW",

    "rć" => "RTO",

    "rf" => "RWY",
    "fr" => "RWY",

    "sł" => "LBS",
    "zł" => "LBS",
    "łz" => "LBS",
    "lz" => "LBS",
    "zl" => "LBS",

    "gł" => "LBG",
    "łg" => "LBG",
    "błk" => "LBG",
    "blk" => "LBG",

    "łt" => "LBT",
    "tł" => "LBT",
    "dł" => "LBT",
    "łd" => "LBT",

    "pł" => "LBW",
    "łp" => "LBW",
    "pl" => "LBW",
    "lp" => "LBW",
    "łw" => "LBW",
    "łl" => "LBW",

    "lsz" => "LSG",
    "szl" => "LSG",
    "szk" => "LSG",
    "ksz" => "LSG",

    "lst" => "LST",
    "lts" => "LST",
    "lft" => "LST",
    "kst" => "LST",
    "xt" => "LST",

    "śli" => "LSW",
    "skv" => "LSW",
    "vsk" => "LSW",
    "skw" => "LSW",
    "wsk" => "LSW",

    "śl" => "LSO",

    "lcz" => "LGW",
    "lgł" => "LGW",

    "lk" => "LGY",
    "kl" => "LGY",

    "lf" => "LWY",
    "fl" => "LWY",

    "zg" => "BSG",
    "gz" => "BSG",
    // "ż" => "BSG",
    "żb" => "BSG",
    "bsz" => "BSG",
    "szb" => "BSG",
    "ższ" => "BSG",

    "zd" => "BST",
    "bst" => "BST",
    "wd" => "BST",

    "sp" => "BSW",
    "ps" => "BSW",
    "zw" => "BSW",
    "źw" => "BSW",

    "źb" => "BSO",
    "śb" => "BSO",

    "zb" => "BSY",
    "bz" => "BSY",

    "gd" => "BGT",
    "dk" => "BGT",

    // "dż" => "BGW",
    "czb" => "BGW",
    "dżdż" => "BGW",
    "czk" => "BGW",
    "kcz" => "BGW",

    // "dzi" => "BTW",
    "tp" => "BTW",
    // "wd" => "BTW", // WTF? Mysterious conflict with BST
    "bci" => "BTW",
    "pci" => "BTW",
    "pt" => "BTW",
    "vd" => "BTW",
    "dv" => "BTW",

    // "dź" => "BTO",
    "bć" => "BTO",
    "pć" => "BTO",
    "dźb" => "BTO",


    "szt" => "SGT",

    "szcz" => "SGW",
    "ksi" => "SGW",
    "szw" => "SGW",
    // "skw" => "SGW", // Conflicts with  LSW
    // "wsk" => "SGW",
    // "skv" => "SGW",

    "kś" => "SGO",

    "ści" => "STW",
    "dsi" => "STW",
    "wci" => "STW",
    "zci" => "STW",

    "ść" => "STO",
    "wć" => "STO",
    "dś" => "STO",

    "sf" => "SWY",
    "fs" => "SWY",
    "stw" => "SWY",
    "stv" => "SWY",

    "czt" => "GTW",
    "czci" => "GTW",

    "czć" => "GTO",

    "kf" => "GWY",
    "fk" => "GWY",
    "gf" => "GWY",
    "fg" => "GWY",

    // page 112, 113, 114, 115
    "rnc" => "CRLS",

    "ntr" => "CRLT",
    "ndr" => "CRLT",
    "rtn" => "CRLT",

    "rni" => "CRLW",
    "drni" => "CRLW",
    "lni" => "CRLW",

    "rń" => "CRLO",
    "ndrz" => "CRLO",
    "drń" => "CRLO",
    "lń" => "CRLO",

    "mbr" => "CRBS",
    "mpr" => "CRBS",
    "mrz" => "CRBS",
    "rzm" => "CRBS",

    "chrz" => "CRBG",
    "rzk" => "CRBG",
    "krz" => "CRBG",
    "krh" => "CRBG",
    "rh" => "CRBG",

    "drz" => "CRBT",
    // "ndrz" => "CRBT", // conflict with CRLO
    "jtr" => "CRBT",

    "prz" => "CRBW",
    "brz" => "CRBW",
    "rzb" => "CRBW",

    "skrz" => "CRSG",
    "zgrz" => "CRSG",

    "zdrz" => "CRST",
    "strz" => "CRST",
    "jstr" => "CRST",

    "chtr" => "CRGT",

    "rczm" => "CRGW",

    "chrzt" => "CRGO",

    "nz" => "CLBS",
    "zn" => "CLBS",
    "łm" => "CLBS",
    "mł" => "CLBS",
    "mbl" => "CLBS",

    "jkl" => "CLBG",
    "jgl" => "CLBG",
    // "chł" => "CLBG", // Conflict with CGW
    "hl" => "CLBG",
    "lh" => "CLBG",

    "nd" => "CLBT",
    "dn" => "CLBT",
    "ndl" => "CLBT",
    "btn" => "CLBT",

    "łni" => "CLBW",
    "pni" => "CLBW",
    "jni" => "CLBW",

    "łń" => "CLBO",
    "ńb" => "CLBO",
    "pń" => "CLBO",
    "jń" => "CLBO",

    "nż" => "CLSG",
    "nks" => "CLSG",
    "nkc" => "CLSG",
    "nsz" => "CLSG",
    "nsk" => "CLSG",
    "ngst" => "CLSG",
    "ńsk" => "CLSG",
    "żn" => "CLSG",
    "nx" => "CLSG",

    "ndz" => "CLST",
    "nc" => "CLST",
    "nstr" => "CLST",
    "zdn" => "CLST",
    "ndżl" => "CLST",
    "nds" => "CLST",
    "nts" => "CLST",
    "nst" => "CLST",
    "ndzl" => "CLST",
    "nszt" => "CLST",

    "śni" => "CLSW",
    "żni" => "CLSW",
    "lśni" => "CLSW",
    "źni" => "CLSW",

    "śń" => "CLSO",
    "żń" => "CLSO",
    "źń" => "CLSO",
    "lśń" => "CLSO",

    "nkt" => "CLGT",

    "rncz" => "CLGW",
    "ńszcz" => "CLGW",
    "ńcz" => "CLGW",
    "ngw" => "CLGW",
    "ncz" => "CLGW",
    "ndż" => "CLGW",

    "nk" => "CLGY",
    "kn" => "CLGY",

    "ńdzi" => "CLTW",
    "nci" => "CLTW",
    "ndzi" => "CLTW",

    "ńdź" => "CLTO",
    "ńć" => "CLTO",
    "ndź" => "CLTO",
    "nć" => "CLTO",

    "nf" => "CLWY",
    "fn" => "CLWY",

    "jsz" => "CBSG",
    // "żm" => "CBSG", // Conflict with "KTP"
    "mż" => "CBSG",
    "jsk" => "CBSG",
    "hm" => "CBSG",
    "mk" => "CBSG",
    "km" => "CBSG",

    "dm" => "CBST",
    "md" => "CBST",
    "mbd" => "CBST",
    "jst" => "CBST",
    "jts" => "CBST",

    "jsi" => "CBSW",

    "jś" => "CBSO",

    "jc" => "CBSY",
    "js" => "CBSY",
    "jsc" => "CBSY",
    "jm" => "CBSY",
    "jz" => "CBSY",

    "chd" => "CBGT",
    "hd" => "CBGT",

    "jk" => "CBGY",
    "hk" => "CBGY",

    "jtv" => "CBTW",
    "jdzi" => "CBTW",

    "jdź" => "CBTO",

    "jf" => "CBWY",

    "gmat" => "CSGT",
    "chc" => "CSGT",

    "schł" => "CSGW",
    "dżm" => "CSGW",
    "jszcz" => "CSGW",
    "mcz" => "CSGW",
    "czm" => "CSGW",

    "chś" => "CSGO", // lol

    "mstw" => "CSTW",
    "mi" => "CSTW",
    "jści" => "CSTW",
    "dźstw" => "CSTW",

    "mć" => "CSTO",
    "ćm" => "CSTO",
    "jść" => "CSTO",
    "jstw" => "CSTO",

    "mf" => "CSWY",
    "fm" => "CSWY",

    "chci" => "CGTW",
    "chć" => "CGTO",

    "rzł" => "RLBS",
    
    // "rgł" => "RLBG", // Conflict with RGW

    "drt" => "RLBT",

    "rż" => "RBSG",

    "zdr" => "RBST",

    "rzi" => "RBSW",
    // "rzł" => "RBSW", // Conflict with RLBS (Whyyyyyy???)

    "rź" => "RBSO",

    "dżr" => "RBGW",

    // "ptr" => "RBTW", // Conflict with RBT
    "rdzi" => "RBTW",
    // "rpt" => "RBTW", // Conflict with RBT
    "rdw" => "RBTW",

    "rdź" => "RBTO",

    "kstr" => "RSGT",
    "sztr" => "RSGT",
    "rszt" => "RSGT",

    "rszcz" => "RSGW",

    "rsk" => "RSGY",
    "skr" => "RSGY",
    "rx" => "RSGY",
    "rks" => "RSGY",

    "rstw" => "RSTW",
    "rści" => "RSTW",

    "rść" => "RSTO",

    "rkł" => "RGWY",

    "lż" => "LBSG",
    "łż" => "LBSG",
    "żl" => "LBSG",
    "szł" => "LBSG",
    "łsz" => "LBSG",

    "stł" => "LBST",
    "łdz" => "LBST",
    "łst" => "LBST",

    "łzł" => "LBSW",
    "łźl" => "LBSW",
    "lzl" => "LBSW",
    "lzi" => "LBSW",

    "źl" => "LBSO",
    "lź" => "LBSO",

    "łkw" => "LBGW",
    "łcz" => "LBGW",
    "łkł" => "LBGW",

    "łk" => "LBGY",
    "kł" => "LBGY",

    "łci" => "LBTW",

    "łć" => "LBTO",
    "ld" => "LBTO",
    "dl" => "LBTO",

    "lszcz" => "LSGW",
    "szczk" => "LSGW",

    "lsk" => "LSGY",
    "skl" => "LSGY",
    // "wsk" => "LSGY", // Conflict with LSW
    "lks" => "LSGY",

    "lstw" => "LSTW",
    "ństw" => "LSTW",
    "wstw" => "LSTW",
    "nstw" => "LSTW",
    "nctw" => "LSTW",

    "lkł" => "LGWY",

    "żd" => "BSGT",

    "psz" => "BSGW",
    "żdż" => "BSGW",
    "żw" => "BSGW",

    "psk" => "BSGY",
    "bsk" => "BSGY",
    "żk" => "BSGY",

    "ździ" => "BSTW",
    "pstw" => "BSTW",
    "pst" => "BSTW",
    "wdzi" => "BSTW",
    // "dsi" => "BSTW", // Conflict with STW
    "źci" => "BSTW",
    "bstw" => "BSTW",

    "źdź" => "BSTO",
    "źć" => "BSTO",
    "wdź" => "BSTO",
    // "dś" => "BSTO", // Conflict with STO

    "pk" => "BGWY",
    "kp" => "BGWY",
    "pcz" => "BGWY",

    "żci" => "SGTW",
    "żć" => "SGTO",
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
