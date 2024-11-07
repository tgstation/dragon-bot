{ lib, fetchFromGitHub, rustPlatform, ... }:

rustPlatform.buildRustPackage rec {
    pname = "dragon-bot";
    version = "0.1.0";
    src = fetchFromGitHub {
        owner = "tgstation";
        repo = "dragon-bot";
        rev = "main";
        sha256 = "sha256-xnI1v7jW7waIGQvv2iTeGHKRTDGK+5SWS/DH1mzk+xk=";
    };
    cargoHash = lib.fakeHash;
    cargoLock = {
        lockFile = ./Cargo.lock;
    };
    meta = {
        description = "A Discord bot for the /tg/station Discord server.";
        license = lib.licenses.mit;
    };
}
