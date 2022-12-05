# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "plover-pl-dict-gen-rs/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
}:
let
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
in let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    plover-pl-dict-gen-rs = rustPackages.unknown.plover-pl-dict-gen-rs."0.1.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".aho-corasick."0.7.20" = overridableMkRustCrate (profileName: rec {
    name = "aho-corasick";
    version = "0.7.20";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "cc936419f96fa211c1b9166887b38e5e40b19958e5b895be7c1f93adec7071ac"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
      memchr = rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" = overridableMkRustCrate (profileName: rec {
    name = "atty";
    version = "0.2.14";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "hermit" then "hermit_abi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.19" { inherit profileName; };
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.137" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "cfg-if";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".console."0.15.2" = overridableMkRustCrate (profileName: rec {
    name = "console";
    version = "0.15.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c050367d967ced717c04b65d8c619d863ef9292ce0c5760028655a2fb298718c"; };
    features = builtins.concatLists [
      [ "ansi-parsing" ]
      [ "unicode-width" ]
    ];
    dependencies = {
      ${ if hostPlatform.isWindows then "encode_unicode" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".encode_unicode."0.3.6" { inherit profileName; };
      lazy_static = rustPackages."registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" { inherit profileName; };
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.137" { inherit profileName; };
      terminal_size = rustPackages."registry+https://github.com/rust-lang/crates.io-index".terminal_size."0.1.17" { inherit profileName; };
      unicode_width = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.10" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".encode_unicode."0.3.6" = overridableMkRustCrate (profileName: rec {
    name = "encode_unicode";
    version = "0.3.6";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a357d28ed41a50f9c765dbfe56cbc04a64e53e5fc58ba79fbc34c10ef3df831f"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.3" = overridableMkRustCrate (profileName: rec {
    name = "env_logger";
    version = "0.9.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a12e6657c4c97ebab115a42dcee77225f7f482cdd841cf7088c657a42e9e00e7"; };
    features = builtins.concatLists [
      [ "atty" ]
      [ "default" ]
      [ "humantime" ]
      [ "regex" ]
      [ "termcolor" ]
    ];
    dependencies = {
      atty = rustPackages."registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" { inherit profileName; };
      humantime = rustPackages."registry+https://github.com/rust-lang/crates.io-index".humantime."2.1.0" { inherit profileName; };
      log = rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" { inherit profileName; };
      regex = rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex."1.7.0" { inherit profileName; };
      termcolor = rustPackages."registry+https://github.com/rust-lang/crates.io-index".termcolor."1.1.3" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.19" = overridableMkRustCrate (profileName: rec {
    name = "hermit-abi";
    version = "0.1.19";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.137" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".humantime."2.1.0" = overridableMkRustCrate (profileName: rec {
    name = "humantime";
    version = "2.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9a3a5bfb195931eeb336b2a7b4d761daec841b97f947d34394601737a7bba5e4"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".indicatif."0.17.2" = overridableMkRustCrate (profileName: rec {
    name = "indicatif";
    version = "0.17.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4295cbb7573c16d310e99e713cf9e75101eb190ab31fccd35f2d2691b4352b19"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "unicode-width" ]
    ];
    dependencies = {
      console = rustPackages."registry+https://github.com/rust-lang/crates.io-index".console."0.15.2" { inherit profileName; };
      number_prefix = rustPackages."registry+https://github.com/rust-lang/crates.io-index".number_prefix."0.4.0" { inherit profileName; };
      portable_atomic = rustPackages."registry+https://github.com/rust-lang/crates.io-index".portable-atomic."0.3.15" { inherit profileName; };
      unicode_width = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.10" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" = overridableMkRustCrate (profileName: rec {
    name = "lazy_static";
    version = "1.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.137" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.137";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "fc7fcc620a3bff7cdd7a365be3376c97191aeaccc2a603e600951e452615bf89"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" = overridableMkRustCrate (profileName: rec {
    name = "log";
    version = "0.4.17";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "abb12e687cfb44aa40f41fc3978ef76448f9b6038cad6aef4259d3c095a2382e"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" = overridableMkRustCrate (profileName: rec {
    name = "memchr";
    version = "2.5.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".number_prefix."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "number_prefix";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "830b246a0e5f20af87141b25c173cd1b609bd7779a4617d6ec582abaf90870f3"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".phf."0.11.1" = overridableMkRustCrate (profileName: rec {
    name = "phf";
    version = "0.11.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "928c6535de93548188ef63bb7c4036bd415cd8f36ad25af44b9789b2ee72a48c"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "macros" ]
      [ "phf_macros" ]
      [ "std" ]
    ];
    dependencies = {
      phf_macros = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".phf_macros."0.11.1" { profileName = "__noProfile"; };
      phf_shared = rustPackages."registry+https://github.com/rust-lang/crates.io-index".phf_shared."0.11.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".phf_generator."0.11.1" = overridableMkRustCrate (profileName: rec {
    name = "phf_generator";
    version = "0.11.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b1181c94580fa345f50f19d738aaa39c0ed30a600d95cb2d3e23f94266f14fbf"; };
    dependencies = {
      phf_shared = rustPackages."registry+https://github.com/rust-lang/crates.io-index".phf_shared."0.11.1" { inherit profileName; };
      rand = rustPackages."registry+https://github.com/rust-lang/crates.io-index".rand."0.8.5" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".phf_macros."0.11.1" = overridableMkRustCrate (profileName: rec {
    name = "phf_macros";
    version = "0.11.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "92aacdc5f16768709a569e913f7451034034178b05bdc8acda226659a3dccc66"; };
    dependencies = {
      phf_generator = rustPackages."registry+https://github.com/rust-lang/crates.io-index".phf_generator."0.11.1" { inherit profileName; };
      phf_shared = rustPackages."registry+https://github.com/rust-lang/crates.io-index".phf_shared."0.11.1" { inherit profileName; };
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.47" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.21" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."1.0.103" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".phf_shared."0.11.1" = overridableMkRustCrate (profileName: rec {
    name = "phf_shared";
    version = "0.11.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e1fb5f6f826b772a8d4c0394209441e7d37cbbb967ae9c7e0e8134365c9ee676"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
    dependencies = {
      siphasher = rustPackages."registry+https://github.com/rust-lang/crates.io-index".siphasher."0.3.10" { inherit profileName; };
    };
  });
  
  "unknown".plover-pl-dict-gen-rs."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "plover-pl-dict-gen-rs";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      env_logger = rustPackages."registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.3" { inherit profileName; };
      indicatif = rustPackages."registry+https://github.com/rust-lang/crates.io-index".indicatif."0.17.2" { inherit profileName; };
      log = rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" { inherit profileName; };
      phf = rustPackages."registry+https://github.com/rust-lang/crates.io-index".phf."0.11.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".portable-atomic."0.3.15" = overridableMkRustCrate (profileName: rec {
    name = "portable-atomic";
    version = "0.3.15";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "15eb2c6e362923af47e13c23ca5afb859e83d54452c55b0b9ac763b8f7c1ac16"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "fallback" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.47" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro2";
    version = "1.0.47";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5ea3d908b0e36316caf9e9e2c4625cdde190a7e6f440d794667ed17a1855e725"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      unicode_ident = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.5" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.21" = overridableMkRustCrate (profileName: rec {
    name = "quote";
    version = "1.0.21";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bbe448f377a7d6961e30f5955f9b8d106c3f5e449d493ee1b125c1d43c2b5179"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.47" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".rand."0.8.5" = overridableMkRustCrate (profileName: rec {
    name = "rand";
    version = "0.8.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"; };
    features = builtins.concatLists [
      [ "small_rng" ]
    ];
    dependencies = {
      rand_core = rustPackages."registry+https://github.com/rust-lang/crates.io-index".rand_core."0.6.4" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".rand_core."0.6.4" = overridableMkRustCrate (profileName: rec {
    name = "rand_core";
    version = "0.6.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex."1.7.0" = overridableMkRustCrate (profileName: rec {
    name = "regex";
    version = "1.7.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e076559ef8e241f2ae3479e36f97bd5741c0330689e217ad51ce2c76808b868a"; };
    features = builtins.concatLists [
      [ "aho-corasick" ]
      [ "memchr" ]
      [ "perf" ]
      [ "perf-cache" ]
      [ "perf-dfa" ]
      [ "perf-inline" ]
      [ "perf-literal" ]
      [ "std" ]
    ];
    dependencies = {
      aho_corasick = rustPackages."registry+https://github.com/rust-lang/crates.io-index".aho-corasick."0.7.20" { inherit profileName; };
      memchr = rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" { inherit profileName; };
      regex_syntax = rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.6.28" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.6.28" = overridableMkRustCrate (profileName: rec {
    name = "regex-syntax";
    version = "0.6.28";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "456c603be3e8d448b072f410900c09faf164fbce2d480456f50eea6e25f9c848"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".siphasher."0.3.10" = overridableMkRustCrate (profileName: rec {
    name = "siphasher";
    version = "0.3.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7bd3e3206899af3f8b12af284fafc038cc1dc2b41d1b89dd17297221c5d225de"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syn."1.0.103" = overridableMkRustCrate (profileName: rec {
    name = "syn";
    version = "1.0.103";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a864042229133ada95abf3b54fdc62ef5ccabe9515b64717bcb9a1919e59445d"; };
    features = builtins.concatLists [
      [ "clone-impls" ]
      [ "default" ]
      [ "derive" ]
      [ "full" ]
      [ "parsing" ]
      [ "printing" ]
      [ "proc-macro" ]
      [ "quote" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.47" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.21" { inherit profileName; };
      unicode_ident = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.5" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".termcolor."1.1.3" = overridableMkRustCrate (profileName: rec {
    name = "termcolor";
    version = "1.1.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bab24d30b911b2376f3a13cc2cd443142f0c81dda04c118693e35b3835757755"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "winapi_util" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.5" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".terminal_size."0.1.17" = overridableMkRustCrate (profileName: rec {
    name = "terminal_size";
    version = "0.1.17";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "633c1a546cee861a1a6d0dc69ebeca693bf4296661ba7852b9d21d159e0506df"; };
    dependencies = {
      ${ if !hostPlatform.isWindows then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.137" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.5" = overridableMkRustCrate (profileName: rec {
    name = "unicode-ident";
    version = "1.0.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "6ceab39d59e4c9499d4e5a8ee0e2735b891bb7308ac83dfb4e80cad195c9f6f3"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.10" = overridableMkRustCrate (profileName: rec {
    name = "unicode-width";
    version = "0.1.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c0edd1e5b14653f783770bce4a4dabb4a5108a5370a5f5d8cfe8710c361f6c8b"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" = overridableMkRustCrate (profileName: rec {
    name = "winapi";
    version = "0.3.9";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"; };
    features = builtins.concatLists [
      [ "consoleapi" ]
      [ "errhandlingapi" ]
      [ "fileapi" ]
      [ "handleapi" ]
      [ "minwinbase" ]
      [ "minwindef" ]
      [ "processenv" ]
      [ "std" ]
      [ "winbase" ]
      [ "wincon" ]
      [ "winerror" ]
      [ "winnt" ]
      [ "winuser" ]
    ];
    dependencies = {
      ${ if hostPlatform.config == "i686-pc-windows-gnu" then "winapi_i686_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" { inherit profileName; };
      ${ if hostPlatform.config == "x86_64-pc-windows-gnu" then "winapi_x86_64_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.5" = overridableMkRustCrate (profileName: rec {
    name = "winapi-util";
    version = "0.1.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"; };
  });
  
}
