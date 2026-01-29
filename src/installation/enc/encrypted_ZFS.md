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

- [Starter Repo containing a flake and the configuration.nix from this chapter](https://github.com/saylesss88/my-flake2)

```bash
git clone https://github.com/saylesss88/my-flake2.git
```

</details>

## What is OpenZFS

ZFS is an advanced filesystem, originally developed by Sun Microsystems in 05.

OpenZFS is a fork of the proprietary Oracle ZFS, it was forked over a decade
ago. Most of the code remains the same so you can check the
[Oracle website](https://docs.oracle.com/en/storage/zfs-storage/index.html) for
docs and administration guides.

ZFS is licensed under the
[Common Development and Distribution License](https://en.wikipedia.org/wiki/CDDL)
(CDDL). Because the CDDL is incompatible with the GPL,
[it is not possible](https://sfconservancy.org/blog/2016/feb/25/zfs-and-linux/)
for ZFS to be included in the Linux Kernel. This requirement, however, does not
prevent a native Linux kernel module from being developed and distributed by a
third party, as is the case with [OpenZFS](https://openzfs.org/) (previously
named ZFS on Linux). --arch wiki

## Comparison: OpenZFS Native Encryption vs. LUKS

<details>
<summary> ✔️ OpenZFS Native Encryption vs. LUKS </summary>

> NOTE: This isn't an attack on ZFS Native Encryption, I'm just presenting the
> information, the choice is yours. Unless you have a high threat model, ZFS
> native encryption has many benefits such improved flexibility and
> authentication.

- **LUKS (Device Layer)**: LUKS operates on the block device (like
  `/dev/nvme0n1`). It knows nothing about files, folders, or datasets. It just
  sees a stream of bytes and scrambles them.
  - Everything on the partition is encrypted. The filesystem sits inside the
    encrypted container.
  - With standard LUKS, an adversary can see you are using encryption (the LUKS
    header is visible), but they cannot determine what filesystem (ZFS, ext4,
    etc.) is inside. If you go a step further and use a detached header, the
    entire drive looks like random noise, providing plausible deniability that
    any data exists at all.

- **ZFS Native (Filesystem Layer)**: ZFS encryption operates at the Dataset
  (Filesystem) level. It is aware of the structure of your data.
  - Only the **file blocks** (the actual content of your files) and the **file
    attributes** (ACLs, permissions) within a specific dataset are encrypted.

No matter how careful you are with ZFS Native encryption, these cannot be
hidden:

- **The Structure**: The fact that `dataset A` is a child of `dataset B`

- **The Volume**: The exact amount of disk space used by each dataset (in bytes)

- **The Activity**: The fact that _something_ changed at a specific time (via
  snapshot creation or `used` property changing)

> By using generic dataset names (e.g., `data/vol1` instead of
> `data/mistress_photos`) and automated snapshot schedules, you can strip the
> semantic value from the exposed metadata. An attacker will see that you have
> data and how much you have, but they won't know what it is or which dataset is
> the valuable one.

**Sanitizing your ZFS usage (Mitigating risk)**:

By following a few simple best practices you can mitigate most, if not all of
the risk depending on the situation.

- Use generic IDs for dataset names, instead of `tank/mistress_photos` use
  `tank/vol-A`

- Automate your snapshots to prevent traffic analysis

- Use padding to mitigate size analysis, add dummy files to obfuscate the size
  (Can be tedious in ZFS). This is the hardest to mitigate, if this is a real
  threat, use LUKS instead.

OpenZFS native encryption allows you to transparently encrypt data at rest
within ZFS itself. It was initially released in May 2019, giving it much less
time to be scrutinized compared to LUKS (initial release in 2004).

OpenZFS Encryption operates at the dataset layer, not the disk layer, which
creates several critical security gaps that full-disk encryption (FDE) solutions
like LUKS completely avoid.

Unlike LUKS, which presents a "black box" of random noise to anyone without the
key, ZFS Native Encryption must leave certain structural elements visible to the
operating system. This is a deliberate design choice that allows ZFS to perform
maintenance tasks (like scrubbing) on locked datasets without requiring the user
to type in a password. (Depending on your threat model, this can be a deal
breaker).

---

## Threat Example

**Example: Threat Model where ZFS Native Encryption falls short compared to
LUKS**

1. The "Pattern of Life" Attack (Timestamp Leaks)

**The Leak**: ZFS snapshot names and creation times are visible in plaintext
(`zfs list -t snapshot`)

**The Threat Model**: An adversary monitoring your backups or seizing your drive
can build a profile of your behavior without decrypting a single byte.

- If you claim to be asleep at 3AM, but `zfs list` shows snapshots being created
  or data changing size at 3:15 AM, your alibi is broken.

- Or, say a whistleblower contacts a journalist. An adversary seizes the
  journalist's laptop. They can't read the files, but they see a new dataset
  grew by exactly 5GB at the exact time the leak occured. Correlation = Guilt.

**Compress & Encrypt**:

It's actually a common misconception that LUKS prevents compression before
encryption, both methods are able to do this.

- ZFS Native: `Data -> Compress -> Encrypt -> Checksum -> Write to Disk`

- ZFS on LUKS: `Data -> Compress -> ZFS Write -> LUKS Encrypt -> Write to Disk`

Both methods save disk space.

**Integrety and Authentication**:

This is the strongest architectural argument for ZFS Native.

- ZFS Native: Uses AES-GCM (default) which is an Authenticated Encryption with
  Associated Data (AEAD) mode. If a single bit is flipped on disk (maliciously
  or accidentally), ZFS refuses to return the bad data and reports a checksum
  error.
  - NOTE: You can get integrity with LUKS2 with `dm-integrity`, but it incurs a
    massive performance penalty and is considered experimental for production
    use.

---

## ZFS Native Encryption OR LUKS

**Use LUKS if**:

- You can't afford to leak metadata like dataset names or snapshot timestamps.
  LUKS makes the drive look like random noise.

- You require rock-solid stability. The ZFS-on-LUKS stack is very
  "battle-tested".

**Use ZFS Native Encryption if**:

- You have "Untrusted" Offsite Backups: This is a really nice feature! You can
  `zfs send -w` your data to a friend's server or a cloud VM. The remote host
  cannot mount or read your data because they never have the key, but they can
  still scrub the pool and verify the data integrity.

- You need granular control: ZFS Native Encryption allows you to for example,
  keep your OS root unencrypted for fast booting/repair, but your `/home`
  directory encrypted.

- You run many VMs/Containers: You can create a new encrypted dataset for a
  specific project/client without repartitioning or creating loopback files.

- You want to verify data authenticity.

- [arsTechnica quick start to openzfs-native-encryption](https://arstechnica.com/gadgets/2021/06/a-quick-start-guide-to-openzfs-native-encryption/)

- [Practical ZFS is native encryption ready for production use?](https://discourse.practicalzfs.com/t/is-native-encryption-ready-for-production-use/532)

</details>

## Getting Started

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

3. Create zpool (Edited 2026-01-18 normalization=none)

```bash
sudo zpool create \
  -o ashift=12 \
  -o autotrim=on \
  -O acltype=posixacl \
  -O canmount=off \
  -O compression=zstd \
  -O normalization=none \
  -O relatime=on \
  -O xattr=sa \
  -O dnodesize=auto \
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

- `mountpoint=legacy` means that systemd will take care of the mounting

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

---

## Prep `configuration.nix`

```bash
head -c4 /dev/urandom | xxd -p > /tmp/rand.txt
```

**Create password file in a persistent location**:

```bash
sudo mkdir -p /mnt/persist/etc/nixos-secrets/passwords

 #1) This is for `initialHashedPassword`
 #   Read this in with `:r /tmp/pass.txt`
mkpasswd --method=yescrypt > /tmp/pass.txt
```

After first reboot, the above files will be placed directly under `/persist/`

(Edited 2026-01-18 use `postMountCommands` > `postResumeCommands`)

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
  # 2. ZFS support see: https://openzfs.github.io/openzfs-docs/Getting%20Started/NixOS/index.html
  # ------------------------------------------------------------------
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs.devNodes = "/dev/";       # Critical for VMs
  # Not needed with LUKS
  boot.zfs.requestEncryptionCredentials = false;
  # systemd handles mounting
  systemd.services.zfs-mount.enable = false;

  services.zfs = {
    autoScrub.enable = true;
    # periodically runs `zpool trim`
    trim.enable = true;
    # autoSnapshot = true;
  };

  # ------------------------------------------------------------------
  # 3. LUKS
  # ------------------------------------------------------------------
   boot.initrd.luks.devices = {
     cryptroot = {
    # replace uuid# with output of UUID # from `sudo blkid /dev/vda2`
       device = "/dev/disk/by-uuid/uuid#";
       allowDiscards = true;
       preLVM = true;
     };
   };

  # ------------------------------------------------------------------
  # 4. Roll-back root to blank snapshot on **every** boot
  # ------------------------------------------------------------------
 # Uncomment after first reboot
 # boot.initrd.postMountCommands = lib.mkAfter ''
 #   zfs rollback -r rpool/local/root@blank
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
    # :r /tmp/pass.txt:
    initialHashedPassword = "";
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

After reboot, you can uncomment:

```nix
  boot.initrd.postMountCommands = lib.mkAfter ''
    zfs rollback -r rpool/local/root@blank
  '';
```

Uncomment the above script and test: (Don't forget that the `/etc` directory
will be wiped, including your `configuration.nix` and
`hardware-configuration.nix`!)

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

---

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

---

## Issues I've Come Across

**sops-nix README**: Explicitly warns: "If you are using Impermanence, the key
used for secret decryption ... must be in a persisted directory, loaded early
enough during boot." It specifically cites activation timing as the reason this
fails.

Typically the solution was to add `neededForUsers = true;` in your
"password_hash" block but that isn't working for this setup. (ZFS/LUKS/Imperm)

I've been using `initialHashedPassword`, which bypasses the race entirely by
baking the salted hash into the store. This is all that I've found that works so
far...

- Without adding the `<serial>disk01</serial>` to the XML when you reboot, your
  system will hang before asking for your LUKS password.

- Without clicking "Customize configuration before install" box and choosing EFI
  Firmware instead of the default BIOS, **you will not be able to boot at all**.
  This seems to be the case for all custom disk layouts with NixOS on the
  libvirt stack.

---

### Resources

<details>
<summary> ✔️ Resources </summary>

- [openzfs-docs](https://openzfs.github.io/openzfs-docs/)

- [openzfs-docs NixOS Root on ZFS](https://openzfs.github.io/openzfs-docs/Getting%20Started/NixOS/Root%20on%20ZFS.html)

- [NixOS Wiki ZFS](https://wiki.nixos.org/wiki/ZFS)

- [klarasystems OpenZFS: Security, Encryption, and Delegation Sept 2025](https://klarasystems.com/articles/keeping-data-safe-with-openzfs-security-encryption-delegation/)

- [klarasystems Improving Replication Security with OpenZFS Delegation](https://klarasystems.com/articles/improving-replication-security-with-openzfs-delegation/)

- [ZFS Guide for starters & advanced users](https://forum.level1techs.com/t/zfs-guide-for-starters-and-advanced-users-concepts-pool-config-tuning-troubleshooting/196035)

- [OpenZFS Sysem Administration](https://openzfs.org/wiki/System_Administration)

- [Oracle Solaris ZFS Admin Guide](https://docs.oracle.com/cd/E19253-01/819-5461/)

- [FreeBSD Handbook Chapter 22 The Z File System (ZFS)](https://docs.freebsd.org/en/books/handbook/zfs/)

- [awesome-zfs](https://github.com/ankek/awesome-zfs)

- [arsTechnica Storage Fundamentals](https://arstechnica.com/series/storage-fundamentals/)

- [sanoid](https://github.com/jimsalterjrs/sanoid/)

- [Practical ZFS](https://discourse.practicalzfs.com/)

- [openzfs/zfs GH Repo](https://github.com/openzfs/zfs)

- [openzfs Issues](https://github.com/openzfs/zfs/issues)

- [zfsonlinux.org](https://zfsonlinux.org/)

- [arch wiki ZFS](https://wiki.archlinux.org/title/ZFS)

- [tech-couch Btrfs Vs ZFS](https://tech-couch.com/post/btrfs-vs-zfs)

- [PureStorage btrfs vs zfs](https://blog.purestorage.com/purely-educational/btrfs-vs-zfs/)

- [The Zettabyte File System design docs](https://www.cs.hmc.edu/~rhodes/cs134/readings/The%20Zettabyte%20File%20System.pdf)

- [Understanding ZFS, the Zettabyte File System](https://mrczntt.com/blog/understanding-zfs-the-zettabyte-file-system/)

- [ZFS - Wikipedia](https://en.wikipedia.org/wiki/ZFS)

</details>
