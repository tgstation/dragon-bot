{
    description = "dragon-bot";

    inputs = {};

    outputs = { ... }: {
        nixosModules = {
            default = { ... }: {
                imports = [ ./dragon-bot.nix ];
            };
        };
    };
}
