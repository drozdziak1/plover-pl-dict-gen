{ stdenv, fetchurl, cmake, openjdk, cppunit, swig, python3 }:
let
  version = "20221204";
  python3WithDeps = python3.withPackages (ps: with ps; [ pyparsing ]);
  polimorf-dict = stdenv.mkDerivation {
    pname = "morfeusz2-polimorf-dict"; inherit version;
    src = fetchurl {
      url = "http://download.sgjp.pl/morfeusz/${version}/polimorf-${version}.tab.gz";
      hash = "sha256-ujoL85ypjuxmqRB37GaKl5PDD4+AYQbnfVSfHg7g9z8=";
    };
    setSourceRoot = "sourceRoot=`pwd`";
    unpackPhase = ''
      cp $src .
      gunzip *.tab.gz
    '';
    installPhase = ''
      mkdir -p $out
      mv *.tab $out/polimorf.tab
    '';
  };
in
stdenv.mkDerivation {
  pname = "morfeusz2";
  src = fetchurl {
    url = "http://download.sgjp.pl/morfeusz/${version}/morfeusz-src-${version}.tar.gz";
    hash = "sha256-j9b+XZC6OIfD0B2vLxJRFhpMjtgR1YWDNmpGtF7yfuU=";
  };
  cmakeFlags = [ "-DINPUT_DICTIONARIES=${polimorf-dict.out}/polimorf.tab" "-DDEFAULT_DICT_NAME=polimorf" "-DEMBEDDED_DEFAULT_DICT=1" ];
  nativeBuildInputs = [ cmake openjdk cppunit swig python3WithDeps ];
  inherit version;
  JAVA_TOOL_OPTIONS = "-Dfile.encoding=UTF8";
}
