---
title: Encrypted Install (BTRFS)
date: 2025-11-22
author: saylesss88
---

# Encrypted Setups

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

NixOS supports file systems that are encrypted using LUKS (Linux Unified Key
Setup). This guide walks you through an encrypted NixOS installation using Disko
for disk management and Btrfs for subvolumes. It is designed for users who want
full disk encryption and a modern filesystem layout. If you prefer an
unencrypted setup, you can skip the LUKS and encryption steps, but this guide
focuses on security and flexibility.

- For Unencrypted layout
  [Click Here](https://saylesss88.github.io/installation/unencrypted/unencrypted.html)

If you choose to set up impermanence, ensure it matches your install. Encrypted
Setup with Encrypted Impermanence and Unencrypted Setup with Unencrypted
Impermanence.

> ❗ NOTE: This is a bit convoluted, there are a few paths you can follow. If
> you choose to use the starter repo (<https://github.com/saylesss88/my-flake>)
> just follow the included README and use this for reference.

## What does LUKS Encryption Protect?

It's important to understand what disk encryption protects and what it doesn't
protect so you don't have any misconceptions about how safe your data is.

- [NixOS Wiki FDE](https://wiki.nixos.org/wiki/Full_Disk_Encryption)

- [Arch Wiki Data-at-rest encryption](https://wiki.archlinux.org/title/Data-at-rest_encryption)

- [Authenticated Booot and DE on Linux](https://0pointer.net/blog/authenticated-boot-and-disk-encryption-on-linux.html)

- [Bypassing FDE with TPM2 Unlock](https://oddlama.org/blog/bypassing-disk-encryption-with-tpm2-unlock/)

**What LUKS Protects**:

- **Data Confidentiality at Rest**: LUKS encrypts entire block devices (such as
  disk partitions or whole drives), ensuring that all data stored on the
  encrypted device is unreadable without the correct decryption key or
  passphrase. This protects sensitive information from unauthorized access if
  the device is lost, stolen, or physically accessed by an attacker.

- **Physical Security**: If someone gains physical possession of your device
  (for example, by stealing your laptop or removing a hard drive), LUKS ensures
  the data remains inaccessible and appears as random, meaningless bytes without
  the correct credentials.

- **Protection Against Offline Attacks**: LUKS defends against attackers who
  attempt to bypass the operating system by booting from another device or
  removing the drive and mounting it elsewhere. Without the decryption key, the
  data remains protected.

**What LUKS Does Not Protect**:

- **Data in Use**: Once the system is booted and the encrypted device is
  unlocked, the data becomes accessible to the operating system and any user or
  process with the necessary permissions. LUKS does not protect against attacks
  on a running system, such as malware, remote exploits, or unauthorized users
  with access to an unlocked session.

- **File-Level Access Control**: LUKS encrypts entire partitions or disks, not
  individual files or directories. It does not provide granular file-level
  encryption or access control within the operating system.

- **Network Attacks**: LUKS only protects data stored on disk. It does not
  encrypt data transmitted over networks or protect against network-based
  attacks.

- **Bootloader and EFI Partitions**: The initial bootloader or EFI system
  partition cannot be encrypted with LUKS, so some parts of the boot process may
  remain exposed unless additional measures are taken. (i.e., Secure Boot,
  additional passwords, TPM2)

To Sum it Up: LUKS encryption protects the confidentiality of all data stored on
an encrypted block device by making it unreadable without the correct passphrase
or key. This ensures that, if your device is lost or stolen, your data remains
secure and inaccessible to unauthorized users. However, LUKS does not protect
data once the system is unlocked and running, nor does it provide file-level
encryption or protect against malware and network attacks. For comprehensive
security, LUKS should be combined with strong access controls and other security
best practices.

## The Install

1. Get the
   [Nixos Minimal ISO](https://channels.nixos.org/nixos-25.05/latest-nixos-minimal-x86_64-linux.iso)
   Get it on a usb stick, I use Ventoy with Ventoy2Disk.sh. The following is the
   link to the
   [Ventoy TarBall](https://sourceforge.net/projects/ventoy/files/v1.1.05/ventoy-1.1.05-linux.tar.gz/download)
   download, untar it with `tar -xzf ventoy-1.1.05-linux.tar.gz`, and make it
   executable with `chmod +x Ventoy2Disk.sh`, and finally execute it with
   `sudo bash Ventoy2Disk.sh` Follow the prompts to finish the install.

2. Configuring Networking

The minimal installer uses `wpa_supplicant` instead of NetworkManager. Choose
one of the following methods to enable networking:

```bash
sudo systemctl start wpa_supplicant
wpa_cli
```

### Option A: Interactive `wpa_cli`

```bash
> add_network
0

> set_network 0 ssid "myhomenetwork"
OK

> set_network 0 psk "mypassword"
OK

> enable_network 0
OK
```

To exit type `quit`, then check your connection with `ping google.com`.

### Option B: Non-Interactive `wpa_passphrase`

This method is quicker for known networks and persists the configuration for the
live environment.

First, identify your wireless interface name (e.g., `wlan0`) using `ip a`.

```bash
sudo systemctl start wpa_supplicant # Ensure wpa_supplicant is running
# This command generates the config and appends it to a file specific to wlan0
sudo wpa_passphrase "myhomenetwork" "mypassword" | sudo tee /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
sudo systemctl restart wpa_supplicant@wlan0.service
```

After either method, exit `wpa_cli` with `quit`. Then test your connection:

```bash
ping 1.1.1.1
```

3. Get your Disk Name with `lsblk`

The output should be something like:

```bash
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
nvme0n1     259:0    0   1,8T  0 disk
```

> ❗ From here, you can either

4. Copy the disk configuration to your machine. You can choose one from the
   [examples directory](https://github.com/nix-community/disko/tree/master/example).

There is still a starter repo that can save you some typing, make sure to
carefully review if you decide to use it:

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
export EDITOR='hx' # or 'vi'
nix-shell -p git yazi helix mkpasswd
git config --global user.name "gitUsername"
git config --global user.email "gitEmail"
# OPTIONAL starter repo containing disk-config set up for impermanence
git clone https://github.com/saylesss88/my-flake.git
```

I prefer `helix` here as it's defaults are great. (i.e., auto closing brackets
and much more)

If you choose to use the starter repo you won't need to run the next command as
it is already populated in the repo and should use the
[Starter Repo README](https://github.com/saylesss88/my-flake) most of the rest
of the guide is for manual disko without the starter repo.

If you click on the layout you want then click the `Raw` button near the top,
then copy the `url` and use it in the following command:

```bash
cd /tmp
curl https://raw.githubusercontent.com/nix-community/disko/refs/heads/master/example/luks-btrfs-subvolumes.nix -o /tmp/disk-config.nix
```

The above curl command is to the `luks-btrfs-subvolumes.nix` layout.

5. Make Necessary changes, I prepared mine for impermanence with the following:

```bash
hx /tmp/disk-config.nix
```

Make sure you identify your system disk name with `lsblk` and change the
`device` attribute below to match your disk.

```bash
lsblk
nvme0n1       259:0    0 476.9G  0 disk
├─nvme0n1p1   259:1    0   512M  0 part  /boot
└─nvme0n1p2   259:2    0 476.4G  0 part
```

My disk is `nvme0n1`, change below to match yours:

```nix
{
  disko.devices = {
    disk = {
      nvme0n1 = {
        type = "disk";
        # Make sure this is correct with `lsblk`
        device = "/dev/nvme0n1";
        content = {
          type = "gpt";
          partitions = {
            ESP = {
              label = "boot";
              name = "ESP";
              size = "1G";
              type = "EF00";
              content = {
                type = "filesystem";
                format = "vfat";
                mountpoint = "/boot";
                mountOptions = [
                  "defaults"
                ];
              };
            };
            luks = {
              size = "100%";
              label = "luks";
              content = {
                type = "luks";
                name = "cryptroot";
                content = {
                  type = "btrfs";
                  extraArgs = ["-L" "nixos" "-f"];
                  subvolumes = {
                    "/root" = {
                      mountpoint = "/";
                      mountOptions = ["subvol=root" "compress=zstd" "noatime"];
                    };
                    "/root-blank" = {
                      mountOptions = ["subvol=root-blank" "nodatacow" "noatime"];
                    };
                    "/home" = {
                      mountpoint = "/home";
                      mountOptions = ["subvol=home" "compress=zstd" "noatime"];
                    };
                    "/nix" = {
                      mountpoint = "/nix";
                      mountOptions = ["subvol=nix" "compress=zstd" "noatime"];
                    };
                    "/persist" = {
                      mountpoint = "/persist";
                      mountOptions = ["subvol=persist" "compress=zstd" "noatime"];
                    };
                    "/log" = {
                      mountpoint = "/var/log";
                      mountOptions = ["subvol=log" "compress=zstd" "noatime"];
                    };
                    "/lib" = {
                      mountpoint = "/var/lib";
                      mountOptions = ["subvol=lib" "compress=zstd" "noatime"];
                    };
                    "/persist/swap" = {
                      mountpoint = "/persist/swap";
                      mountOptions = ["subvol=swap" "noatime" "nodatacow" "compress=no"];
                      swap.swapfile.size = "18G";
                    };
                  };
                };
              };
            };
          };
        };
      };
    };
  };

  fileSystems."/persist".neededForBoot = true;
  fileSystems."/var/log".neededForBoot = true;
  fileSystems."/var/lib".neededForBoot = true;
}
```

I have 16G of RAM so to be safe for hibernation I chose to give it some extra
space. The boot partition is 1G, this extra space is for specialisations and
lanzaboote.

or for a swapfile:

```nix
swapDevices = [
  {
    device = "/persist/swap/swapfile";
    size = 18 * 1024; # Size in MB (18GB)
    # or
    # size = 16384; # Size in MB (16G);
  }
];
```

## Setting up zram and /tmp on RAM

While `/tmp` is handled by `tmpfs` (as shown the below `configuration.nix`), you
can further enhance memory efficiency with `zram` for compressed swap, as shown
below.

> ```nix
> {
>   lib,
>   config,
>   ...
> }: let
>   cfg = config.custom.zram;
> in {
>   options.custom.zram = {
>     enable = lib.mkEnableOption "Enable utils module";
>   };
>
>   config = lib.mkIf cfg.enable {
>     zramSwap = {
>       enable = true;
>       # one of "lzo", "lz4", "zstd"
>       algorithm = "zstd";
>        priority = 5;
>        memoryPercent = 50;
>     };
>   };
> }
> ```
>
> And in your `configuration.nix` you would add:
>
> ```nix
> # configuration.nix
> custom = {
>     zram.enable = true;
> };
> ```

After adding the above module and rebuilding, you can see it with:

```bash
swapon --show
NAME       TYPE      SIZE USED PRIO
/dev/zram0 partition 7.5G   0B    5
```

6.  Run disko to partition, format and mount your disks. **Warning** this will
    wipe **EVERYTHING** on your disk. Disko doesn't work with dual boot.

```bash
sudo nix --experimental-features "nix-command flakes" run github:nix-community/disko/latest -- --mode destroy,format,mount /tmp/disk-config.nix
```

Check it with the following:

```bash
mount | grep /mnt
```

The output for an `nvme0n1` disk would be similar to the following:

```bash
#... snip ...
/dev/nvme0n1p2 on /mnt type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=285,subvol=/root)
/dev/nvme0n1p2 on /mnt/persist type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=261,subvol=/persist)
/dev/nvme0n1p2 on /mnt/etc type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=261,subvol=/persist)
/dev/nvme0n1p2 on /mnt/nix type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=260,subvol=/nix)
/dev/nvme0n1p2 on /mnt/var/lib type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=258,subvol=/lib)
/dev/nvme0n1p2 on /mnt/var/log type btrfs (rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=259,subvol=/log)
/dev/nvme0n1p2 on /mnt/nix/store type btrfs (ro,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=260,subvol=/nix)
# ... snip ...
```

7. Generate necessary files, here we use `--no-filesystems` because disko
   handles the `fileSystems` attribute for us.

```bash
nixos-generate-config --no-filesystems --root /mnt
```

- The above command will place a `configuration.nix` and
  `hardware-configuration.nix` in `/mnt/etc/nixos/`

It may be helpful to add a couple things to your `configuration.nix` now, while
it's in its default location. You can just add what you want and rebuild once
with `sudo nixos-rebuild switch` and move on. (i.e. `git`, an editor, etc.).

### Setting a Flake for your minimal Install

8. Create the flake in your home directory to avoid needing to use sudo for
   every command:

```bash
cd   # Move to home directory
mkdir flake
cd /mnt/etc/nixos/
sudo mv hardware-configuration.nix configuration.nix ~/flake/
sudo mv /tmp/disk-config.nix ~/flake/
```

```bash
cd flake
hx flake.nix
```

> You'll change `hostName = nixpkgs.lib.nixosSystem` to your chosen hostname,
> (e.g. `magic = nixpkgs.lib.nixosSystem`). This will be the same as your
> `networking.hostName = "magic";` in your `configuration.nix` that we will set
> up shortly.

```nix
# flake.nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    disko.url = "github:nix-community/disko/latest";
    disko.inputs.nixpkgs.follows = "nixpkgs";
    # impermanence.url = "github:nix-community/impermanence";
  };

  outputs = inputs@{ nixpkgs, ... }: {
    nixosConfigurations = {
      # Change `hostName` to your chosen host name
      nixos = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          inputs.disko.nixosModules.disko
          # inputs.impermanence.nixosModules.impermanence
        ];
      };
    };
  };
}
```

9. Edit `configuration.nix` with what is required, the following are required, I
   clone my original flake repo and move the pieces into place but it's fairly
   easy to just type it all out:

- Bootloader, (e.g., `boot.loader.systemd-boot.enable = true;`)

- User, the example uses `username` change this to your chosen username. If you
  don't set your hostname it will be `nixos`.

- Networking, `networking.networkmanager.enable = true;`

- `hardware-configuration.nix` & `disk-config.nix` for this setup

- If you type this out by hand and mess up a single character, you will have to
  start over completely. A fairly safe way to do this is with `vim` or `hx` and
  redirect the hashed pass to a `/tmp/pass.txt`, you can then read it into your
  `users.nix`:

```bash
mkpasswd --method=yescrypt > /tmp/pass.txt
# Enter your chosen password
```

And then when inside `configuration.nix`, move to the line where you want the
hashed password and type `:r /tmp/pass.txt` to read the hash into your current
file.

```nix
# configuration.nix
{
  config,
  lib,
  pkgs,
  inputs,
  ...
}: {
  imports = [
    # Include the results of the hardware scan.
    ./hardware-configuration.nix
    ./disk-config.nix
  ];

  # systemd Stage 1: if enabled, it handles unlocking of LUKS-encrypted volumes during boot.
    boot.initrd.luks.devices = {
    cryptroot = {
      device = "/dev/disk/by-partlabel/luks";
      allowDiscards = true;
    };
  };

  # This complements using zram, putting /tmp on RAM
    boot = {
    tmp = {
      useTmpfs = true;
      tmpfsSize = "50%";
    };
  };

  # Enable autoScrub for btrfs
    services.btrfs.autoScrub = {
    enable = true;
    interval = "weekly";
    fileSystems = ["/"];
  };


  # Change me!
  networking.hostName = "nixos"; # This will match the `hostname` of your flake

  networking.networkmanager.enable = true;

  boot.loader.systemd-boot.enable = true; # (for UEFI systems only)
  # List packages installed in system profile.
  # You can use https://search.nixos.org/ to find more packages (and options).
  environment.systemPackages = with pkgs; [
    vim # Do not forget to add an editor to edit configuration.nix! The Nano editor is also installed by default.
    #   wget
    git
  ];

  time.timeZone = "America/New_York";

# Change me to your chosen username (i.e. change nixosUser to your username)
  users.users.nixosUser = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ]; # Add "wheel" for sudo access
    initialHashedPassword = "READ_MKPASSWD_OUTPUT_HERE"; # <-- This is where it goes!
    # home = "/home/nixos"; # Optional: Disko typically handles home subvolumes
  };
  # Change me to match your chosen username
  users.group.nixosUser = {};

  console.keyMap = "us";

  nixpkgs.config.allowUnfree = true;

  system.stateVersion = "25.05";
}
```

Although, just adding the `disk-config.nix` works for prompting you for your
encryption passphrase adding the following is a more robust way of ensuring Nix
is aware of this:

```nix
    boot.initrd.luks.devices = {
    cryptroot = {
      device = "/dev/disk/by-partlabel/luks";
      allowDiscards = true;
    };
  };
```

10. Move the flake to `/mnt/etc/nixos` and run `nixos-install`:

```bash
sudo mv ~/flake /mnt/etc/nixos/
```

- Give everything a quick once over, insuring your host is set in both your
  `flake.nix`, and `configuration.nix`. Ensure you changed the username in the
  `configuration.nix` from `nixos` to your chosen name, this is the name you'll
  use to login after you enter your encryption passphrase.

The below command uses `#nixos` because that's what the defaults are, you'll
change it to your chosen hostname.

```bash
sudo nixos-install --flake /mnt/etc/nixos/flake#nixos
```

- You will be prompted to enter a new password if everything succeeds.

## Create a Blank Snapshot of /root

This is essential if you plan on using impermanence with this encrypted setup.
We take a snapshot of `/root` while it's a clean slate, right after we run disko
to format the disk.

To access all of the subvolumes, we have to mount the Btrfs partitions
top-level.

1. Unlock the LUKS device, if not already unlocked as it should be from running
   disko:

```bash
sudo cryptsetup open /dev/disk/by-partlabel/luks cryptroot
```

2. Mount the Btrfs top-level (`subvolid=5`):

```bash
sudo mount -o subvolid=5 /dev/mapper/cryptroot /mnt
```

3. List the contents:

```bash
ls /mnt
# you should see something like
root   home  nix  persist  log  lib  ...
```

4. Now we can take a snapshot of the `root` subvolume:

```bash
sudo btrfs subvolume snapshot -r /mnt/root /mnt/root-blank
```

5. Verify Your Blank Snapshot:

Before continuing, make sure your blank snapshot exists. This is crucial for
impermanence to work properly.

```bash
sudo btrfs subvolume list /mnt
```

You should see output containing both `root` and `root-blank` subvolumes:

```bash
ID 256 gen ... path root
ID 257 gen ... path root-blank
```

Check that the snapshot is read only, this ensures that our snapshot will remain
the same as the day we took it. It was set `ro` in disko but lets check anyways:

```bash
sudo btrfs property get -ts /mnt/root-blank
# output should be
ro=true
```

5. Make sure to unmount:

```bash
sudo umount /mnt
```

- If everything checks out, reboot the system and you should be prompted to
  enter your `user` and `password` to login to a shell to get started.

- The flake will be placed at `/etc/nixos/flake` after the install and reboot, I
  choose to move it to my home directory. Since the file was first in `/etc`
  you'll need to adjust the permissions with something like
  `sudo chown -R $USER:$USER ~/flake` and then you can work on it without
  privilege escalation. This requires that you create a group for your user as
  done in the `configuration.nix` above.

- You can check the layout of your btrfs system with:

```bash
sudo btrfs subvolume list /
```

## Persisting Critical System State

The following is a one time operation, we're just getting it out of the way now.
This moves all of the important system state to a persistant location, further
preparing for impermanence.

It's essential that you have first run the `nixos-install` command to populate
these directories before copying them over.

```bash
sudo mkdir -p /mnt/persist/etc
sudo mkdir -p /mnt/persist/var/lib
sudo mkdir -p /mnt/persist/var/log
sudo mkdir -p /mnt/persist/home
sudo mkdir -p /mnt/persist/root
sudo cp -a /mnt/etc/. /mnt/persist/etc/
sudo cp -a /mnt/var/lib/. /mnt/persist/var/lib
sudo cp -a /mnt/var/log/. /mnt/persist/var/log
sudo cp -a /mnt/home/. /mnt/persist/home/
sudo cp -a /mnt/root/. /mnt/persist/root/
```

Since we are in a live environment, after the install and reboot the `/mnt`
prefix will be removed.

## Reboot

Now that everything is done, we can safely reboot and ensure that our LUKS
password/passphrase is accepted as well as our userlevel password and username.

After reboot, you can continue to setup
[Sops Encrypted Secrets](https://saylesss88.github.io/installation/enc/sops-nix.html)
and
[Lanzaboote Secure Boot](https://saylesss88.github.io/installation/enc/lanzaboote.html)

- To set up impermanence for this specific layout, follow the link
  [Encrypted Impermanence](https://saylesss88.github.io/installation/enc/encrypted_impermanence.html)

- [BTRFS Subvolumes](https://btrfs.readthedocs.io/en/latest/Subvolumes.html)

- [systemd-cryptenroll man page](https://www.freedesktop.org/software/systemd/man/latest/systemd-cryptenroll.html)

- [Linux TPM PCR Registry](https://uapi-group.org/specifications/specs/linux_tpm_pcr_registry/)

- [Bypassing FDE with TPM2](https://oddlama.org/blog/bypassing-disk-encryption-with-tpm2-unlock/)
