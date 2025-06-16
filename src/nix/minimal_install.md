# Minimal BTRFS-Subvol Install with Disko and Flakes

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

Figure 1: **BTRFS Logo**: Image of the BTRFS logo. Sourced from the
[BTRFS repo](https://github.com/btrfs) ![BTRFS logo](../images/btrfs1.png)

## Why I Chose BTRFS

I chose BTRFS because I was already familiar with it from using it with Arch
Linux and I found it to be very easy to use. From what I've read, there are
licensing issues between the Linux Kernel and ZFS which means that ZFS is not
part of the Linux Kernel; it's maintained by the OpenZFS project and available
as a separate kernel module. This can cause issues and make you think more about
your filesystem than I personally want to at this point.

While ZFS is a solid choice and offers some benefits over BTRFS, I recommend
looking into it before making your own decision.

If you have a ton of RAM you could most likely skip the minimal install and just
set your system up as needed or just use
[tmpfs as root](https://elis.nu/blog/2020/05/nixos-tmpfs-as-root/)

## Getting Started with Disko

Disko allows you to declaratively partition and format your disks, and then
mount them to your system. I recommend checking out the
[README](https://github.com/nix-community/disko/tree/master?tab=readme-ov-file)
as it is a **disk destroyer** if used incorrectly.

We will mainly be following the
[disko quickstart guide](https://github.com/nix-community/disko/blob/master/docs/quickstart.md)

Figure 2: **Disko Logo**: Image of the logo for Disko, the NixOS declarative
disk partitioning tool. Sourced from the
[Disko project](https://github.com/nix-community/disko)
![disko logo](../images/disko1.png)

1. Get the
   [Nixos Minimal ISO](https://channels.nixos.org/nixos-25.05/latest-nixos-minimal-x86_64-linux.iso)
   Get it on a usb stick, I use Ventoy with Ventoy2Disk.sh. The following is the
   link to the
   [Ventoy TarBall](https://sourceforge.net/projects/ventoy/files/v1.1.05/ventoy-1.1.05-linux.tar.gz/download)
   download, untar it with `tar -xzf ventoy-1.1.05-linux.tar.gz`, and make it
   executable with `chmod +x Ventoy2Disk.sh`, and finally execute it with
   `sudo bash Ventoy2Disk.sh` Follow the prompts to finish the install.

2. The minimal installer uses `wpa_supplicant` instead of NetworkManager, to
   enable networking run the following:

```bash
sudo systemctl start wpa_supplicant
wpa_cli
```

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

Another option is to do the following, so either the above method or the below
method after starting `wpa_supplicant`:

```bash
# Alternative for quick setup (less interactive, but often faster)
sudo wpa_passphrase "myhomenetwork" "mypassword" >> /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
sudo systemctl restart wpa_supplicant@wlan0.service
```

3. Get your Disk Name with `lsblk`

The output should be something like:

```bash
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
nvme0n1     259:0    0   1,8T  0 disk
```

4. Copy the disk configuration to your machine. You can choose one from the
   [examples directory](https://github.com/nix-community/disko/tree/master/example).

-
- **Option A**: (Simpler for new users) I also created a starter repo containing
  much of what's needed, you should still follow the step to generate your own
  `hardware-configuration.nix` but most everything else should work after you
  change a few things labeled `# Change me!`. If you clone the repo you can skip
  the next curl command.

```bash
cd ~
git clone https://github.com/saylesss88/my-flake.git
```

After cloning the repo, follow step 7 to generate your configuration and replace
the repos `hardware-configuration.nix` with your newly generated one. I
recommend making all the necessary changes (changing the `#Change me!`
locations) while in your home directory, then moving the flake to
`/mnt/etc/nixos/` and installing from there.

- **Option B**: (More flexible, more manual steps) Skip cloning the repo above
  and for the btrfs-subvolume default layout, run the following:

```bash
cd /tmp
curl https://raw.githubusercontent.com/nix-community/disko/refs/heads/master/example/btrfs-subvolumes.nix -o /tmp/disk-config.nix
```

5. Make Necessary changes, I set mine up for impermanence with the following:

```bash
nano /tmp/disk-config.nix
```

```nix
{
  disko.devices = {
    disk = {
      main = {
        type = "disk";
        device = "/dev/nvme0n1";
        content = {
          type = "gpt";
          partitions = {
            ESP = {
              priority = 1;
              name = "ESP";
              start = "1M";
              end = "512M";
              type = "EF00";
              content = {
                type = "filesystem";
                format = "vfat";
                mountpoint = "/boot";
                mountOptions = ["umask=0077"];
              };
            };
            root = {
              size = "100%";
              content = {
                type = "btrfs";
                extraArgs = ["-f"]; # Override existing partition
                # Subvolumes must set a mountpoint in order to be mounted,
                # unless their parent is mounted
                subvolumes = {
                  # Subvolume name is different from mountpoint
                  "/root" = {
                    mountpoint = "/";
                    mountOptions = ["subvol=root" "compress=zstd" "noatime"];
                  };
                  # Subvolume name is the same as the mountpoint
                  "/home" = {
                    mountOptions = ["subvol=home" "compress=zstd" "noatime"];
                    mountpoint = "/home";
                  };
                  # Sub(sub)volume doesn't need a mountpoint as its parent is mounted
                  "/home/user" = {};
                  # Parent is not mounted so the mountpoint must be set
                  "/nix" = {
                    mountOptions = [
                      "subvol=nix"
                      "compress=zstd"
                      "noatime"
                    ];
                    mountpoint = "/nix";
                  };
                  "/nix/persist" = {
                    mountpoint = "/nix/persist";
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
                  # This subvolume will be created but not mounted
                  "/test" = {};
                  # Subvolume for the swapfile
                  "/nix/persist/swap" = {
                    mountpoint = "/nix/persist/swap";
                    mountOptions = ["subvol=swap" "noatime" "nodatacow"];
                    swap = {
                      swapfile.size = "8G";
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
  fileSystems."/nix/persist".neededForBoot = true;
  fileSystems."/var/log".neededForBoot = true;
  fileSystems."/var/lib".neededForBoot = true;
}
```

> ❗ NOTE: It may be unnecessary and even redundant having disko manage your
> swap. Especially if you use `zram` as it will take priority over the swap:

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
> >       priority = 5;
> >       memoryPercent = 50;
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

It may be helpful to add a couple things to your `configuration.nix` now,
rebuild and then move on. Such as, your hostname, git, an editor of your choice.
After your additions run `sudo nixos-rebuild switch` to apply the changes. If
you do this, you can skip the `nix-shell -p` command coming up.

```bash
sudo mv /tmp/disk-config.nix /mnt/etc/nixos
```

### Setting a Flake for your minimal Install

8. Create the flake in your home directory, then move it to `/mnt/etc/nixos`.
   This avoids needing to use `sudo` for every command while in the
   `/mnt/etc/nixos` directory.

```bash
cd ~
mkdir flake && cd flake
nix-shell -p git yazi helix
export NIX_CONFIG='experimental-features = nix-command flakes'
export EDITOR='hx'
hx flake.nix
```

> You'll change `hostname = nixpkgs.lib.nixosSystem` to your chosen hostname,
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
      hostname = nixpkgs.lib.nixosSystem {
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

Move all the files into your flake:

```bash
cd /mnt/etc/nixos/
sudo mv disk-config.nix hardware-configuration.nix configuration.nix ~/flake
```

9. Edit `configuration.nix` with what is required, the following is required, I
   clone my original flake repo and move the pieces into place but it's fairly
   easy to just type it all out:

- Bootloader, (e.g., `boot.loader.systemd-boot.enable = true;`)

- User, the example uses `username` change this to your chosen username. If you
  don't set your hostname it will be `nixos`.

- Networking, `networking.networkmanager.enable = true;`

- `hardware-configuration.nix` & `disk-config.nix` for this setup

- `initialHashedPassword`: Run `mkpasswd -m SHA-512 -s`, then enter your desired
  password. Example output,

```bash
Password: your_secret_password
Retype password: your_secret_password
$6$random_salt$your_hashed_password_string_here_this_is_very_long_and_complex
```

copy the hashed password and use it for the value of your
`initialHashedPassword`

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

  networking.hostName = "magic"; # This will match the `hostname` of your flake

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

  users.users.nixos = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" ]; # Add "wheel" for sudo access
    initialHashedPassword = "COPY_YOUR_MKPASSWD_OUTPUT_HERE"; # <-- This is where it goes!
    # home = "/home/nixos"; # Optional: Disko typically handles home subvolumes
  };

  console.keyMap = "us";

  nixpkgs.config.allowUnfree = true;

  system.stateVersion = "25.05";
}
```

10. Move the flake to `/mnt/etc/nixos` and run `nixos-install`:

```bash
sudo mv ~/flake /mnt/etc/nixos/
sudo nixos-install --flake /mnt/etc/nixos/flake .#hostname
# if the above command doesn't work try this:
sudo nixos-install --flake /mnt/etc/nixos/flake#hostname
```

- You will be prompted to enter a new password if everything succeeds.

- If everything checks out, reboot the system and you should be prompted to
  enter your `user` and `password` to login to a shell to get started.

- The flake will be placed at `/etc/nixos/flake`, I choose to move it to my home
  directory. Since the file was first in `/etc` you'll need to adjust the
  permissions with something like `sudo chown username:users ~/flake`(`username`
  will be your username) and then you can work on it without privilege
  escalation.

- To continue following along and set up impermanence
  [Click Here](https://saylesss88.github.io/nix/impermanence.html)
