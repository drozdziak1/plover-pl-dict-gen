{ lib, fetchFromGitHub, python3Packages, wmctrl, qtbase, mkDerivationWith }:
let
  rtf-tokenize = python3Packages.buildPythonPackage rec {
    pname = "rtf_tokenize";
    version = "1.0.0";
    pyproject = true;
    src = python3Packages.fetchPypi {
      inherit pname version;
      hash = "sha256-XD3zkNAEeb12N8gjv81v37Id3RuWroFUY95+HtOS1gg=";
    };
  };
  plover-stroke = python3Packages.buildPythonPackage rec {
    pname = "plover_stroke";
    version = "1.1.0";
    pyproject = true;
    src = python3Packages.fetchPypi {
      inherit pname version;
      hash = "sha256-3gOyP0ruZrZfaffU7MQjNoG0NUFQLYa/FP3inqpy0VM=";
    };
  };

  plover = with python3Packages; mkDerivationWith buildPythonPackage rec {
    pname = "plover";
    version = "4.0.0rc2";

    meta = with lib; {
      broken = stdenv.isDarwin;
      description = "OpenSteno Plover stenography software";
      maintainers = with maintainers; [ twey kovirobi ];
      license = licenses.gpl2;
    };

    src = fetchFromGitHub {
      owner = "openstenoproject";
      repo = "plover";
      rev = "v${version}";
      sha256 = "sha256-rmMec/BbvOJ92u8Tmp3Kv2YezzJxB/L8UrDntTDSKj4=";
    };

    # I'm not sure why we don't find PyQt5 here but there's a similar
    # sed on many of the platforms Plover builds for
    postPatch = "sed -i /PyQt5/d setup.cfg";

    checkInputs = [ pytest mock ];
    propagatedBuildInputs = [ babel pyqt5 xlib pyserial appdirs wcwidth setuptools plover-stroke rtf-tokenize ];
    doCheck = false;


    dontWrapQtApps = true;

    preFixup = ''
      makeWrapperArgs+=("''${qtWrapperArgs[@]}")
    '';
  };

  plover-polish-slowik = with python3Packages; buildPythonPackage rec {
    pname = "plover_polish_slowik";
    version = "0.0.1";
    src = fetchFromGitHub {
      owner = "flamenco108";
      repo = "plover_polish_slowik";
      rev = "18e1b63dc904631461d3babd379d69ca3e09a587";
      sha256 = null;
    };
    propagatedBuildInputs = with python3Packages; [plover];

  };

  plover-plugins-manager = with python3Packages; mkDerivationWith buildPythonPackage rec {
    pname = "plover_plugins_manager";
    version = "0.7.1";
    pyproject = true;
    src = python3Packages.fetchPypi {
      inherit pname version;
      hash = "sha256-/RiWbGxPtm+0mhDi0heEVb6iBKuyBm6IOq2yrj17n9s=";
    };
    doCheck = false;
    dontWrapQtApps = true;
    propagatedBuildInputs = with python3Packages; [plover readme_renderer requests-cache requests-futures pkginfo wheel pip plover-polish-slowik];
    preFixup = ''
      makeWrapperArgs+=("''${qtWrapperArgs[@]}")
    '';
  };

in
{
  stable = throw "plover.stable was removed because it used Python 2. Use plover.dev instead."; # added 2022-06-05
  dev = plover-plugins-manager;
}
