+++
title = "Enter Nix #2: Flakes and Automatic System Upgrades"
date = 2025-04-02
category = "nixos"

[extra]
comments = false
postid = "0"
miku_img = "miku_music"
miku_q = "I <3 unattended upgrades!"
+++

my current nixos configuration setup:

- ~/git/nixos/nixos-auto-upgrades.md
- ~/git/nixos/flake.nix
- ~/git/nixos/g14.nix

symlinked to:

- /etc/nixos/configuration.nix
- /etc/nixos/flake.nix
- /etc/nixos/g14.nix

### {{title}}

#### 1. enable flakes

g14 is my hostname, you need to change it to your hostname

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: {
    nixosConfigurations.g14 = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [./configuration.nix];
    };
  };
}
```

#### 2. add auto upgrade service

```nix
system.autoUpgrade = {
    enable = true;
    randomizedDelaySec = "30min"; # Adds a random delay to prevent simultaneous updates
    dates = "daily"; # or "weekly", "monthly", etc.
    flags = ["--impure" "--flake" "/etc/nixos"];
    allowReboot = true; # Allow the system to reboot if necessary
    # email = "your-email@example.com"; # Uncomment to receive email notifications
    # emailOnFailure = true;
};
```

also, you need to change your rebuild command to:

```sh
$ nixos-rebuild switch --flake /etc/nixos
```

if you get the error "Failed to start nvidia-powerd service.", add this line to your `configuration.nix`.

```nix
hardware.nvidia.dynamicBoost.enable = false;
```

To see the status of the timer run

```sh
$ systemctl status nixos-upgrade.timer
```

The upgrade log can be printed with this command

```sh
$ systemctl status nixos-upgrade.service
```

useful links:

- <https://nixos.wiki/wiki/Flakes>
- <https://nixos.wiki/wiki/Automatic_system_upgrades>
- <https://aires.fyi/blog/why-is-enabling-automatic-updates-in-nixos-hard/>
