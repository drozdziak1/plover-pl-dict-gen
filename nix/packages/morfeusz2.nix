{ stdenv, fetchurl, cmake, openjdk, cppunit, swig, python3 }:
let
  version = "20221204";
  python3WithDeps = python3.withPackages (ps: with ps; [ pyparsing ]);
in
stdenv.mkDerivation {
  pname = "morfeusz2";
  src = fetchurl {
    url = "http://download.sgjp.pl/morfeusz/${version}/morfeusz-src-${version}.tar.gz";
    sha256 = "sha256-j9b+XZC6OIfD0B2vLxJRFhpMjtgR1YWDNmpGtF7yfuU=";
  };
  nativeBuildInputs = [ cmake openjdk cppunit swig ];
  buildInputs = [ python3WithDeps ];
  inherit version;
  JAVA_TOOL_OPTIONS = "-Dfile.encoding=UTF8";
}
