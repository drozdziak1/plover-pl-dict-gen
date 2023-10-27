//! This module contains static maps of all known word parts for the
//! Polish steno dictionary

/// Left-hand combinations used only to initiate a word part
pub static LEFT_HAND_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "f" => "F",
    "z" => "Z",
    "s" => "S",
    "k" => "K",
    "q" => "K*", // Not in book, showed up in input dictionary
    "t" => "T",
    "p" => "P",
    "v" => "V",
    "w" => "V",
    "l" => "L-",
    "r" => "R-",
    "n" => "LR-",
    "m" => "KP-",

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

    // page 121
    "scj" => "ZSTJ",
    "zdj" => "ZT",

    "ckl" => "ZSKL",
    "mdl" => "KTPL",
    "mgl" => "KPLJ",
    "pchl" => "KTPL",
    "schl" => "SKTL",
    "scl" => "SKL",
    "sfl" => "SVL",
    "skl" => "SKL",
    "spl" => "SPL",
    "stl" => "STL",
    "szkl" => "ZSKL",
    "szpl" => "ZSPL",
    "tkl" => "KTL*",
    "wgl" => "FKLJ",
    "wkl" => "FKL",
    "wpl" => "FPL",
    "wśl" => "FSL",
    "wszl" => "FTPL",
    "wwl" => "FVL",
    "wzl" => "FSL",
    "zbl" => "ZPL",
    "zgl" => "ZKL",
    "zwl" => "ZVL",

    // page 122
    "mdł" => "KTPL",
    "mgł" => "KPLJ",
    "pchł" => "KTPL",
    "schł" => "SKTL",
    "skł" => "SKL",
    "spł" => "SPL",
    "stł" => "STL",
    "szkł" => "ZSPL",
    "wbł" => "FPLJ",
    "wchł" => "FKTL",
    "wgł" => "FKL",
    "wkł" => "FKL",
    "wpł" => "FPL",
    "wsł" => "FSL",
    "wtł" => "FTL",
    "zbł" => "ZPL",
    "zgł" => "ZKL",
    "zmł" => "ZKPL",
    "zwł" => "ZVL",

    "schr" => "SKTR",
    "sfr" => "SVR",
    "sgr" => "SKRJ",
    "skr" => "SKR",
    "smr" => "SKPR",
    "spr" => "SPR",
    "str" => "STR",
    "szkr" => "ZSKR",
    "szpr" => "ZSPR",
    "sztr" => "ZSTR",
    "wbr" => "FPRJ",

    // page 123
    "wdr" => "FTRJ",
    "wgr" => "FKRJ",
    "wkr" => "FKR",
    "wpr" => "FPR",
    "wśr" => "FSR",
    "wtr" => "FTR",
    "wzr" => "FSR",
    "zbr" => "ZPR",
    "zdr" => "ZTR",
    "zgr" => "ZKR",
    "zmr" => "ZKPR",
    "zwr" => "ZVR",

    "schrz" => "SKTR",
    "skrz" => "SKR",
    "sprz" => "SPR",
    "strz" => "STR",
    "wgrz" => "FSKR",
    "wkrz" => "FSKR",
    "wprz" => "FPR",
    "wtrz" => "FTR",
    "zbrz" => "ZPR",
    "zdrz" => "ZTR",
    "zgrz" => "ZKR",

    // page 124
    "ckn" => "ZSKLR",
    "ćpn" => "TPLR",
    "czchn" => "KTPVLR",
    "czkn" => "KPVLR",
    "dźgn" => "KTLRJ",
    "lgn" => "KVLR*",
    "lśn" => "SVLR",
    "mgn" => "KPLRJ",
    "mkn" => "KPLR",
    "pchn" => "KTPLR",
    "psn" => "SPLR*",
    "rżn" => "TPLRJ*",
    "schn" => "SKTLR",
    "skn" => "SKLR",
    "tchn" => "KTLR*",
    "tkn" => "KTLR*",
    "wgn" => "FKLR",
    "wsn" => "FSLR",
    "wzn" => "FSLR",
    "zgn" => "ZKLR",
    "żgn" => "KTPLR",
    "źgn" => "ZKLR",
    "zmn" => "ZKPLR",
    "krnąbr" => "KLREIARB",

    // page 135
    // "dzb" => "ZTP*", // Conflict with ZSP
    "wzb" => "FSP",
    "źdźb" => "ZTP~",
    "zgb" => "ZKP",

    "kszt" => "KTP*",
    "mst" => "SKTP*",
    "mść" => "SKTP*",
    "wst" => "FST",
    "wść" => "FST",
    "wszt" => "FTP*",
    "zst" => "ZST",

    "bzd" => "ZTP*",
    "bźdź" => "ZTP*",
    "gwdz" => "ZSKV*",
    "wzd" => "FST",
    "wżd" => "FTP*",

    "psk" => "SKP*",
    "wsk" => "FSK",
    "wzg" => "FSK",

    "stch" => "SKT*",
    "wsch" => "FSKT",

    "wsp" => "FSP",
    "wśp" => "FSP",

    // page 136
    "szczm" => "KTPV*",
    "wdm" => "FKTP",
    "wsm" => "FSKP",
    "wzm" => "FSKP",
    "zdm" => "ZKTP",

    "bżdż" => "ZTPJ*",
    "mszcz" => "KTPV",
    "pszcz" => "TPV*",
    "wszcz" => "FTPV",

    "schw" => "SKTV",
    "skw" => "SKV",
    "stw" => "STV",
    "szczw" => "TPV*",
    "szkw" => "ZSKV",
    "tkw" => "KTV",
    "wśw" => "FSV",
    "wzw" => "FSV",
    "zdw" => "ZTV",
    "zdzw" => "ZTW",
    "zdźw" => "ZTW",
    "zgw" => "ZKV",

    "sks" => "SKV*",
    "sps" => "SPV*",

    "zbź" => "ZPV*",
    "zbzi" => "ZPV*",

    // page 141
    "wzgl" => "FSKL",
    "źdźbl" => "ZTPL",

    "wspł" => "FSPL",
    "wzdł" => "FSTL",
    "źdźbł" => "ZTPL",

    "lsnk" => "SKLR*",
    "stchn" => "SKTLR*",

    "bzdr" => "ZTPR",
    "pstr" => "STPR",
    "wskr" => "FSKR",
    "wstr" => "FSTR",
    "wzbr" => "FSPR",
    "wzdr" => "FSTR",

    // page 142
    "bzdrz" => "ZTPR",
    "pstrz" => "STPR",
    "wskrz" => "FSKR",
    "wstrz" => "FSTR",
};

// Combinations That typically reside in the middle of a word part, but can
// appear at the end.
pub static CENTER_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "j" => "J",
    "e" => "E",
    "i" => "I",
    "a" => "A",
    "u" => "U",
    "o" => "AU", // Custom
    "y" => "IAU",

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
pub static RIGHT_HAND_COMBOS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 26
    "c" => "C",
    "r" => "-R",
    "l" => "-L",
    "b" => "B",
    "s" => "S",
    "g" => "G",
    "t" => "T",
    "o" => "O",
    "y" => "Y",
    "v" => "W",
    "w" => "W",

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

pub static PREFIXES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 195, 196
    "bez" => "PJE*S",
    "beze" => "PJE*SO",
    "blisko" => "PL*ISGO",
    "ciało" => "T*ALBO",
    "daleko" => "TLE*GO",
    "dla" => "TL*A",
    "długo" => "TLJ*UGO",
    "do" => "TJ*O",
    // "dookoła" => "TJ*AULBGOY", // I don't think this exists as a prefix
    "drogo" => "TR*AUGO",
    "gdzie" => "KTE*",
    "kiedy" => "KT*Y",
    "koło" => "K*AULBO",
    "między" => "KPE*IACY",
    "mimo" => "KP*ICSO",
    "na" => "LR*A",
    "nad" => "LR*ABT",
    "nade" => "LR*ABTO",
    "nie" => "LRE*",
    "o" => "*O",
    "ob" => "*AUB",
    "obe" => "*AUBO",
    "od" => "*AUBT",
    "ode" => "*AUBTO",
    "około" => "*AULBGO",
    "pa" => "P*A",
    "po" => "P*O",
    "pod" => "P*AUBT",
    "pode" => "P*AUBTO",
    "polo" => "P*AULO",
    "poza" => "P*AUBSOY",
    "pra" => "PR*A",
    "pro" => "PR*O",

    "prze" => "PRE*",
    "przeciw" => "PR*T",
    "przed" => "PRE*T",
    "przede" => "PRE*TO",
    "przez" => "PRE*S",
    "przeze" => "PRE*SO",
    "przy" => "PR*I",
    "roz" => "R*AUS",
    "roze" => "R*AUSO",
    "u" => "*U",
    "wiel" => "VE*IL",
    "wielo" => "VE*ILO",
    "wielko" => "VE*ILGO",
    "wokół" => "FKE*IULB", // Not in the book, felt relevant for stuff like "Wokółuszny"
    "wspól" => "FSPE*IUL",
    "współ" => "FSPE*IULB", // Same, współpracownik, współlokator etc.
    "wy" => "V*I",
    "wze" => "FSE*",
    "za" => "Z*A",

    // page 197, 198
    "a" => "*A",
    "ab" => "*AB",
    "ambi" => "*ACBSY",
    "an" => "*ACL",
    "ana" => "*ACLOY",
    "anty" => "*ACLT",
    "arcy" => "*ACRY",
    "audio" => "*AUOY",
    "auto" => "*AUTO",
    "bio" => "PJ*IO",
    "chemo" => "KTE*CSO",
    "cyber" => "S*IRB",
    "de" => "TJE*",
    "dez" => "TJE*BS",
    "dis" => "TJ*IS",
    "dys" => "TE*IAUS",
    "eks" => "E*LS",
    "ekstra" => "E*RLS",
    "eu" => "E*U",
    "euro" => "E*URO",
    "fono" => "F*AUCLO",
    "foto" => "F*AUTO",
    "geo" => "KJE*O",
    "giga" => "KJ*IG",
    "hiper" => "KTJ*IRB",
    "hipo" => "KTJ*IBO",
    "i" => "*I",
    "im" => "*ICS",
    "info" => "*ICLW",
    "infra" => "*ICRLW",
    "inter" => "*ICRLT",
    "kom" => "K*AUCS",
    "kon" => "K*AUCL",
    "kontra" => "K*AUCLT",
    "krypto" => "KR*IBTO",

    "kwazi" => "KV*ASY",
    "makro" => "KP*AGO",
    "maksy" => "KP*ASG",
    "mega" => "KPE*GOY",
    "meta" => "KPE*T",
    "mikro" => "KP*IGY",
    "mini" => "KP*ICLY",
    "mono" => "KP*AUCLO",
    "multi" => "KP*ULT",
    "neo" => "LRE*O",
    "post" => "P*AUST",
    "pre" => "PR*TO",
    // "pro" => "PR*AU", // Conflict with native PR*O
    "proto" => "PR*AUTO",
    "pseudo" => "SPE*UTO",
    "radio" => "R*AOY",
    "re" => "RE*",
    "semi" => "SE*CSY",
    "sub" => "S*UB",
    "super" => "S*URB",
    "syn" => "S*ICL",
    "techno" => "T*ECLGO",
    "tele" => "TE*AL",
    "termo" => "TE*CRSO",
    "trans" => "TR*ACLS",
    "turbo" => "T*URBO",
    "ultra" => "*URLT",
    "uni" => "*UCLY",
    // "vice" => "V*ICO", // I very rarely see people write "vicemistrz", "vicedyrektor", "wice" feels more common
    "wice" => "V*ICO",

    // page 208
    "w" => "V*",
    "we" => "VE*",
    "z" => "Z*",
    "ze" => "ZE*",
};

pub static NULL_INFIX: &'static str = "XF-OY"; // Base for other infixes
pub static SPACING_INFIX: &'static str = "XF*OY"; // Forces space, e.g. if you make a prefix by mistake where you wanted a preposition

pub static SUFFIXES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 154, 155, 156, 157, 158
    "a" => "ZKPLA",
    "ach" => "ZKPLACG",
    "actw" => "ZKPLACTW",
    "ami" => "ZKPLACSY",
    "arstw" => "ZKPLARSTW",
    "ą" => "ZKPLEAI",
    "ba" => "XFPJA",
    "bach" => "XFPJACG",
    "bami" => "XFPJACSY",
    "bą" => "XFPJEIA",
    "bę" => "XFPJEAU",
    "bie" => "XFPJEI",
    "bo" => "XFPJAU",
    "bom" => "XFPJAUCS",
    "by" => "XFPJY",
    "ca" => "ZSAOY",
    "cach" => "ZSACGOY",
    "cami" => "ZSKPIOY",
    "ccy" => "ZS*IAUOY",
    "ce" => "ZSEOY",
    "cem" => "ZSECSOY",
    "ci" => "ZSIOY",
    "cka" => "ZSKAOY",
    "cką" => "ZSKEIAOY",
    "cki" => "ZSKIOY",
    "ckich" => "ZSKICGOY",
    "ckie" => "ZSKEOY",

    "ckiego" => "ZSKEGOY",
    "ckiej" => "ZSKECBOY",
    "ckiemu" => "ZSKECSW",
    "ckim" => "ZSKICSOY",
    "ckimi" => "ZSKPIOY",
    "com" => "ZSAUCSOY",
    "cowi" => "ZSWIOY",
    "ców" => "ZSEIUOY",
    "cu" => "ZSUOY",
    "cza" => "XFPVA",
    "czą" => "XFPVEIA",
    "cze" => "XFPVE",
    "czego" => "XFPVEGO",
    "czej" => "XFPVECB",
    "czemu" => "XFPVECSW",
    "czy" => "XFPVY",
    "czych" => "XFPVICG",
    "czym" => "XFPVICS",
    "czymi" => "XFPVICSY",
    "e" => "ZKPLE",
    "ec" => "ZKPLEC",
    "ectw" => "ZKPLECTW",
    "ecz" => "ZKPLEGW",
    "ego" => "ZKPLEGO",
    "ej" => "ZKPLECB",
    "ejsz" => "ZKPLECBSG",
    "ek" => "ZKPLEBG",
    "em" => "ZKPLECS",
    "emu" => "ZKPLECSW",
    "erstw" => "ZKPLERSTW",
    "ę" => "ZKPLEAU",
    "ęc" => "ZKPLEAUC",
    "ęstw" => "ZKPLEAUSWY",
    "ęt" => "ZKPLEAUT",
    "i" => "ZKPLI",
    "ia" => "ZKPLIA",
    "iach" => "ZKPLIACG",
    "iami" => "ZKPLIACSY",
    "ią" => "ZKPLJEIA",
    "ich" => "ZKPLICG",

    "ie" => "ZKPLEI",
    "iec" => "ZKPLEIC",
    "iego" => "ZKPLEIGO",
    "iej" => "ZKPLEICB",
    "iemu" => "ZKPLEICSW",
    "ię" => "ZKPLJEAU",
    "iem" => "ZKPLEICS",
    "ien" => "ZKPLEICL",
    "ii" => "ZKPLIEY",
    "im" => "ZKPLICS",
    "imi" => "ZKPLICSY",
    "iom" => "ZKPLJAUCS",
    "ion" => "ZKPLJAUCL",
    "iów" => "ZKPLJUW",
    "iu" => "ZKPLIU",
    "ją" => "XFJEIA",
    "ji" => "XFJI",
    "ja" => "XFJA",
    "je" => "XFJE",
    "jom" => "XFJAUCS",
    "jach" => "XFJACG",
    "jami" => "XFJACSY",
    "ję" => "XFJEAU",
    "ka" => "XFKA",
    "kach" => "XFKACG",
    "kami" => "XFKACSY",
    "kę" => "XFKEAU",
    "ki" => "XFKI",
    "kiem" => "XFKECS",
    "ko" => "XFKO",
    "kom" => "XFKAUCS",
    "mi" => "XFKPI",
    "na" => "XFLRA",
    "ną" => "XFLREIA",
    "ne" => "XFLRE",
    "nego" => "XFLREGO",
    "nej" => "XFLRECB",
    "nemu" => "XFLRECSW",
    "ni" => "XFLRI",
    "nia" => "XFLRJA",

    "nią" => "XFLRJEIA",
    "niąt" => "XFLRJEIAT",
    "nictw" => "XFLRICTW",
    "nie" => "XFLREI",
    "nię" => "XFLRJEAU",
    "nięc" => "XFLRJEAUC",
    "nięci" => "XFLRJEAUTW",
    "niego" => "XFLREIGO",
    "niej" => "XFLREICB",
    "niemu" => "XFLREICSW",
    "nięt" => "XFLRJEAUT",
    "nich" => "XFLRICG",
    "nim" => "XFLRICS",
    "nimi" => "XFLRICSY",
    "ny" => "XFLRY",
    "nych" => "XFLRIAUCG",
    "nym" => "XFLRIAUCS",
    "nymi" => "XFLRIAUCSY",
    "o" => "ZKPLAU",
    "om" => "ZKPLAUCS",
    "ostw" => "ZKPLOSWY",
    "owi" => "ZKPLOWY",
    "ów" => "ZKPLEIUW",
    "scy" => "ZS*IOY",
    "si" => "SIOY",
    "ska" => "SKAOY",
    "ską" => "SKEIAOY",
    "ski" => "SKIOY",
    "skich" => "SKICGOY",
    "skie" => "SKEOY",
    "skiego" => "SKEGOY",
    "skiej" => "SKECBOY",
    "skiemu" => "SKPUOY",
    "skim" => "SKICSOY",
    "skimi" => "SKPIOY",
    "stwa" => "STVAOY",
    "stwach" => "STVACGOY",
    "stwami" => "SKTPVIOY",
    "stwem" => "STVECSOY",
    "stwie" => "STVEIOY",

    "stwo" => "STVAUOY",
    "stwom" => "STVAUCSOY",
    "stwu" => "STVUOY",
    "sza" => "XFTPA",
    "szą" => "XFTPEIA",
    "sze" => "XFTPE",
    "szego" => "XFTPEGO",
    "szej" => "XFTPECB",
    "szemu" => "XFTPECSW",
    "szy" => "XFTPI",
    "szych" => "XFTPICG",
    "szym" => "XFTPICS",
    "szymi" => "XFTPICSY",
    "u" => "ZKPLU",
    "y" => "ZKPLIAU",
    "ych" => "ZKPLIAUCB",
    "ym" => "ZKPLIAUCS",
    "ymi" => "ZKPLIAUCSY",

    // page 173-177
    // "a" => "ZKPLA",
    "acie" => "ZKPLACO",
    "ać" => "ZKPLATO",
    "aj" => "ZKPLACB",
    "ają" => "ZKPLACBSTO",
    "aje" => "ZKPLACBTO",
    "ajecie" => "ZKPLACBO",
    "ajemy" => "ZKPLACBSY",
    "ajesz" => "ZKPLACBSG",
    "aję" => "ZKPLACBW",
    "ali" => "ZKPLALY",
    "ał" => "ZKPLALB",
    "ało" => "ZKPLALBO",
    "ały" => "ZKPLALBY",
    "am" => "ZKPLACS",
    "amy" => "ZKPLACSY",

    "an" => "ZKPLACL",
    "ano" => "ZKPLACLO",
    "any" => "ZKPLACLY",
    "asz" => "ZKPLASG",
    "aś" => "ZKPLASO",
    "aw" => "ZKPLAW",
    "awsz" => "ZKPLASGW",
    "awszy" => "ZKPLASGWY",
    // "ą" => "ZKPLEIA",
    "ąc" => "ZKPLEIAC", // The book duplicates here for no reason, decided to add C on my own
    "ący" => "ZKPLEIACY",
    // "by" => "XFPJY",
    "bym" => "XFPEIAUCS",
    "byś" => "XFPEIAUSO",
    "byście" => "XFPEIAUSTO",
    "bysmy" => "XFPEIAUCSOY",
    "cie" => "XFT~E",
    // "e" => "ZKPLE",
    "ecie" => "ZKPLECO",
    "eć" => "ZKPLETO",
    "eją" => "ZKPLECBSTO",
    "eje" => "ZKPLECBTO",
    "ejecie" => "ZKPLECBO",
    "ejemy" => "ZKPLECBSY",
    "ejesz" => "zkplecbsg",
    "eję" => "ZKPLEC",
    "eli" => "ZKPLELY",
    "ely" => "ZKPLELBY",
    "eł" => "ZKPLELB",
    "eło" => "ZKPLELBO",
    // "em" => "ZKPLECS",
    "emy" => "ZKPLECSY",
    "en" => "ZKPLECL",
    "esz" => "ZKPLESG",
    "eś" => "ZKPLESO",
    // "ę" => "ZKPLEAU",
    // "i" => "ZKPLI",
    // "ią" => "ZKPLJEAU",
    "iąc" => "ZKPLJEAUC",
    "iący" => "ZKPLJEAUCY",

    "icie" => "ZKPLICO",
    "ić" => "ZKPLITO",
    // "ie" => "ZKPLEI",
    "iecie" => "ZKPLEICO",
    "ieć" => "ZKPLEITO",
    "iemy" => "ZKPLEICSY",
    // "ien" => "ZKPLEICL",
    "iesz" => "ZKPLEISG",
    // "ię" => "ZKPLJEAU",
    "ij" => "ZKPLICB",
    "ili" => "ZKPLILY",
    "ił" => "ZKPLILB",
    "iło" => "ZKPLILBO",
    "iły" => "ZKPLILBY",
    // "im" => "ZKPLICS",
    "imy" => "ZKPLICSY",
    // "ion" => "ZKPLJAUCL",
    "iono" => "ZKPLJAUCLO",
    "iony" => "ZKPLJAUCLY",
    "isz" => "ZKPLISG",
    "iście" => "ZKPLISTO",
    "iśmy" => "ZKPLICSOY",
    "iw" => "ZKPLIW",
    "łam" => "XFLJACS",
    "łaś" => "XFLJASO",
    "łem" => "XFLJECS",
    "łeś" => "XFLJESO",
    "my" => "XFKPY",
    // "ną" => "XFLREAU", // Conflict with XFLREIA
    "nąc" => "XFLREAUC",
    "nąć" => "XFLREAUTO",
    "nął" => "XFLREAU",
    "nąli" => "XFLREAULY",
    "nąw" => "XFLREAUW",
    "nę" => "XFLREIA",
    "nęl" => "XFLREIAL",
    "nęli" => "XFLREIALY",
    "nęł" => "XFLREIALB",
    "nęło" => "XFLREIALBO",
    "nęły" => "XFLREIALBY",

    // "nie" => "XFLRE", // Conflict with XFLREI
    "niecie" => "XFLRECO",
    "niemy" => "XFLRECSY",
    "niesz" => "XFLRESG",
    "niet" => "XFLRET",
    "nięcie" => "XFLREIACO",
    "nięto" => "XFLRJEIATO",
    "nij" => "XFLRICB",
    "nijcie" => "XFLRICBO",
    "nijmy" => "XFLRICBSY",
    "nion" => "XFLRJAUCL",
    "niony" => "XFLRJAUCLY",
    // "om" => "ZKPLAUCS",
    "on" => "ZKPLAUCL",
    "ona" => "ZKPLAUCLGY",
    "one" => "ZKPLAUCLTO",
    "oni" => "ZKPLAUCLWY",
    "ono" => "ZKPLAUCLO",
    "ony" => "ZKPLAUCLY",
    "oś" => "ZKPLAUSO",
    "ować" => "ZKPL-TO",
    "ował" => "ZKPL-LB",
    "owało" => "ZKPL-LBO",
    "owały" => "ZKPL-LBY",
    "owan" => "ZKPL-CL",
    "owaw" => "ZKPL-W",
    "uj" => "ZKPLUCB",
    "ują" => "ZKPLJEAU",
    "ujć" => "ZKPLUCBTO",
    "uje" => "ZKPLJE",
    "ujecie" => "ZKPLJECO",
    "ujemy" => "ZKPLJECSY",
    "ujesz" => "ZKPLJESG",
    // "y" => "ZKPLY",
    "ycie" => "ZKPLIAUCO",
    "yli" => "ZKPLIAULY",
    "ył" => "ZKPLIAULB",
    "yło" => "ZKPLIAULBO",

    "yły" => "ZKPLIAULBY",
    "ymy" => "ZKPLAUCSY",
    "ysz" => "ZKPLIAUSG",
    "yście" => "ZKPLIAUSTO",
    "yśmy" => "ZKPLIAUCSOY",
    "yw" => "ZKPLIAUW",

    "kolwiek" => "XFKLEG",

    "kro" => "XFKR~AU" // -krotność, -krotny, -kroć, not in book, adapted from Czech "-krat"
};

// Oddball shortcuts and expressions with dedicated chords
pub static SHORTCUTS: phf::Map<&'static str, &'static str> = phf::phf_map! {
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
    "kiedy" => "KTY", // Not in the book, based on the prefix/preposition pattern

    // page 177
    "żebym" => "TPJ*ICS",
    "żebyś" => "TPJ*ISO",
    "żeby" => "TPJ*I",
    "żebyśmy" => "TPJ*ICSOY",
    "żebyście" => "TPJ*ISTO",

    "jestem" => "JECST",
    "jesteś" => "STESO",
    "jest" => "JEST",
    "jesteśmy" => "STECSO",
    "jesteście" => "STESTO",
    "są" => "SEAU",

    "będę" => "PJEIA",
    "będziesz" => "PJESG",
    "będzie" => "PJE",
    "będziemy" => "PJECSY",
    "będziecie" => "PJECO",
    "będą" => "PJEAU",

    "nie jestem" => "LRJECST",
    "nie jesteś" => "STLRESO",
    "nie jest " => "LRJEST",
    "nie jesteśmy" => "STLRECSO",
    "nie jesteście" => "STLRESTO",
    "nie są" => "SLREAU",

    "nie będę" => "PLRJEIA",
    "nie będziesz" => "PLRJESG",
    "nie będzie" => "PLRJE",
    "nie będziemy" => "PLRJECSY",
    "nie będziecie" => "PLRJECO",
    "nie będą" => "PLRJEAU",

    // page 178
    "iść" => "ISTO",
    "idę" => "TJEIA",
    "idziesz" => "TJESG",
    "idzie" => "TJEICSO",
    "idziemy" => "TJECSY",
    "idziecie" => "TJETO",
    "idą" => "TJEAU",

    "idąc" => "TJEAUC",

    "pójść" => "PEIUSTO",
    "pójdę" => "PEIUTO",
    "pójdziesz" => "PEIUSGT",
    "pójdzie" => "PEIUCSO", // Book lists PEIUTE which is impossible
    "pójdziemy" => "PEIUCSY",
    "pódziecie" => "PEIUCTO",
    "pójdą" => "PEIUBTO",

    "przyjść" => "PRISTO",
    "przyjdę" => "PRITW",
    "przyjdziesz" => "PRISGT",
    "przyjdzie" => "PRITO",
    "przyjdziemy" => "PRICSY",
    "przyjdziecie" => "PRICTO",
    "przyjdą" => "PRIBTO",

    "przyszedł" => "PRISGTW",
    "przyszło" => "PRILBSGO",
    "przyszły" => "PRILBSGY",

    // page 195, 196
    "bez" => "PJES",
    "beze" => "PJESO",
    "blisko" => "PLISGO",
    "daleko" => "TLEGO",
    "dla" => "TLA",
    "długo" => "TLJUGO",
    "do" => "TJO",
    "dookoła" => "TJAULBGOY",
    "kołem" => "KL-CS",
    "koło" => "KAULBO",
    "między" => "KPEIACY",
    "mimo" => "KPICSO",
    "na" => "LRA",
    "nad" => "LRABT",
    "nade" => "LRABTO",
    "nie" => "LRE",
    "niż" => "LRIBSG",
    "o" => "O",
    "ob" => "AUB",
    "obe" => "AUBO",
    "od" => "AUBT",
    "ode" => "AUBTO",
    "około" => "AULBGO",
    "oprócz" => "PR-GW",
    "po" => "PO",
    "pod" => "PAUBT",
    "pode" => "PAUBTO",
    "poza" => "PAUSOY",
    "pro" => "PRO",

    "prócz" => "PREIUGW",
    "przeciw" => "PR-T",
    "przed" => "PRET",
    "przede" => "PRETO",
    "przez" => "PRES",
    "przeze" => "PRESO",
    "przy" => "PRI",
    "spod" => "SPAUBT",
    "spomiędzy" => "SKPECY",
    "sponad" => "SPLRABT",
    "spośród" => "SPRUT",
    "spoza" => "SPAUSOY", // Book lists SPOSOY which is impossible
    "sprzed" => "SPREBT",
    "u" => "U",
    "wbrew" => "FPREW",
    "według" => "FTLJUG",
    "wokół" => "FKEIULB",
    "za" => "ZA",
    "zamiast" => "ZKPIAST",
    "znad" => "ZLRAUBT",
    "znade" => "ZLRAUBTO",
    "zza" => "ZSA",

    // page 207
    "w" => "V-W",
    "we" => "VE",
    "z" => "Z-BS",
    "ze" => "ZE",
    "ku" => "KU",

    // page 220
    // "ja" => "JA",
    // "mnie" => "KPLRE",
    "mię" => "KPEIA",
    // "mi" => "KPI",
    // "mną" => "KPEAU",

    "sam" => "SACS",
    "sama" => "SACSOY",
    "same" => "SACSTO",
    "sami" => "SACSWY",
    "samo" => "SACSO",

    // "my" => "KPY",
    // "nas" => "LRAS",
    // "nam" => "LRACS",
    // "nami" => "LRACSY",

    // "ty" => "TY",
    "cię" => "TEIA",
    "ciebie" => "TEBTO",
    "ci" => "TI",
    "tobie" => "TAUBTO",
    "tobą" => "TAUBO",

    "se" => "SE",
    "się" => "SEIA",
    "siebie" => "SEBTO",
    "sobie" => "SAUBTO",
    "sobą" => "SAUBO",

    // "wy" => "VY",
    "was" => "VAS",
    "wam" => "VACS",
    "wami" => "VACSY",

    "on" => "AUCL",
    "ona" => "AUCLOY",
    "ono" => "AUCLO",
    "go" => "KJO",
    "jego" => "JEGO",
    "niego" => "LREGO",
    "jej" => "JECB",
    "niej" => "LRECB",
    "mu" => "KPU",
    "jemu" => "JECSW",
    "niemu" => "LRECSW",
    "ją" => "JEAU",
    "nią" => "LREAU",
    "je" => "JE",
    // "nie" => "LRE",
    "nim" => "LRICS",
    // "niej" => "LRECB",

    "oni" => "AUCLY",
    "one" => "AUCLTO",
    "ich" => "ICG",
    "nich" => "LRICG",
    "im" => "ICS",
    // "nim" => "LRICS",
    // "je" => "JE",
    // "nie" => "LRE",
    "nimi" => "LRICSY",

    // page 221
    "mój" => "KPUCB",
    "moja" => "KPAUOY",
    "moją" => "KPAUTW",
    "moje" => "KPAUTO",
    "mojego" => "KPEGO",
    "mojej" => "KPECB",
    "mojemu" => "KPECSW",
    "moi" => "KPAUY",
    "moich" => "KPIAUCG",
    "moim" => "KPIAUCS",
    "moimi" => "KPIAUCSY",

    "mego" => "KPE*GO",
    "memu" => "KPE*CSW",
    "mym" => "KP*IAUCS",
    "ma" => "KPA",
    "mej" => "KPE*CB",
    "mą" => "KPEAU",
    "me" => "KPE",
    "mych" => "KP*IAUCG",
    "mymi" => "KP*IAUCSY",

    // "jego" => "JEGO",

    "nasi" => "LRASY",
    "nasz" => "LRASG",
    "nasza" => "LRASGOY",
    "naszą" => "LRASGTW",
    "nasze" => "LRASGTO",
    "naszego" => "LRASGO",
    "naszej" => "LRACBSG",
    "naszemu" => "LRACSGW",
    "naszych" => "LRIAUCB",
    "naszym" => "LRIAUCS",
    "naszymi" => "LRIAUCSY",

    "twój" => "TVUCB",
    "twoja" => "TVAUOY",
    "twoją" => "TVAUTW",
    "twoje" => "TVAUTO",
    "twojego" => "TVEGO",
    "twojej" => "TVECB",
    "twojemu" => "TVECSW",
    "twoi" => "TVAUY",
    "twoich" => "TVIAUCG",
    "twoim" => "TVIAUCS",
    "twoimi" => "TVIAUCSY",

    "twego" => "TVE*GO",
    "twemu" => "TVE*CSO",
    "twym" => "TV*IAUCS",
    "twa" => "TVA",
    "twej" => "TVE*CB",
    "twą" => "TVEAU",
    "twe" => "TVE",
    "twych" => "TV*IAUCG",
    "twymi" => "TV*IAUCSY",

    // "jej" => "JECB",

    "wasi" => "VASY",
    "wasz" => "VASG",
    "wasza" => "VASGOY",
    "waszą" => "VASGTW",
    "wasze" => "VASGTO",
    "waszego" => "VASGO",
    "waszej" => "VACBSG",
    "waszemu" => "VACSGW",
    "waszych" => "VIAUCB",
    "waszym" => "VIAUCS",
    "waszymi" => "VIAUCSY",

    "swój" => "SVUCB",
    "swoja" => "SVAUOY",
    "swoją" => "SVAUTW",
    "swoje" => "SVAUTO",
    "swojego" => "SVEGO",
    "swojej" => "SVECB",
    "swojemu" => "SVECSW",
    "swoi" => "SVAUY",
    "swoich" => "SVIAUCG",
    "swoim" => "SVIAUCS",
    "swoimi" => "SVIAUCSY",

    "swego" => "SVE*GO",
    "swemu" => "SVE*CSO",
    "swym" => "SV*IAUCS",
    "swa" => "SVA",
    "swej" => "SVE*CB",
    "swą" => "SVEAU",
    "swe" => "SVE",
    "swych" => "SV*IAUCG",
    "swymi" => "SV*IAUCSY",

    // "ich" => "ICG",

    // page 228, 229, 230
    // "ci" => "TI",
    "ta" => "TA",
    "tą" => "TEAU",
    "te" => "TE",
    "tego" => "TEGO",
    "tej" => "TECB",
    "temu" => "TECSW",
    "ten" => "TECL",
    "tę" => "TEIA",
    // "to" => "TO",
    "tych" => "TIAUCG",
    "tym" => "TIAUCS",
    "tymi" => "TIAUCSY",

    "ów" => "EIUW",
    "owa" => "AUCOY",
    "ową" => "AUCTW",
    "owe" => "AUCTO",
    "owego" => "AUCGO",
    "owej" => "AUCB",
    "owemu" => "AUCSW",
    // "owę" => "AUCW", // I don't think this word exists
    "owi" => "AUCY",
    "owo" => "AUCO",
    "owych" => "AUCG",
    "owym" => "AUCS",
    "owymi" => "AUCSY",

    "och" => "*AUCG", // Like "Och Karol"
    "ach" => "*ACG",
    "ech" => "*ECG",

    // "kto" => "KTO",
    "kogo" => "KAUGO",
    "komu" => "KAUCSW",
    "kim" => "KICS",

    "nikt" => "LRIGT",
    "nikogo" => "KLRAUGO",
    "nikomu" => "KLRAUCSW",
    "nikim" => "KLRICS",

    "co" => "ZSO",
    "czego" => "PVEGO",
    "czemu" => "PVECSW",
    "czym" => "PVICS",

    "nic" => "LRIC",
    "niczego" => "PVLREGO",
    "niczemu" => "PVLRECSW",
    "niczym" => "PVLRICS",


    "jacy" => "ZSJI",
    "jaka" => "KJA",
    "jaką" => "KJEAU",
    "jaki" => "KJI",
    "jakich" => "KJICG",
    "jakie" => "KJE",
    "jakiego" => "KJEGO",
    "jakiej" => "KJECB",
    "jakiemu" => "KJECSW",
    "jakim" => "KJICS",
    "jakimi" => "KJICSY",

    "niejacy" => "ZSLRJI",
    "niejaka" => "KLRJA",
    "niejaką" => "KLRJEAU",
    "niejaki" => "KLRJI",
    "niejakich" => "KLRJICG",
    "niejakie" => "KLRJE",
    "niejakiego" => "KLRJEGO",
    "niejakiej" => "KLRJECB",
    "niejakiemu" => "KLRJECSW",
    "niejakim" => "KLRJICS",
    "niejakimi" => "KLRJICSY",

    "czyi" => "PVIY",
    "czyich" => "PVJICG",
    "czyim" => "PVJICS",
    "czyimi" => "PVICSY",
    "czyj" => "PVICB",
    "czyja" => "PVICBOY",
    "czyją" => "PVICBTW",
    "czyje" => "PVICBTO",
    "czyjego" => "PVICBGO",
    "czyjej" => "PVJECB",
    "czyjemu" => "PVICBSW",

    "niczyi" => "PVLRIY",
    "niczyich" => "PVLRJICG",
    "niczyim" => "PVLRJICS",
    "niczyimi" => "PVLRICSY",
    "niczyj" => "PVLRICB",
    "niczyja" => "PVLRICBOY",
    "niczyją" => "PVLRICBTW",
    "niczyje" => "PVLRICBTO",
    "niczyjego" => "PVLRICBGO",
    "niczyjej" => "PVLRJECB",
    "niczyjemu" => "PVLRICBSW",

    "która" => "KTRA",
    "którą" => "KTREAU",
    "które" => "KTRE",
    "którego" => "KTREGO",
    "której" => "KTRECB",
    "któremu" => "KTRECSW",
    "który" => "KTRY",
    "których" => "KTRIAUCG",
    "którym" => "KTRIAUCS",
    "którymi" => "KTRIAUCSY",
    "którzy" => "KTRJI",

    "niektóra" => "KTLRA",
    "niektórą" => "KTLREAU",
    "niektóre" => "KTLRE",
    "niektórego" => "KTLREGO",
    "niektórej" => "KTLRECB",
    "niektóremu" => "KTLRECSW",
    "niektóry" => "KTLRY",
    "niektórych" => "KTLRIAUCG",
    "niektórym" => "KTLRIAUCS",
    "niektórymi" => "KTLRIAUCSY",
    "niektórzy" => "KTLRJI",

    // page 238, 239
    "zero" => "ZERO",

    "jeden" => "TLRJECL",
    "jedna" => "TLRJOY",
    "jedno" => "TLRJO",
    "jedni" => "TLRJ-WY",
    "jedne" => "TLRJ-TO",
    "jednego" => "TLRJEGO",
    "jednym" => "TLRJICS",
    "jednemu" => "TLRJECSW",
    "jednej" => "TLRJECB",
    "jedną" => "TLRJEAU",
    "jednych" => "TLRJICG",
    "jednymi" => "TLRJICSY",

    "dwaj" => "TVJACB",
    "dwa" => "TVJA",
    "dwie" => "TVJEI",
    "dwu" => "TVJU",
    "dwóch" => "TVJUCG",
    "dwóm" => "TVJUCS",
    "dwoma" => "TVJAUCSOY",
    "dwiema" => "TVJECSOY",

    "trzej" => "TRJECB",
    "trzy" => "TRJI",
    "trzech" => "TRJECG",
    "trzem" => "TRJECS",
    "trzema" => "TRJECSOY",

    "czterej" => "PVEACRB",
    "cztery" => "PVERY",
    "czterech" => "PVEACRG",
    "czterem" => "PVEACRS",
    "czterema" => "PVEACRSOY",

    "pięciu" => "PJEIATW",
    "pięć" => "PJEIATO",
    "pięcioma" => "PJEIACST",

    "sześć" => "TPEST",
    "siedem" => "SECBST",
    "osiem" => "AUCLS",
    "dziewięć" => "TVJEIT",
    "dziesięć" => "TJEST",

    "jedenaście" => "JECLST",
    "dwanaście" => "TVJACLST",
    "trzynaście" => "TRICLST",
    "czternaście" => "PVRECLST",
    "piętnaście" => "PJEIACLST",
    "szesnaście" => "TPECLST",
    "siedemnaście" => "SECLST",
    "osiemnaście" => "SKP-CLST",
    "dziewiętnaście" => "TVJECLST",

    "dwadzieścia" => "TVJACT",
    "trzydzieści" => "TRICT",
    "czterdzieści" => "PVECT",
    "pięćdziesiąt" => "PJEIACT",
    "sześćdziesiąt" => "TPECT",
    "siedemdziesiąt" => "SECT",
    "osiemdziesiąt" => "SKP-CT",
    "dziewięćdziesiąt" => "TVJ-CT",

    "sto" => "STO",
    "dwieście" => "TVJEISTO",
    "trzysta" => "TRISTO",
    "czterysta" => "PVERSTO",
    "pięćset" => "PJEIASTO",
    "sześćset" => "TP-STO",
    "siedemset" => "SJ-STO",
    "osiemset" => "SKP-STO",
    "dziewięćset" => "TVJ-STO",

    "tysiąc" => "STEAUC",
    "milion" => "KP-L",
    "miliard" => "KP-LT",
    "bilion" => "PJ-L",

    // page 248, 249, 250
    "albo" => "ALBO",
    "ameryk" => "KPRIBG",
    "bardzo" => "PARSTO",
    "była" => "PEIAULBOY",
    "chciałbym" => "KTPICS",
    "człowiecz" => "PVLAUGW",
    "człowiek" => "PVLAUBG",
    "detektyw" => "TVJEBGT",
    "detekt" => "TJEBGT",
    "dlaczego" => "PVLEGO",
    "dlatego" => "TLEGO",
    "dobranoc" => "TLRJAUC",
    "dobrze" => "TJAUCRBTO",
    "doktor" => "TJAURT",
    "doktorz" => "TJAUCRBT",
    "dopiero" => "PERBTO",
    "dziękuję" => "TJ-BG",
    "gdyby" => "KTIBY",
    "histor" => "KTJIRST",
    "ile" => "ILTO",
    "inform" => "FLRAUCrs",
    "interes" => "TREST", // interest is probably wrong as a briefed word root
    "jeszcze" => "J-SGW",
    "jeśli" => "JELSY",
    "kierunk" => "KRUCLG",
    "kilka" => "KILGOY",
    "kilku" => "KILGW",
    "kobiet" => "KPJET",
    "kocha" => "KAUCGTO", // I'm not convinced, but it's not wrong, changed from kochać
    "komplet" => "KECST",
    "kompleci" => "KECSTW",
    "korekt" => "KAURBGT",
    "laborator" => "LABT",
    "miesiąc" => "SKPEAUC",
    "moment" => "KPAUCST",
    "momenci" => "KPAUCSTW",
    "można" => "TPLRJA",

    "natychmiast" => "KTPIAST",
    "numer" => "LRUCRS",
    "oczywiście" => "PVRISTO",
    "pamięta" => "KPEIATO", // changed from pamiętać, more useful for all possible suffixes
    "pieniądz" => "PLREAUCW",
    "polic" => "PLIC",
    "ponieważ" => "PLRESG",
    "powin" => "PVLRECL", // Changed from powinien
    "prezenci" => "PRECLTW",
    "prezent" => "PRECLT",
    "prezydenci" => "PREBSTW",
    "prezydent" => "PREBST",
    "problem" => "PRAULB",
    "profesor" => "PRAURW",
    "profesorz" => "PRAUCRBW",
    "proszę" => "PRAUSGW",
    "przepraszam" => "P-BW",
    "przyjaciel" => "PRILT",
    "przyjaciół" => "PRILBTW",
    "restaurac" => "STRAUC",
    "rozumie" => "ZKPRETW", // Changed from rozumieć and ZKPRETO
    "samochod" => "SKPAUBT",
    "samochodzi" => "SKPAUBTW",
    "samochód" => "SKPEIUBT",
    "samoloci" => "SKPAUTW",
    "samolot" => "SKPAUT",
    "sekund" => "SKUCLT",
    "specjal" => "SPEACL",
    "sytuac" => "STUC",
    "szpital" => "ZSPILT",
    "szuka" => "TPATO", // Changed from szukać
    "telefon" => "TLEWY",
    "telewiz" => "TLESW",
    "temat" => "TECST",
    "teraz" => "TRABS",
    "trzeba" => "TREBOY",
    "tygod" => "TIGT",
    "tyle" => "TILTO",
    "wiadomość" => "FKPAUSTO",

    "wiadomości" => "FKPAUSTW",
    "wkrótce" => "FKRUC",
    "wszystko" => "FTPISGTO",
    "wtedy" => "FTEBY",
    // "zamiast" => "ZKPIAST",
    "zawsze" => "Z-SGW",
    "zobaczy" => "ZPVITW", // Changed from zobaczyć/ZPVITO
    "zrobi" => "ZRAUBTW", // Changed from zrobić/ZRAUBTO
};

pub static SPECIAL_CHARS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // page 302
    "{.}" => "P-L",
    "{,}" => "V-B",
    "{?}" => "V-L",
    "{!}" => "P-B",
    "{;}" => "VR-RB",
    "{:}" => "VR*RB",
    "\\" => "PL-RB",
    "/" => "VR-CL",
    "\"" => "KPL-CLS",
    "(" => "STVR-RBGW",
    ")" => "ZKPL-CLST",
    "-" => "ST-GW",
    "--" => "ST*GW",
    "{:stop:...}" => "ZK-ST",
    "@" => "PVLR-CRLB",
    "{" => "KTPV-LBSG",
    "}" => "ZKST-SGTW",
    "'" => "LR-CR",
    "[" => "KTPV-LBSG",
    "]" => "KTPV*LBSG",
    "{^.^}" => "X*O", // Like pornhub.com

    // page 303
    // TODO(2022-11-21): Use proper Plover syntax for these actions
    "<new page>" => "P-CRLBSGTW",
    "\n" => "L-CRLBSGTW",
    "\t" => "T-CRLBSGTW",
    "<new paragraph>" => "R-CRLBSGTW",

    // page 304
    // TODO(2022-11-21): Fill in with actions in Plover's
    // format. Needs to go to a new-line, tab in, put speaker title
    // and surname, colon and space
    "<speaker1>" => "ZSKTPVLR-C",
    "<speaker2>" => "ZSKTPVLR-L",
    "<speaker3>" => "ZSKTPVLR-S",
    "<speaker4>" => "ZSKTPVLR-T",
    "<speaker5>" => "ZSKTPVLR-O",
    "<speaker6>" => "ZSKTPVLR-R",
    "<speaker7>" => "ZSKTPVLR-B",
    "<speaker8>" => "ZSKTPVLR-G",
    "<speaker9>" => "ZSKTPVLR-W",
    "<speaker10>" => "ZSKTPVLR-Y",
};

pub static COMMANDS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // Enable/disable output. WARNING: This will be parsed by this
    // codebase as an invalid combination due to JIU
    "{plover:toggle}" => "JEIAU",

    // Add a new dictionary entry
    "{plover:add_translation}" => "JIU",

    // Lookup outlines
    "{plover:lookup}" => "JIAU",

    // Force space
    "{^ ^}" => SPACING_INFIX,

    // Capitalize the previous word
    "{*-|}" => "X~",

    // Capitalize next word
    "{-|}" => "~O",
};

// Contains words like "pralina" which shouldn't use "pra-" like "pradziadek"
pub static PREFIX_EXCEPTIONS: phf::Set<&'static str> = phf::phf_set! {
    "pralina",
};

// Contains words like "przepych" which shouldn't use "-ych" like "biernych"
pub static SUFFIX_EXCEPTIONS: phf::Set<&'static str> = phf::phf_set! {
    "przepych"
};

pub static SJP_DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

pub static PL_DIACRITICS: &'static str = "ąćęłńóśźż";
