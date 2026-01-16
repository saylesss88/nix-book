---
title: Encrypted Impermanence
date: 2025-12-06
author: saylesss88
collection: "blog"
tags: ["impermanence", "security"]
draft: false
---

# Encrypted Impermanence

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

> ❗ Important Note: This guide details a setup involving encrypted partitions
> and impermanent NixOS. While powerful, such configurations require careful
> attention to detail. Incorrect steps, especially concerning encryption keys or
> persistent data paths, can lead to **permanent data loss**. Please read all
> instructions thoroughly before proceeding and consider backing up any critical
> data beforehand. This has only been tested with the disk layout described in
> [Encrypted Setups](https://saylesss88.github.io/installation/encrypted_manual.html)

### Getting Started

1. Add impermanence to your `flake.nix`. You will change the `hostname` in the
   flake to match your `networking.hostName`.

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

If you followed the
[Encrypted Setups](https://saylesss88.github.io/installation/encrypted_manual.html)
guide, your encrypted subvolume should be located at:
`/dev/mapper/cryptroot /mnt`

- Your encrypted Btrfs partition, once unlocked by LUKS, will be available at
  `/dev/mapper/cryptroot` as configured here in the `disk-config.nix`:

```nix
# disk-config2.nix
# ... snip ...
            luks = {
              size = "100%";
              label = "luks";
              content = {
                type = "luks";
                name = "cryptroot";
                content = {
# ... snip ...
```

Double check that the paths exist:

```bash
cd /dev/mapper/crypt<TAB>  # autocomplete should fill out /dev/mapper/cryptroot
```

3. Create an `impermanence.nix`:

Now, create a new file named `impermanence.nix` in your configuration directory
(i.e. your flake directory). This file will contain all the specific settings
for your impermanent setup, including BTRFS subvolume management and persistent
data locations. Since this file is right next to your `configuration.nix`,
you'll just add an `imports = [ ./impermanence.nix ]` to your
`configuration.nix` apply it to your configuration.

```nix
{
  config,
  lib,
  ...
}: {
  boot.initrd.postDeviceCommands = lib.mkAfter ''
    echo "Rollback running" > /mnt/rollback.log
     mkdir -p /mnt
     mount -t btrfs /dev/mapper/cryptroot /mnt

     # Recursively delete all nested subvolumes inside /mnt/root
     btrfs subvolume list -o /mnt/root | cut -f9 -d' ' | while read subvolume; do
       echo "Deleting /$subvolume subvolume..." >> /mnt/rollback.log
       btrfs subvolume delete "/mnt/$subvolume"
     done

     echo "Deleting /root subvolume..." >> /mnt/rollback.log
     btrfs subvolume delete /mnt/root

     echo "Restoring blank /root subvolume..." >> /mnt/rollback.log
     btrfs subvolume snapshot /mnt/root-blank /mnt/root

     umount /mnt
  '';

  environment.persistence."/persist" = {
    directories = [
      "/etc"
      "/var/spool"
      "/srv"
      "/etc/NetworkManager/system-connections"
      "/var/lib/bluetooth"
    ];
    files = [
      # "/etc/machine-id"
      # Add more files you want to persist
    ];
  };

# optional quality of life setting
  security.sudo.extraConfig = ''
    Defaults lecture = never
  '';
}
```

- `/mnt/rollback.log`: this log will be available during the boot process for
  debugging if the rollback fails, but won't persist.

With the above impermanence script, the btrfs subvolumes are deleted recursively
and replaced with the `root-blank` snapshot we took during the install.

I have commented out `"/etc/machine-id"` because we already copied over all of
the files to their persistent location and the above setting would work once and
then cause a conflict.

## configuration.nix changes

```nix
# configuration.nix
  boot.initrd.luks.devices = {
    cryptroot = {
      device = "/dev/disk/by-partlabel/luks";
      allowDiscards = true;
      preLVM = true;
    };
  };
```

- This defines how your system's initial ramdisk (`initrd`) should handle a
  specific encrypted disk during the boot process. It helps with timing and is a
  more robust way of telling Nix that we are using an encrypted disk.

The following is optional to enable `autoScrub` for btrfs, the wiki shows
`interval = "monthly";` FYI.

```nix
# configuration.nix
  services.btrfs.autoScrub = {
    enable = true;
    interval = "weekly";
    fileSystems = ["/"];
  };
```

- Remember to ensure that your `hostname` in your `configuration.nix` matches
  the `hostname` in your `flake.nix`.

### Applying Your Impermanence Configuration

Once you have completed all the steps and created or modified the necessary
files (`flake.nix`, `impermanence.nix`), you need to apply these changes to your
NixOS system.

1. Navigate to your NixOS configuration directory (where your `flake.nix` is
   located).

```bash
cd /path/to/your/flake
```

2. Rebuild and Switch: Execute the `nixos-rebuild switch` command. This command
   will:

- Evaluate your `flake.nix` and the modules it imports (including your new
  `impermanence.nix`).

- Build a new NixOS system closure based on your updated configuration.

- Activate the new system configuration, making it the current running system.

```bash
sudo nixos-rebuild switch --flake .#hostname # Replace 'hostname' with your actual system hostname
```

3. Perform an Impermanence Test (Before Reboot):

- Before you reboot, create a temporary directory and file in a non-persistent
  location. Since you haven't explicitly added `/imperm_test` to your
  `environment.persistence."/persist"` directories, this file should not survive
  a reboot.

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
specified in your `environment.persistence."/persist"` configuration will be
persistent.
