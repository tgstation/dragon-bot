{ pkgs, stdenv, config, lib, fetchFromGitHub, ... }:

let
version = "main";

in stdenv.mkDerivation {
    name = "dragon-bot";

    meta = with lib; {
      platforms = platforms.x86_64;
    };

    src = fetchFromGitHub {
        owner = "tgstation";
        repo = "dragon-bot";
        rev = "${version}";
        hash = "sha256-xnI1v7jW7waIGQvv2iTeGHKRTDGK+5SWS/DH1mzk+xk=";
    };

    buildInputs = with pkgs; [
        git cargo
    ];

    buildPhase = ''
        cargo build --release
    '';

    testPhase = ''
        cargo test --release
    '';

    installPhase = ''
        mkdir -p $out/bin
        mv ./target/release/dragon-bot $out/bin/dragon-bot
    '';
}
