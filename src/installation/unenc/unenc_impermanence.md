---
title: Unencrypted BTRFS Impermanence with Flakes
date: 2025-11-24
author: saylesss88
collection: blog
tags: ["nixos", "btrfs", "impermanence"]
draft: false
---

# Unencrypted BTRFS Impermanence with Flakes

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

Figure 1: Impermanence Logo: Image of the Impermanence logo. Sourced from the

[Impermanence repo](https://github.com/nix-community/impermanence)

This guide is for an unencrypted setup, there are a few links at the end for
encrypted setups. This guide follows the previous
[minimal install guide](https://saylesss88.github.io/installation/unencrypted_setups.html)
but you should be able to adjust it carefully to meet your needs.

This section details how to set up impermanence on your NixOS system using BTRFS
subvolumes. With impermanence, your operating system’s root filesystem will
reset to a pristine state on each reboot, while designated directories and files
remain persistent. This provides a highly reliable and rollback-friendly system.

In NixOS, “state” is any data or condition of the system that isn’t defined in
your declarative configuration. The impermanence approach aims to make this
state temporary (ephemeral) or easily resettable, so your system always matches
your configuration and can recover from unwanted changes or corruption.

## Impermanence: The Concept and Its BTRFS Implementation

In a traditional Linux system, most of this state is stored on the disk and
persists indefinitely unless manually deleted or modified. However, this can
lead to configuration drift, where the system accumulates changes (e.g., log
files, temporary files, or unintended configuration tweaks) that make it harder
to reproduce or maintain.

Impermanence, in the context of operating systems, refers to a setup where the
majority of the system’s root filesystem (`/`) is reset to a pristine state on
every reboot. This means any changes made to the system (e.g., installing new
packages, modifying system files outside of configuration management, creating
temporary files) are discarded upon shutdown or reboot.

## What Does Impermanence Do?

Impermanence is a NixOS approach that makes the system stateless (or nearly
stateless) by wiping the root filesystem (`/`) on each boot, ensuring a clean,
predictable starting point. Only explicitly designated data (persistent state)
is preserved across reboots, typically stored in specific locations like the
/nix/persist subvolume. This is possible because NixOS can boot with only the
`/boot`, and `/nix` directories. This achieves:

1. Clean Root Filesystem:

- The root subvolume is deleted and recreated on each boot, erasing transient
  state (e.g., temporary files, runtime data).

- This ensures the system starts fresh, reducing clutter and making it behave
  closer to a declarative system defined by your NixOS configuration.

2. Selective Persistence:

- Critical state (e.g., user files, logs, system configuration) is preserved in
  designated persistent subvolumes (e.g., /nix/persist, /var/log, /var/lib) or
  files.

- You control exactly what state persists by configuring
  `environment.persistence."/nix/persist"` or other mechanisms.

- ❗ The understanding around persisting `/var/lib/nixos` seems to be evolving.
  See,The importance of persisting `/var/lib/nixos` See also necessary system
  state

3. Reproducibility and Security:

- By wiping transient state, impermanence prevents unintended changes from
  accumulating, making the system more reproducible.

- It enhances security by ensuring sensitive temporary data (e.g., /tmp, runtime
  credentials) is erased on reboot.

### Getting Started

1. Add impermanence to your flake.nix. You will change the hostname in the flake
   to match your networking.hostName.

```nix
# flake.nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    disko.url = "github:nix-community/disko/latest";
    disko.inputs.nixpkgs.follows = "nixpkgs";
    impermanence.url = "github:nix-community/impermanence";
  };

  outputs = inputs@{ nixpkgs, ... }: {
    nixosConfigurations = {
      hostname = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          inputs.disko.nixosModules.disko
          inputs.impermanence.nixosModules.impermanence
        ];
      };
    };
  };
}
```

2. Discover where your root subvolume is located with `findmnt`:

Before configuring impermanence, it’s crucial to know the device path and
subvolume path of your main BTRFS partition where the root filesystem (/) is
located. This information is needed for the mount command within the
impermanence script.

```bash
findmnt /
TARGET   SOURCE         FSTYPE OPTIONS
/        /dev/disk/by-partlabel/disk-main-root[/root]
                        btrfs  rw,noatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=275,sub
```

From the SOURCE column, note the full path, including the device (e.g.,
`/dev/disk/by-partlabel/disk-main-root`) and the subvolume in brackets (e.g.,
`[/root]`). You will use the device path in the next step

`/dev/disk/by-partlabel/disk-main-root` is a symlink to the actual device path
(e.g. `/dev/nvme0n1p2`), but using the partlabel is generally more robust for
scripts.

3. Create an impermanence.nix:

Now, create a new file named `impermanence.nix` in your configuration directory
(i.e. your flake directory). This file will contain all the specific settings
for your impermanent setup, including BTRFS subvolume management and persistent
data locations. Since this file is right next to your `configuration.nix`,
you’ll just add an `imports = [ ./impermanence.nix` ] to your
`configuration.nix` apply it to your configuration.

```nix
{lib, ...}: {
  #  Reset root subvolume on boot
  boot.initrd.postResumeCommands = lib.mkAfter ''
    mkdir /btrfs_tmp
      mount /dev/disk/by-partlabel/disk-main-root /btrfs_tmp # CONFIRM THIS IS CORRECT FROM findmnt
      if [[ -e /btrfs_tmp/root ]]; then
        mkdir -p /btrfs_tmp/old_roots
        timestamp=$(date --date="@$(stat -c %Y /btrfs_tmp/root)" "+%Y-%m-%-d_%H:%M:%S")
        mv /btrfs_tmp/root "/btrfs_tmp/old_roots/$timestamp"
      fi

      delete_subvolume_recursively() {
        IFS=$'\n'
        for i in $(btrfs subvolume list -o "$1" | cut -f 9- -d ' '); do
          delete_subvolume_recursively "/btrfs_tmp/$i"
        done
        btrfs subvolume delete "$1"
      }

      for i in $(find /btrfs_tmp/old_roots/ -maxdepth 1 -mtime +30); do
        delete_subvolume_recursively "$i"
      done

      btrfs subvolume create /btrfs_tmp/root
      umount /btrfs_tmp
  '';

  # Use /persist as the persistence root, matching Disko's mountpoint
  environment.persistence."/nix/persist" = {
    hideMounts = true;
    directories = [
      "/etc" # System configuration (Keep this here for persistence via bind-mount)
      "/var/spool" # Mail queues, cron jobs
      "/srv" # Web server data, etc.
      "/root"
    ];
    files = [
    ];
  };
}
```

With btrfs subvolumes since each directory is its own subvolume, when the root
is wiped on reboot the subvolumes are untouched.

### Applying Your Impermanence Configuration

Once you have completed all the steps and created or modified the necessary
files (`flake.nix`, `impermanence.nix`), you need to apply these changes to your
NixOS system.

1. Navigate to your NixOS configuration directory (where your flake.nix is
   located).

```bash
cd /path/to/your/flake
```

2. Rebuild and Switch: Execute the `nixos-rebuild switch` command. This command
   will:

- Evaluate your flake.nix and the modules it imports (including your new
  impermanence.nix).

- Build a new NixOS system closure based on your updated configuration.

- Activate the new system configuration, making it the current running system.

> ❗ NOTE: On the first rebuild after setting up impermanence, you may find that
> you’re not in the password database or cannot log in/sudo. This occurs because
> the initial state of your new ephemeral root filesystem, including /etc (where
> user passwords are stored), is fresh. It has to do with the timing of when
> environment.persistence takes effect during the first boot.

> To avoid this password issue, before your first nixos-rebuild switch for
> impermanence, run:
>
> ```bash
> sudo mkdir -p /nix/persist/etc # Ensure the target directory exists
> sudo cp -a /etc/* /nix/persist/etc
> ```
>
> - This copies your current /etc directory contents (including existing user
>   passwords) into your persistent >>storage.
> - Crucially: You must also ensure that `/etc` is explicitly included in your
>   `environment.persistence."/nix/persist"`.directories list in your
>   `impermanence.nix` like we did above, (or main configuration). This
>   configures >NixOS to persistently bind-mount `/nix/persist/etc` over `/etc`
>   on every subsequent boot. Once these steps are done and you reboot, your
>   user passwords should function correctly, and future rebuilds will > not
>   present this problem.

```bash
sudo nixos-rebuild switch --flake .#hostname # Replace 'hostname' with your actual system hostname
```

3. Perform an Impermanence Test (Before Reboot):

- Before you reboot, create a temporary directory and file in a non-persistent
  location. Since you haven’t explicitly added `/imperm_test` to your
  `environment.persistence."/nix/persist"` directories, this file should not
  survive a reboot.

```bash
mkdir /imperm_test
echo "This should be Gone after Reboot" | sudo tee /imperm_test/testfile
ls -l /imperm_test/testfile # Verify the file exists
cat /imperm_test/testfile # Verify content
```

4. Reboot Your System: For the impermanence setup to take full effect and for
   your root filesystem to be reset for the first time, you must reboot your
   machine.

```bash
sudo reboot
```

5. Verify Impermanence (After Reboot):

- After the system has rebooted, check if the test directory and file still
  exist:

```bash
ls -l /imperm_test/testfile
```

You should see an output like `ls: cannot access '/imperm_test/testfile'`: No
such file or directory. This confirms that the `/imperm_test` directory and its
contents were indeed ephemeral and were removed during the reboot process,
indicating your impermanence setup is working correctly!

Your system should now come up with a fresh root filesystem, and only the data
specified in your `environment.persistence."/nix/persist"` configuration will be
persistent.

### Recovery with nixos-enter and chroot

This is if you followed the minimal_install guide, it will need to be changed
for a different disk layout.

[Chroot](https://en.wikipedia.org/wiki/Chroot) is an operation that changes the
apparent root directory for the current running process and their children. A
program that is run in such a modified environment cannot access files and
commands outside that environmental directory tree. This modified environment is
called a chroot jail. –NixOS wiki

`nixos-enter` allows you to access a NixOS installation from a NixOS rescue
system. To use, setup `/mnt` as described in the
[installation manual](https://nixos.org/manual/nixos/stable/#sec-installation)

🛠️ Recovery: Chroot into Your NixOS Btrfs+Impermanence System

Take note of your layout from commands like:

```bash
sudo fdisk -l
lsblk
sudo btrfs subvol list /
```

Also inspect your `disk-config.nix` to ensure you refer to the correct `subvol=`
names.

If you need to repair your system (e.g., forgot root password, fix a broken
config, etc.), follow these steps to chroot into your NixOS install:

1. Boot a Live ISO

Boot from a NixOS (or any recent Linux) live USB.

Open a terminal and become root:

```bash
sudo -i
```

2. Identify Your Devices

Your main disk is `/dev/nvme0n1`

- EFI partition: `/dev/nvme0n1p1` (mounted at `/boot`)

- Root partition: `/dev/nvme0n1p2` (Btrfs, with subvolumes)

3. Mount the Btrfs Root Subvolume

First, mount the Btrfs partition somewhere temporary (not as / yet):

```bash
mount -o subvol=root,compress=zstd,noatime /dev/nvme0n1p2 /mnt
```

4. Mount Other Subvolumes

Now mount your other subvolumes as defined in your `disko.nix`:

```bash
# Mount Other Subvolumes
# (Ensure /mnt directories are created for each *mountpoint*)

# Home
mkdir -p /mnt/home
mount -o subvol=home,compress=zstd,noatime /dev/nvme0n1p2 /mnt/home

# IMPORTANT: No separate mount for /mnt/home/user, as it's a nested subvolume
# and handled by the /home mount.

# Nix store
mkdir -p /mnt/nix
mount -o subvol=nix,compress=zstd,noatime /dev/nvme0n1p2 /mnt/nix

# Nix persist
mkdir -p /mnt/nix/persist
# CRITICAL: Based our disko.nix, the subvolume name is 'persist', not 'nix/persist'
mount -o subvol=persist,compress=zstd,noatime /dev/nvme0n1p2 /mnt/nix/persist

# /var/log
mkdir -p /mnt/var/log
mount -o subvol=log,compress=zstd,noatime /dev/nvme0n1p2 /mnt/var/log

# /var/lib
mkdir -p /mnt/var/lib
# Confirmed: The subvolume named 'lib' is mounted to /var/lib
mount -o subvol=lib,compress=zstd,noatime /dev/nvme0n1p2 /mnt/var/lib
```

Note: If you get “subvolume not found,” check the subvolume names with
`btrfs subvol list /mnt`.

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

You can now run nixos-rebuild, reset passwords, or fix configs as needed. 🔎

📓 Notes

- Adjust `compress=zstd,noatime` if your config uses different mount options.

- For impermanence, make sure to mount all persistent subvolumes you need.

- If you use swap, you may want to enable it too (e.g., swapon /dev/zram0 if
  relevant).

You can now recover, repair, or maintain your NixOS system as needed!

#### Related Material

- [Change root (chroot](https://wiki.nixos.org/wiki/Change_root)

- [nixos-enter](https://www.mankier.com/8/nixos-enter)

- [erase your darlings](https://grahamc.com/blog/erase-your-darlings/)

- [Guide for Btrfs with LUKS](https://haseebmajid.dev/posts/2024-07-30-how-i-setup-btrfs-and-luks-on-nixos-using-disko/)

- [notashelf impermanence](https://notashelf.dev/posts/impermanence)

- [NixOS wiki Impermanence](https://wiki.nixos.org/wiki/Impermanence)

- [nix-community impermanence module](https://github.com/nix-community/impermanence)
