---
title: ZFS Bare Metal Impermanence
date: 2026-03-01
author: saylesss88
collection: "blog"
tags: ["ZFS", "Impermanence", "Encryption"]
draft: false
---

# ZFS Imperm Bare-Metal

I couldn't get disko to bend to my will so I wrote the following bash script.
The script automates the steps in Graham Christensen's
[Erase your darlings](https://grahamc.com/blog/erase-your-darlings/)

## The Storage Architecture

The script below automates the "Erase Your Darlings" setup. It organizes your
data into three distinct "levels" of persistence:

- **The Volatile (`/`)**: A ZFS dataset that is blank at boot. We take a
  snapshot called @blank immediately after creation. In your NixOS
  configuration, you will set up a boot-time script to roll back to this @blank
  snapshot, effectively "formatting" your root in milliseconds.

- **The Store (`/nix`)**: A separate dataset for the Nix store. This doesn't
  need to be wiped because Nix already manages its own integrity.

- **The Safe (`/persist` and `/home`)**: These datasets hold the things you
  actually care about—your SSH keys, browser profiles, and project files.

**What this script automates**

This bash script handles the "Stage 0" heavy lifting. It will:

1. **Partition** your disk with an EFI boot partition and a LUKS2 encrypted
   container.

2. **Initialize** a ZFS pool (`rpool`) with performance-optimized settings (like
   `ashift=12` and `zstd` compression).

3. **Carve** out the datasets required for an Impermanence setup.

4. **Mount** the hierarchy into `/mnt` so `nixos-generate-config` can detect the
   specialized ZFS layout.

> **WARNING**: This is a destructive operation. Running this script will wipe
> the target drive completely. Ensure you have backed up any existing data
> before proceeding.

- [The Setup Script](https://github.com/saylesss88/my-flake2/blob/main/install.sh)

## Quick Start

1. Start with the minimal ISO:

- [NixOS Downloads](https://nixos.org/download/)

- [NixOS Manual Installation](https://nixos.org/manual/nixos/stable/index.html#sec-installation-manual)

- The script handles the partitioning, formatting, mounting, and lastly, runs
  `nixos-generate-config --root /mnt`. After running the script, edit the files
  in the repo matching your user and device. Finally, after you're sure you
  haven't missed anything, run `nixos-install`.

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
```

2. Clone the [starter repo](https://github.com/saylesss88/my-flake2#):

```bash
git clone https://github.com/saylesss88/my-flake2.git
```

3. Inspect & Run the script provided with the repo & follow prompts.:

> **WARNING**: This is a destructive operation. Running this script will wipe
> the target drive completely. Ensure you have backed up any existing data
> before proceeding.

```bash
sudo chmod +x ./install.sh
sudo bash ./install.sh
```

- I tested the script on an `nvme0n1` drive with no issues.

4. Run the following commands:

```bash
# Get your UUID#
sudo blkid /dev/YOUR_DISK > /tmp/blk.txt
# Generate a hashed password
mkpasswd -m yescrypt > /tmp/pass.txt
# Generate a rand # for `networking.hostId`
head -c4 /dev/urandom | xxd -p > /tmp/rand.txt
```

5. Edit `flake.nix`, `configuration.nix`, and replace the repos
   `hardware-configuration.nix` with your own.

6. Add `neededForBoot` to the `home` and `persist` datasets in the generated
   `hardware-configuration.nix`.

Example:

```nix
  fileSystems."/home" = {
    device = "rpool/safe/home";
    fsType = "zfs";
    neededForBoot = true;
  };

  fileSystems."/persist" = {
    device = "rpool/safe/persist";
    fsType = "zfs";
    neededForBoot = true;
  };
```

7. Move the flake to `/mnt/etc/nixos/`

```bash
sudo mv ~/my-flake2 /mnt/etc/nixos/
```

8. Do a final check and install (change `host` to your host name)

```bash
sudo nixos-install --flake /mnt/etc/nixos/myflake2#host
```

- Read the comments, they let you know of requirements.

9. Reboot. I typically run `nixos-install` with the minimal requirements,
   reboot, and then configure my window manager/DE.

10. After reboot, adjust permissions for your `$USER`:

```bash
sudo mkdir -p /persist/home/$USER
# Set ownership for the persistent home directory
sudo chown -R 1000:100 /persist/home/$USER

# Ensure the home dataset itself is accessible
sudo chmod 755 /home
sudo chmod 755 /persist/home
# Test file, should be gone after reboot
sudo touch /etc/rollback-canary
```

10. Uncomment the import of the impermanence module in the `configuration.nix`.

11. Reboot, then check:

```bash
sudo ls /etc/rollback-canary
```

- You should get an error:
  `"/etc/rollback-canary": No such file or directory (os error 2)`

---

## What gets Wiped vs. What Stays

What gets wiped?:

Since we roll back (`rpool/local/root`):

- `/etc` (including system configs) -> WIPED

- `/var` (logs, databases, containers) -> WIPED

- `/root` (the root users home directory) -> WIPED

- `/usr` (though in NixOS this is mostly empty) -> WIPED

What survives?:

- `/nix` (mounted from `rpool/local/nix`) -> PERSISTS

- `/boot` (mounted from `rpool/local/boot`) -> PERSISTS

- `/home` (mounted from `rpool/safe/home`) -> PERSISTS

- `/persists` (mounted from `rpool/safe/persist`) -> PERSISTS
