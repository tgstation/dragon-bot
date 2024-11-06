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
        hash = "sha256-DofTPeRx7lMX2Un3OYeQ0ZiSpYfdfTp7yvYAIRRwjG8=";
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

    # mv ./dragon-bot/build/release/dragon-bot $out/bin/dragon-bot
    installPhase = ''
        mkdir -p $out/bin
        touch $out/bin/dragon-bot
    '';
}
