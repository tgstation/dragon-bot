{
    description = "dragon-bot";

    inputs = {};

    outputs = { ... }: {
        nixosModules = {
            dragon-bot = { ... }: {
                imports = [ ./dragon-bot.nix ];
            };
        };
    };
}
