{ pkgs, lib, config, mkIf, ... }:

let
  package = pkgs.callPackage ./package.nix { };
  cfg = config.services.dragon-bot;
in {
  options = {
    services.dragon-bot = {
      home-directory = lib.mkOption {
        type = lib.types.str;
        default = "/home/${cfg.username}";
        description = ''
          The home directory of dragon-bot. Should be persistent.
        '';
      };

      username = lib.mkOption {
        type = lib.types.str;
        default = "dragon-bot";
        description = ''
          The username for the system user running dragon-bot.
        '';
      };
    };
  };

  config = {
    users.groups."${cfg.username}" = {};
    users.users."${cfg.username}" = {
      group = "${cfg.username}";
      isSystemUser = true;
      createHome = true;
      home = cfg.home-directory;
    };

    systemd.services.dragon-bot = {
      description = "dragon-bot";
      serviceConfig = {
        User = cfg.username;
        WorkingDirectory = "${package}";
        ExecStart = "${package}/bin/dragon-bot";
        Restart = "always";
        KillMode = "process";
        ReloadSignal = "SIGUSR2";
      };
      wantedBy = [ "multi-user.target" ];
    };
  };
}
