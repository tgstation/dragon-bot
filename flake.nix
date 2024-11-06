{
    description = "dragon-bot";

    inputs = {};

    outputs = {
        nixosModules = {
            dragon-bot = { config, pkgs, lib, ... }: {
                imports = [ ./dragon-bot.nix ];
            };
        };
    }
}
