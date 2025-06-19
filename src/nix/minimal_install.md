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

<details>
<summary> ✔️ Click for BTRFS Subvolume Overview </summary>

A **Btrfs subvolume** is essentially a distinct section within a Btrfs
filesystem that maintains its own set of files and directories, along with a
separate inode numbering system. Unlike block-level partitions (such as LVM
logical volumes), Btrfs subvolumes operate at the file level and are based on
file extents.

**Extents** in Btrfs are contiguous blocks of data on disk that store the actual
contents of files. When files are created or modified, Btrfs manages these
extents efficiently, allowing features like deduplication and snapshots.
Multiple subvolumes can reference the same extents, meaning that identical data
is not duplicated on disk, which saves space and improves performance.

A **snapshot** in Btrfs is a special kind of subvolume that starts with the same
content as another subvolume at the time the snapshot is taken. Snapshots are
typically writable by default, so you can make changes in the snapshot without
affecting the original subvolume. This is possible because Btrfs tracks changes
at the extent level, only creating new extents when files are modified (a
technique called copy-on-write).

Subvolumes in Btrfs behave much like regular directories from a user’s
perspective, but they support additional operations such as renaming, moving,
and nesting (placing subvolumes within other subvolumes). There are no
restrictions on nesting, though it can affect how snapshots are created and
managed. Each subvolume is assigned a unique and unchangeable numeric ID
(subvolid or rootid).

You can access a Btrfs subvolume in two main ways:

- As a normal directory within the filesystem.

- By mounting it directly as if it were a separate filesystem, using the subvol
  or subvolid mount options. When mounted this way, you only see the contents of
  that subvolume, similar to how a bind mount works.

When a new Btrfs filesystem is created, it starts with a "top-level" subvolume
(with an internal ID of 5). This subvolume is always present and cannot be
deleted or replaced, and it is the default mount point unless changed with btrfs
subvolume set-default.

Subvolumes can also have storage quotas set using Btrfs’s quota groups
(qgroups), but otherwise, they all draw from the same underlying storage pool.
Thanks to features like deduplication and snapshots, subvolumes can share data
efficiently at the extent level.While ZFS is a solid choice and offers some
benefits over BTRFS, I recommend looking into it before making your own
decision.

</details>

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

- **Option A**: (Simpler for new users) I also created a starter repo containing
  much of what's needed. If you choose this option follow the `README.md`
  included with the repo.

```bash
cd ~
git clone https://github.com/saylesss88/my-flake.git
```

> Make sure to change line 7 in `disk-config.nix` to what you got from step 3
> `device = "/dev/nvme0n1";`

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

- For `/tmp` on RAM use something like the following. I've found that having
  disko manage swaps causes unnecessary issues. Using zram follows the ephemeral
  route:

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

After adding the above module, you can see it with:

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

- You can check the layout of your btrfs system with:

```bash
sudo btrfs subvolume list /
```

- You may notice some `old_roots` in the output, which are snapshots, which are
  likely created before system upgrades or reboots for rollback purposes. They
  can be deleted or rolled back as needed.

- [BTRFS Subvolumes](https://btrfs.readthedocs.io/en/latest/Subvolumes.html)

- To continue following along and set up impermanence
  [Click Here](https://saylesss88.github.io/nix/impermanence.html)

## Recovery with `nixos-enter` and chroot

🛠️ Recovery: Chroot into Your NixOS Btrfs+Impermanence System

If you need to repair your system (e.g., forgot root password, fix a broken
config, etc.), follow these steps to chroot into your NixOS install:

1. Boot a Live ISO

   Boot from a NixOS (or any recent Linux) live USB.

   Open a terminal and become root:

```bash
sudo -i
```

2. Identify Your Devices

Your main disk is /dev/nvme0n1

- EFI partition: /dev/nvme0n1p1 (mounted at /boot)

- Root partition: /dev/nvme0n1p2 (Btrfs, with subvolumes)

3. Mount the Btrfs Root Subvolume

First, mount the Btrfs partition somewhere temporary (not as / yet):

```bash
mount -o subvol=root,compress=zstd,noatime /dev/nvme0n1p2 /mnt
```

4. Mount Other Subvolumes

Now mount your other subvolumes as defined in your disko.nix:

```bash
# Home

mkdir -p /mnt/home mount -o subvol=home,compress=zstd,noatime /dev/nvme0n1p2
/mnt/home

# User home (optional, usually not needed unless you want to access it directly)

mkdir -p /mnt/home/user mount -o subvol=home/user,compress=zstd,noatime
/dev/nvme0n1p2 /mnt/home/user

# Nix store

mkdir -p /mnt/nix mount -o subvol=nix,compress=zstd,noatime /dev/nvme0n1p2
/mnt/nix

# Nix persist

mkdir -p /mnt/nix/persist mount -o subvol=persist,compress=zstd,noatime
/dev/nvme0n1p2 /mnt/nix/persist

# /var/log

mkdir -p /mnt/var/log mount -o subvol=log,compress=zstd,noatime /dev/nvme0n1p2
/mnt/var/log

# /var/lib

mkdir -p /mnt/var/lib mount -o subvol=lib,compress=zstd,noatime /dev/nvme0n1p2
/mnt/var/lib
```

Note: If you get "subvolume not found," check the subvolume names with btrfs
subvol list /mnt.

5. Mount the EFI Partition

```bash
mkdir -p /mnt/boot mount /dev/nvme0n1p1 /mnt/boot
```

6. (Optional) Mount Virtual Filesystems

```bash
mount --bind /dev /mnt/dev mount --bind /proc /mnt/proc mount --bind /sys
/mnt/sys mount --bind /run /mnt/run
```

7. Chroot

```bash
chroot /mnt /run/current-system/sw/bin/bash
```

or, if using a non-NixOS live system:

```bash
nixos-enter
```

(You may need to install nixos-enter with nix-shell -p nixos-enter.) 8. You’re
In!

You can now run `nixos-rebuild`, reset passwords, or fix configs as needed. 🔎

📓 Notes

- Adjust compress=zstd,noatime if your config uses different mount options.

- For impermanence, make sure to mount all persistent subvolumes you need.

- If you use swap, you may want to enable it too (e.g., swapon /dev/zram0 if
  relevant).

You can now recover, repair, or maintain your NixOS system as needed!
