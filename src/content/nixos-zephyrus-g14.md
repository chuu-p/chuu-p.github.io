+++
title = "Enter Nix #1: NixOS on a Asus Rog Zephyrus G14"
date = 2025-03-31
category = "nixos"

[extra]
comments = false
postid = "0"
miku_img = "miku_zen"
miku_q = "Let go your earthly tether. Enter the void. Empty and become wind. - Guru Laghima"
+++

My main machine is a Asus Rog Zephyrus G14 Laptop. It is by far the best laptop I have ever used.
It has awesome performance and is very portable, with good battery life.

I used PopOS (based on Ubuntu) as my daily OS for many years. After learning about 
nixos, i decided it is time for a new distro. I love the nixos philosophy, and it is now
the best linux distro i know.

### {{title}}

> How to install nixos on a laptop with a nvdia graphics card

#### Boot into nixos

First, download iso, flash iso to usb-stick, reboot into iso, **choose "nomodeset" boot**.

Now, we mount the relevant drives and enter nixos.

~~~sh
$ sudo mount /dev/nvme0n3 /mnt
$ sudo mount /dev/nvme0n1 /mnt/boot
$ sudo nixos-enter
~~~

add this line to `/etc/nixos/configuration.nix`

`boot.kernelParams = [ "nomodeset" ];`

~~~sh
$ sudo nano /etc/nixos/configuration.nix
# add this line:
# boot.kernelParams = [ "nomodeset" ];
$ sudo nixos-rebuild switch
~~~

Now blackscreen is fixed, but the hardware it is not optimally configured yet.

Next we add the hardware support / drivers.

~~~shell
# reboot into
$ sudo reboot

# install nixos hardware 
# https://github.com/NixOS/nixos-hardware
$ sudo nix-channel --add https://github.com/NixOS/nixos-hardware/archive/master.tar.gz nixos-hardware
$ sudo nix-channel --update
$ sudo nano /etc/nixos/configuration.nix
# imports = [
#   <nixos-hardware/asus/zephyrus/ga401>
#   ./hardware-configuration.nix
# ];
$ sudo nixos-rebuild switch
# you can ignore the following error: could not start nvidia power service
$ sudo reboot
~~~

Now, it works!

