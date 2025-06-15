# Unencrypted BTRFS Impermanence with Flakes

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

Figure 1: **Impermanence Logo**: Image of the Impermanence logo. Sourced from
the

[Impermanence repo](https://github.com/nix-community/impermanence)

![Impermanence Logo](../images/Impermanence.png)

This guide is for an unencrypted setup, there are a few links at the end for
encrypted setups. This guide follows the previous
[minimal install guide](https://saylesss88.github.io/nix/impermanence.html) but
you should be able to adjust it carefully to meet your needs.

This section details how to set up impermanence on your NixOS system using BTRFS
subvolumes. With impermanence, your operating system's root filesystem will
reset to a pristine state on each reboot, while designated directories and files
remain persistent. This provides a highly reliable and rollback-friendly system.

## Impermanence: The Concept and Its BTRFS Implementation

In a traditional Linux system, most of this state is stored on the disk and
persists indefinitely unless manually deleted or modified. However, this can
lead to configuration drift, where the system accumulates changes (e.g., log
files, temporary files, or unintended configuration tweaks) that make it harder
to reproduce or maintain.

Impermanence, in the context of operating systems, refers to a setup where the
majority of the system's root filesystem (`/`) is reset to a pristine state on
every reboot. This means any changes made to the system (e.g., installing new
packages, modifying system files outside of configuration management, creating
temporary files) are discarded upon shutdown or reboot.

### What Does Impermanence Do?

Impermanence is a NixOS approach that makes the system stateless (or nearly
stateless) by wiping the root filesystem (`/`) on each boot, ensuring a clean,
predictable starting point. Only explicitly designated data (persistent state)
is preserved across reboots, typically stored in specific locations like the
`/persist` subvolume. This achieves:

1. Clean Root Filesystem:

- The root subvolume is deleted and recreated on each boot, erasing transient
  state (e.g., temporary files, runtime data).

- This ensures the system starts fresh, reducing clutter and making it behave
  closer to a declarative system defined by your NixOS configuration.

2. Selective Persistence:

- Critical state (e.g., user files, logs, system configuration) is preserved in
  designated persistent subvolumes (e.g., `/persist`, `/var/log`, `/var/lib`) or
  files.

- You control exactly what state persists by configuring
  `environment.persistence."/persist"` or other mechanisms.

3. Reproducibility and Security:

- By wiping transient state, impermanence prevents unintended changes from
  accumulating, making the system more reproducible.

- It enhances security by ensuring sensitive temporary data (e.g., `/tmp`,
  runtime credentials) is erased on reboot.

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

Before configuring impermanence, it's crucial to know the device path and
subvolume path of your main BTRFS partition where the root filesystem (`/`) is
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

3. Create an `impermanence.nix`:

Now, create a new file named `impermanence.nix` in your configuration directory
(i.e. your flake directory). This file will contain all the specific settings
for your impermanent setup, including BTRFS subvolume management and persistent
data locations

```nix
{lib, ...}: {
  # Reset root subvolume on boot
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
  environment.persistence."/persist" = {
    hideMounts = true;
    directories = [
      "/etc" # System configuration (Keep this here for persistence via bind-mount)
      "/var/spool" # Mail queues, cron jobs
      "/srv" # Web server data, etc.
      "/root" # Root user's home
      # "/var/log" # Persist logs are handled by disko
    ];
    files = [
      #"/persist/swapfile" # Persist swapfile (impermanence manages this file)
    ];
  };

  # Swapfile configuration (definition for Systemd)
  swapDevices = [
    {
      device = "/persist/swapfile"; # Points to the persistent location of the swapfile
      size = 8192; # 8 GB in MiB
    }
  ];

  # --- SWAPFILE INITIALIZATION & FORMATTING (CRITICAL for activation) ---
  # 1. Ensure the swapfile exists at the specified size with correct permissions early via tmpfiles.
  #    The ${toString (8 * 1024 * 1024 * 1024)} converts 8GB to bytes.
  systemd.tmpfiles.rules = [
    "f /persist/swapfile 0600 - - ${toString (8 * 1024 * 1024 * 1024)} -"
  ];

  # 2. Format the swapfile *only if it's not already formatted* during boot.
  boot.initrd.postDeviceCommands = lib.mkAfter ''
    if ! blkid -p /persist/swapfile | grep -q 'TYPE="swap"'; then
      echo "NixOS: Formatting /persist/swapfile..."
      mkswap /persist/swapfile
    fi
  '';
  # --- END SWAPFILE INITIALIZATION & FORMATTING ---
}
```

> ❗ NOTE: While `"/persist"` is perfectly functional and valid,
> `"/nix/persist"` (or often `/var/lib/impermanence` with tools like
> `impermanence`) has emerged as a very common and somewhat "standard" location
> in the NixOS community for the persistent data. If you choose to go for
> `"/nix/persist"` here, make sure to match
> `  environment.persistence."/nix/persist" = {` in your `impermanence.nix`

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

#### Related Material

- [erase your darlings](https://grahamc.com/blog/erase-your-darlings/)

- [Guide for Btrfs with LUKS](https://haseebmajid.dev/posts/2024-07-30-how-i-setup-btrfs-and-luks-on-nixos-using-disko/)

- [notashelf impermanence](https://notashelf.dev/posts/impermanence)

- [NixOS wiki Impermanence](https://wiki.nixos.org/wiki/Impermanence)

- [nix-community impermanence module](https://github.com/nix-community/impermanence)
