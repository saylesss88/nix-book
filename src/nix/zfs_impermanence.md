---
title: ZFS Impermanence in a VM
date: 2026-01-16
author: saylesss88
collection: "blog"
tags: ["Virtual Machines", "KVM", "Impermanence"]
draft: false
---

# ZFS Impermanence in a VM

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

I only tested this within a VM although with a few small tweaks it should work
on bare metal. I used the libvirtd stack with KVM for this.

Yet another blog post inspired by
[erase your darlings](https://grahamc.com/blog/erase-your-darlings/)

Start with a minimal ISO.

[Download Minimal (64-bit Intel-AMD)](https://channels.nixos.org/nixos-25.11/latest-nixos-minimal-x86_64-linux.iso)

Choose the LTS image, it comes with the `zfs` module enabled.

I've also found that for my system it works best to switch the Video Model to
Virtio, with 3D accelleration disabled (causes mouse inversion). You can also
set the Display Spice Listen type to `None`, and enable `OpenGL`.

When creating the VM, before clicking "Finish", check the "Customize
configuration before install" box and choose EFI Firmware > BIOS. **You will
waste a bunch of time if you forget to do this**!

- I used `OVMF_CODE.fd` in my testing.

Check out your layout:

```bash
sudo fdisk -l
```

Format your disk:

```bash
sudo cfdisk /dev/vda
```

Create a 1G **EFI System** first, then a **Linux Filesystem** with the remaining
space. I used (100G)

For the following guide, you want `/dev/vda1` to be your **EFI System**
partition, and `/dev/vda2` to be the **Linux Filesystem** partition.

```bash
sudo fdisk -l
```

```bash
sudo mkfs.vfat -n EFI /dev/vda1
```

## Create Your ZFS Partitions

1. Create a zpool:

```bash
zpool create \
  -o ashift=12 \
  -o autotrim=on \
  -O acltype=posixacl \
  -O canmount=off \
  -O dnodesize=auto \
  -O normalization=formD \
  -O relatime=on \
  -O xattr=sa \
  -O mountpoint=none \
  rpool /dev/vda2
```

2. Create all datasets with parents (`-p`):

```bash
# root (ephemeral – will be rolled back)
zfs create -p -o canmount=noauto -o mountpoint=legacy rpool/local/root

# blank snapshot (the “erase” target)
zfs snapshot rpool/local/root@blank

zfs create -p -o mountpoint=legacy rpool/local/boot
# /nix – read-only store, must survive rollbacks
zfs create -p -o mountpoint=legacy rpool/local/nix

# persisted areas
zfs create -p -o mountpoint=legacy rpool/safe/home
zfs create -p -o mountpoint=legacy rpool/safe/persist
```

3. Mount everything under `/mnt`:

```bash
mount -t zfs rpool/local/root /mnt

mkdir -p /mnt/{boot,boot/efi,nix,home,persist}
mount -t vfat -o umask=0077 /dev/vda1 /mnt/boot/efi
mount -t zfs rpool/local/nix   /mnt/nix
mount -t zfs rpool/safe/home  /mnt/home
mount -t zfs rpool/safe/persist /mnt/persist
```

> Note: By placing your Nix flake in `/home/user/nixos-config` (which lives on
> `rpool/safe/home`), it persists naturally. You don't need to add your
> configuration files to the `environment.persistence` module lists because the
> underlying storage isn't being wiped.

4. Continue with the rest of the install

```bash
nixos-generate-config --root /mnt
# edit /mnt/etc/nixos/configuration.nix  (add ZFS + rollback + impermanence)
```

<details>
<summary> ✔️ Quick checklist: </summary>

Quick checklist to confirm that you've taken all of the necessary steps.

```bash
# 1. pool
zpool create -o ashift=12 -o autotrim=on -O acltype=posixacl -O canmount=off \
  -O dnodesize=auto -O normalization=formD -O relatime=on -O xattr=sa \
  -O mountpoint=none rpool /dev/vda2

# 2. datasets + snapshot
zfs create -p -o canmount=noauto -o mountpoint=legacy rpool/local/root
zfs snapshot rpool/local/root@blank
zfs create -p -o mountpoint=legacy rpool/local/nix
zfs create -p -o mountpoint=legacy rpool/safe/home
zfs create -p -o mountpoint=legacy rpool/safe/persist
# add a /boot dataset
zfs create -p -o mountpoint=legacy rpool/local/boot

# 3. mounts
mount -t zfs rpool/local/root /mnt
mkdir -p /mnt/{boot,boot/efi,nix,home,persist}

# /boot on ZFS
mount -t zfs rpool/local/boot /mnt/boot

# ESP on /boot/efi
mount -t vfat -o umask=0077 /dev/vda1 /mnt/boot/efi

mount -t zfs rpool/local/nix /mnt/nix
mount -t zfs rpool/safe/home /mnt/home
mount -t zfs rpool/safe/persist /mnt/persist
```

</details>

## Prep `configuration.nix`

```bash
head -c4 /dev/urandom | xxd -p > /tmp/rand.txt
```

**Create password file in a persistent location**:

```bash
sudo mkdir -p /mnt/persist/etc/nixos-secrets/passwords

# 2) Create the password hash and write it to the persistent file
# Replace "your-password" and "your-user"
sudo sh -c 'mkpasswd -m yescrypt "your-password" > /mnt/persist/etc/nixos-secrets/passwords/your-user'

# 3) Lock down permissions
sudo chown root:root /mnt/persist/etc/nixos-secrets/passwords/your-user
sudo chmod 600 /mnt/persist/etc/nixos-secrets/passwords/your-user
```

- After first reboot, the above files will be placed directly under `/persist/`

You will read `rand.txt` into the `configuration.nix` with `:r /tmp/rand.txt`.

Edit the `/mnt/etc/nixos/configuration.nix`:

```nix
{ config, lib, pkgs, ... }:

{
  # ------------------------------------------------------------------
  # 1. Boot loader – systemd-boot (UEFI only)
  # ------------------------------------------------------------------
  boot.loader = {
    systemd-boot = {
      enable = true;
      consoleMode = "max";           # Full 80×25 console in VM
      editor = false;                # Security – no edit at boot
    };
    efi = {
      canTouchEfiVariables = true;   # libvirt provides /sys/firmware/efi
      efiSysMountPoint = "/boot/efi";    # Our 1 GiB FAT32 partition
    };
  };

  # ------------------------------------------------------------------
  # 2. ZFS support
  # ------------------------------------------------------------------
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs.devNodes = "/dev/";       # Critical for VMs

  # Unique 8-hex hostId (run once in live ISO: head -c4 /dev/urandom | xxd -p)
  networking.hostId = "a1b2c3d4";    # <<<--- replace with your own value

  # ------------------------------------------------------------------
  # 3. Roll-back root to blank snapshot on **every** boot
  # ------------------------------------------------------------------
  boot.initrd.postDeviceCommands = lib.mkAfter ''
    zpool import -N -f rpool
    zfs rollback -r rpool/local/root@blank
    zpool export rpool
  '';

  # ------------------------------------------------------------------
  # 4. Basic system (root password, serial console for VM)
  # ------------------------------------------------------------------
  users.users.root.initialPassword = "changeme";   # change after first login
  boot.kernelParams = [ "console=ttyS0,115200n8" ];

  users.mutableUsers = false;

  users.users.your-user = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    group = "your-user";
    # The location of `hashedPasswordFile` after first reboot
    hashedPasswordFile = "/persist/etc/nixos-secrets/passwords/your-user";
  };

  # This enables `chown -R your-user:your-user`
  users.groups.your-user = { };

  # ------------------------------------------------------------------
  # 5. (Optional) Enable SSH for post-install configuration
  # ------------------------------------------------------------------
  # services.openssh = {
  #  enable = true;
  #  settings.PermitRootLogin = "yes";
  #};

  # ------------------------------------------------------------------
  # 6. Mark /persist as needed for boot
  # ------------------------------------------------------------------
  fileSystems."/persist".neededForBoot = true;
}
```

> NOTE: Before my first rebuild, I had:
>
> ```nix
> boot.initrd.postDeviceCommands = lib.mkAfter ''
>    zfs rollback -r rpool/local/root@blank
> '';
> ```
>
> But noticed in my logs that on reboot the `rpool/local/root@blank` wasn't
> available yet, I assume due to a race condition or something I did or forgot
> to do. Adding the `zpool import` fixed the issue. I mention this in case you
> get different results from your first reboot.

```bash
sudo nixos-install --root /mnt
```

```bash
reboot
```

---

## What gets Wiped vs. What Stays

**What gets wiped?**:

Since we roll back `/`(`rpool/local/root`):

- `/etc` (including system configs) -> WIPED

- `/var` (logs, databases, containers) -> WIPED

- `/root` (the root users home directory) -> WIPED

- `/usr` (though in NixOS this is mostly empty) -> WIPED

**What survives?**:

- `/nix` (mounted from `rpool/local/nix`) -> PERSISTS

- `/boot` (mounted from `rpool/local/boot`) -> PERSISTS

- `/home` (mounted from `rpool/safe/home`) -> PERSISTS

- `/persists` (mounted from `rpool/safe/persist`) -> PERSISTS

**Why this matters for secrets?**

**SSH Host keys** typically live in `/etc/ssh`. Since `/etc` is wiped, they
disappear. Store them in `/persist/etc/ssh` and tell NixOS to look there. (or
symlink them)

**User Secrets** (`~/.config/sops`): They live in `/home` so they're safe.

---

## Integrating into a Flake

After first reboot, I recommend setting up a flake in a persistent location such
as `/home/your-user/flake`. Because subsequent reboots will wipe the `/etc`
directory.

- [Example Flake](https://github.com/saylesss88/flakey), this is a WIP
  adaptation from another flake I had.

```bash
sudo mkdir /imperm_test
echo "This should be Gone after Reboot" | sudo tee /imperm_test/testfile
sudo ls -l /imperm_test/testfile # Verify the file exists
sudo cat /imperm_test/testfile # Verify content
```

Reboot and check again:

```bash
sudo ls -l /imperm_test/testfile # Verify the file no longer exists
sudo cat /imperm_test/testfile # Verify content is missing
```

---

## Persisting SSH Keys

```bash
sudo mkdir -p /persist/etc/ssh
sudo ssh-keygen -t ed25519 -f /persist/etc/ssh/ssh_host_ed25519_key -N ""
```

OR if you still have keys in `/etc/ssh` you want to keep just copy them to the
persistent location:

```bash
sudo cp /etc/ssh/ssh_host_ed25519_key* /persist/etc/ssh/
```

**Tell NixOS where to find them**

```nix
services.openssh = {
  hostKeys = [
    {
      path = "/persist/etc/ssh/ssh_host_ed25519_key";
      type = ed25519;
    }
  ];
}
```

After I initially get things working, I switch to `sops-nix`, the following
guide works for this setup:
[sops-nix Guide](https://saylesss88.github.io/installation/enc/sops-nix.html)

---

### Resources

- [erase-your-darlings](https://grahamc.com/blog/erase-your-darlings/)

- [NixOS Wiki ZFS](https://wiki.nixos.org/wiki/ZFS)
