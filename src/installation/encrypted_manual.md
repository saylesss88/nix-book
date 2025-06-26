# Encrypted Manual Setup

This guide walks you through a manual, encrypted NixOS installation using Disko
for disk management and Btrfs for subvolumes. It is designed for users who want
full disk encryption and a modern filesystem layout. If you prefer an
unencrypted setup, you can skip the LUKS and encryption steps, but this guide
focuses on security and flexibility.

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
ping google.com
```

3. Get your Disk Name with `lsblk`

The output should be something like:

```bash
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
nvme0n1     259:0    0   1,8T  0 disk
```

4. Copy the disk configuration to your machine. You can choose one from the
   [examples directory](https://github.com/nix-community/disko/tree/master/example).

- There is still a starter repo that can save you some typing, make sure to
  carefully review if you decide to use it:

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
export EDITOR='hx' # or 'vi'
nix-shell -p git yazi helix mkpasswd
git config --global user.name "gitUsername"
git config --global user.email "gitEmail"
git clone https://github.com/saylesss88/my-flake.git
```

- I've moved away from trying to configure sops pre install, it adds unnecessary
  complexity to a minimal install in my opinion.

- If you click on the layout you want then click the `Raw` button near the top,
  then copy the url and use it in the following command:

```bash
cd /tmp
curl https://raw.githubusercontent.com/nix-community/disko/refs/heads/master/example/luks-btrfs-subvolumes.nix -o /tmp/disk-config.nix
```

- The above curl command is to the `luks-btrfs-subvolumes.nix` layout.

5. Make Necessary changes, I set mine up for impermanence with the following:

```bash
hx /tmp/disk-config.nix
```

```nix
{
  disko.devices = {
    disk = {
      nvme0n1 = {
        type = "disk";
        device = "/dev/nvme0n1";
        content = {
          type = "gpt";
          partitions = {
            ESP = {
              label = "boot";
              name = "ESP";
              size = "512M";
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

## Create a Blank Snapshot of /root

This will give us more options when it comes to impermanence. (i.e. being able
to rollback to a fresh snapshot of root on reboot)

To access all of the subvolumes, we have to mount the Btrfs partitions
top-level.

1. Unlock the LUKS device:

```bash
sudo cryptsetup open /dev/disk/by-partlabel/luks cryptroot
```

2. Mount the Btrfs top-level (subvolid=5):

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

5. Make sure to unmount:

```bash
sudo umount /mnt
```

- For `/tmp` on RAM use something like the following. I've found that having
  disko manage swaps causes unnecessary issues. Also, with btrfs it's easy to
  create a swap after the fact if you choose to.

Using zram follows the ephemeral route:

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

- If you type this out by hand and mess up a single character, you will have to
  start over completely. A fairly safe way to do this is with `vim` or `hx` and
  redirect the hashed pass to a `/tmp/hash`, you can then read it into your
  `users.nix`:

```bash
mkpasswd -m SHA-512 -s > /tmp/pass.txt
# Enter your chosen password
```

And then when inside `users.nix`, move to the line where you want the hashed
password and type `:r /tmp/pass.txt` to read the hash into your current file.

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
  permissions with something like `sudo chown $USER:users ~/flake` and then you
  can work on it without privilege escalation.

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
