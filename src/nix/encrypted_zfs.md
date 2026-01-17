---
title: ZFS with LUKS and Impermanence
date: 2026-01-17
author: saylesss88
collection: "blog"
tags: ["Virtual Machines", "KVM", "Impermanence", "Encryption"]
draft: false
---

# ZFS with LUKS and Impermanence

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

I tested this on the libvirt stack with KVM, this should work on bare metal with
a few omissions.

<details>
<summary> ✔️ SSH Method </summary>

This saves a ton of typing...

1. Boot the minimal ISO

2. Set a password for the `nixos` user: `sudo passwd nixos`

3. Find the IP address: `ip a` (look for `etho` or `wlan0`)

4. SSH in from your host or another machine: `ssh nixos@192.168.1.x`

</details>

When creating the VM, before clicking "Finish", check the "Customize
configuration before install" box and choose EFI Firmware > BIOS. **You will
waste a bunch of time if you forget to do this**!

- I used `OVMF_CODE.fd` in my testing.

**Format your disk**

1. Partition & Format

```bash
sudo cfdisk /dev/vda
sudo mkfs.fat -F32 /dev/vda1
```

2. Setup LUKS

```bash
sudo cryptsetup luksFormat /dev/vda2
sudo cryptsetup open /dev/vda2 cryptroot
```

3. Create zpool

```bash
sudo zpool create \
  -o ashift=12 \
  -o autotrim=on \
  -O acltype=posixacl \
  -O canmount=off \
  -O compression=zstd \
  -O normalization=formD \
  -O relatime=on \
  -O xattr=sa \
  -O mountpoint=none \
  rpool /dev/mapper/cryptroot
```

5. Dataset Creation

```bash
# root (ephemeral)
sudo zfs create -p -o canmount=noauto -o mountpoint=legacy rpool/local/root
sudo zfs snapshot rpool/local/root@blank

# nix store
sudo zfs create -p -o mountpoint=legacy rpool/local/nix

# persistent data
sudo zfs create -p -o mountpoint=legacy rpool/safe/home
sudo zfs create -p -o mountpoint=legacy rpool/safe/persist
```

6. Mounting

```bash
# 1. Mount root first
sudo mount -t zfs rpool/local/root /mnt

# 2. Create directories
sudo mkdir -p /mnt/{nix,home,persist,boot}

# 3. Mount ESP directly to /boot (simpler and safer for systemd-boot)
sudo mount -t vfat -o umask=0077 /dev/vda1 /mnt/boot

# 4. Mount other ZFS datasets
sudo mount -t zfs rpool/local/nix /mnt/nix
sudo mount -t zfs rpool/safe/home /mnt/home
sudo mount -t zfs rpool/safe/persist /mnt/persist
```

7. Configuration Prep

```bash
sudo nixos-generate-config --root /mnt
```

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
nix-shell -p helix
```

```bash
sudo blkid /dev/vda2
# Copy the uuid
```

```nix
# configuration.nix
 boot.initrd.luks.devices = {
     cryptroot = {
       device = "/dev/disk/by-uuid/uuid#";
       allowDiscards = true;
       preLVM = true;
     };
   };
```

```nix
boot.initrd.luks.devices."cryptroot".device = "/dev/disk/by-uuid/<UUID-OF-PARTITION-2>";
```

## Prep `configuration.nix`

```bash
head -c4 /dev/urandom | xxd -p > /tmp/rand.txt
```

**Create password file in a persistent location**:

```bash
sudo mkdir -p /mnt/persist/etc/nixos-secrets/passwords

 #2) Create the password hash and write it to the persistent file
 #Replace "your-password" and "your-user"
sudo sh -c 'mkpasswd -m yescrypt "your-password" > /mnt/persist/etc/nixos-secrets/passwords/your-user'

 #3) Lock down permissions
sudo chown root:root /mnt/persist/etc/nixos-secrets/passwords/your-user
sudo chmod 600 /mnt/persist/etc/nixos-secrets/passwords/your-user

 #4) Optionally create an admin user, this is for `initialHashedPassword`
 #   Read this in with `:r /tmp/pass.txt`
mkpasswd --method=yescrypt > /tmp/pass.txt
```

After first reboot, the above files will be placed directly under `/persist/`

```nix
{ config, lib, pkgs, ... }:

{
  # ------------------------------------------------------------------
  # 1. Boot loader – systemd-boot (UEFI only)
  # ------------------------------------------------------------------
  boot.loader = {
    systemd-boot = {
      enable = true;
      consoleMode = "max";
      editor = false;
    };
    efi = {
      canTouchEfiVariables = true;
      efiSysMountPoint = "/boot";
    };
  };

  # ------------------------------------------------------------------
  # 2. ZFS support
  # ------------------------------------------------------------------
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs.devNodes = "/dev/";       # Critical for VMs
  # Not needed with LUKS
  boot.zfs.requestEncryptionCredentials = false;

  # ------------------------------------------------------------------
  # 3. LUKS
  # ------------------------------------------------------------------
   boot.initrd.luks.devices = {
     cryptroot = {
    # replace uuid# with output of `sudo blkid /dev/vda2`
       device = "/dev/disk/by-uuid/uuid#";
       allowDiscards = true;
       preLVM = true;
     };
   };

  # ------------------------------------------------------------------
  # 4. Roll-back root to blank snapshot on **every** boot
  # ------------------------------------------------------------------
 # Uncomment after first reboot
 # boot.initrd.postDeviceCommands = lib.mkAfter ''
 #   zpool import -N -f rpool
 #   zfs rollback -r rpool/local/root@blank
 #   zpool export rpool
 # '';

  # ------------------------------------------------------------------
  # 5. Basic system (root password, serial console for VM)
  # ------------------------------------------------------------------
  # Unique 8-hex hostId (run once in live ISO: head -c4 /dev/urandom | xxd -p)
  networking.hostId = "a1b2c3d4";    # <<<--- replace with your own value

  users.users.root.initialPassword = "changeme";   # change after first login

  boot.kernelParams = [ "console=tty1" ];

  # ------------------------------------------------------------------
  #  Users
  # ------------------------------------------------------------------

  users.mutableUsers = false;

  # Change `your-user`
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
  #  (Optional) Helpful for recovery situations
  # ------------------------------------------------------------------
  # users.users.admin = {
  #  isNormalUser = true;
  #  description = "admin account";
  #  extraGroups = [ "wheel" ];
  #  group = "admin";
    # hashedPasswordFile = config.sops.secrets.password_hash.path;
    # initialHashedPassword = "Output of `:r /tmp/pass.txt`";
 # };

 # users.groups.admin = { };
  # ------------------------------------------------------------------

  # ------------------------------------------------------------------
  # 6. (Optional) Enable SSH for post-install configuration
  # ------------------------------------------------------------------
  # services.openssh = {
  #  enable = true;
  #  settings.PermitRootLogin = "yes";
  #};

  # ------------------------------------------------------------------
  # 7. Mark /persist as needed for boot
  # ------------------------------------------------------------------
  fileSystems."/persist".neededForBoot = true;
}
```

I ran `nixos-install` and rebooted once before uncommenting:

```nix
  boot.initrd.postDeviceCommands = lib.mkAfter ''
    zpool import -N -f rpool
    zfs rollback -r rpool/local/root@blank
    zpool export rpool
  '';
```

Rebuilding and testing, Reboot, uncomment the above script and test: (Don't
forget that the `/etc` directory will be wiped, including your
`configuration.nix` and `hardware-configuration.nix`!)

**Configuration backup**

```bash
sudo mkdir -p /persist/etc
sudo cp /etc/nixos/hardware-configuration.nix /etc/nixos/configuration.nix /persist/etc/
```

**Rollback Test**

```bash
sudo touch /etc/rollback-canary
sudo reboot
```

If the rollback is working, `/etc/rollback-canary` should be gone after reboot
(while things in `/persist` remain).

## Adding a disk serial (libvirt XML)

NixOS ZFS boot support is broken for virtio drives without serial numbers.
Virtio disks without serials don't appear in /dev/disk/by-id, but ZFS boot logic
only tries to import pools from /dev/disk/by-id. The official OpenZFS NixOS
documentation explicitly states: "If virtio is used as disk bus, power off the
VM and set serial numbers for disk"

In the `<disk ...>` block of your root disk add:

Add to `.zshrc`:

```bash
export LIBVIRT_DEFAULT_URI="qemu:///system"
```

```bash
virsh list --all
virsh edit nixos-unstable
```

In the first `<disk ... device='disk'>` section (the one with target `dev='vda'`
`bus='virtio'`), add a `<serial>` line, e.g.:

```xml
<disk type='file' device='disk'>
  <driver name='qemu' type='qcow2' discard='unmap'/>
  <source file='/var/lib/libvirt/images/nixos-unstable-1.qcow2' index='2'/>
  <backingStore/>
  <target dev='vda' bus='virtio'/>
  <serial>disk01</serial>
  <alias name='virtio-disk0'/>
  ...
</disk>
```
