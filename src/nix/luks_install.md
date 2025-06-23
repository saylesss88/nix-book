# Minimal LUKS Encrypted Install with Btrfs Subvolumes

Follow this guide to set up a minimal NixOS install with LUKS, Btrfs, and
encrypted secrets.

1. Prepare the Minimal ISO and Networking

- Download the NixOS minimal ISO and boot it.

- Set up networking:

```bash
sudo systemctl start wpa_supplicant wpa_cli
wpa_cli
> add_network
0
> set_network 0 ssid "myhomenetwork"
OK
> set_network 0 psk "mypassword"
OK
> enable_network 0
OK
> quit
```

Or, use:

```bash
sudo wpa_passphrase "myhomenetwork" "mypassword" >>
/etc/wpa_supplicant/wpa_supplicant-wlan0.conf
sudo systemctl restart wpa_supplicant@wlan0.service
```

Test your connection:

```bash
ping google.com
```

2. Enable Experimental Nix Features

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
```

3. Install Essential Tools

````bash
nix-shell -p git helix yazi age sops mkpasswd
export EDITOR='hx'
git config --global user.name "YourUsername"
git config --global user.email "YourGitEmail"
```

4. Clone the Configuration Repository

```bash
git clone https://github.com/saylesss88/my-flake.git
````

5. Customize Configuration Files

- Modify `flake.nix`, `users.nix`, and `configuration.nix` to set your desired
  `hostname` and `username`.

  Check your disk device with `lsblk` and update line 5 of `disk-config2.nix`
  accordingly. (i.e. the `device` needs to match your device)

6. Generate Hashed Passwords and LUKS Passphrase

   Generate a hashed password:

```bash
mkpasswd -m SHA-512 -s > /tmp/user_hashed_pass.txt
```

7. Set Up Age Key for SOPS

```bash
mkdir -p ~/.config/sops/age age-keygen -o ~/.config/sops/age/keys.txt
```

- Copy the public key and add it to .sops.yaml in your repo:

```yaml
keys:

- &personal_age_key
  age1yuvx83vtxr8rf6t8vmwjeymt9u83h4cwktnqvmn49rhy36chj3tqlgunhz creation_rules:
- path_regex: "secrets/.\*\\.yaml$" key_groups:

  - age:
    - \*personal_age_key
```

- Add `.sops.yaml` to git:

```bash
git add .sops.yaml
```

- If using Git and you forget to add the `.sops.yaml`, sops won't be able to
  decrypt your secrets.

8. Generate SSH Key

```bash
ssh-keygen -t ed25519 -C "your_email@example.com" > /tmp/ssh_key.txt
```

- Copy the private key into `secrets/github-deploy-key.yaml` using SOPS:

```bash
sops secrets/github-deploy-key.yaml
```

Inside, paste:

```yaml
github_deploy_key_ed25519: |
  -----BEGIN OPENSSH PRIVATE KEY----- ...
  -----END OPENSSH PRIVATE KEY-----
```

- Indent the key content by 2 spaces.

9. Add Hashed Password to SOPS

```bash
sops secrets/password-hash.yaml
```

Inside, paste:

```yaml
password_hash:
```

- Read in your hashed password:

```yaml
:r /tmp/user_hashed_pass.txt
```

10. Configure SOPS in Nix

Ensure your sops.nix looks like this (adjust paths as needed):

```nix
{...}: {
sops = { defaultSopsFile = ./sops.yaml;
    age.sshKeyPaths = ["/etc/ssh/ssh_host_ed25519_key"];
    age.keyFile = "/home/jr/sops/age/keys.txt";

    secrets = {
      "password_hash" = {
        sopsFile = ./secrets/password-hash.yaml;
        owner = "root";
        group = "root";
        mode = "0400";
        neededForUsers = true;
      };
      "github_deploy_key_ed25519" = {
        sopsFile = ./secrets/github-deploy-key.yaml;
        key = "github_deploy_key_ed25519";
        owner = "root";
        group = "root";
        mode = "0400";
      };
    };

}; }
```

11. Apply Configuration

First generate your `hardware-configuration.nix` and replace the repos existing
`hardware-configuration.nix` with the freshly generated one:

```bash
nixos-generate-config --no-filesystems --root /mnt
sudo mv /tmp/etc/nixos/hardware-configuration.nix ~/my-flake
```

Now you can rebuild to apply the changes:

```bash
sudo nixos-rebuild switch --flake ~/my-flake#your-hostname
```

12. Prepare Disko for Partitioning and Formatting

- Run Disko to wipe, partition, and format your disk (WARNING: this destroys
  data):

```bash
sudo nix --experimental-features "nix-command flakes" run github:nix-community/disko/latest -- --mode destroy,format,mount ~/my-flake/disk-config2.nix
```

- You will be prompted for your LUKS passphrase.

Check mounts:

```bash
mount | grep /mnt
```

13. Update and Apply Configuration

- Update `configuration.nix` with your hostname, packages, etc.

- Replace your flake's `hardware-configuration.nix` with the newly generated
  one:

```bash
rm ~/my-flake/hardware-configuration.nix
sudo mv /mnt/etc/nixos/hardware-configuration.nix ~/my-flake
```

14. Move Flake and Install NixOS

```bash
sudo mv ~/my-flake /mnt/etc/nixos/
sudo nixos-install --flake /mnt/etc/nixos/my-flake#hostname
```

- You will be prompted to set a new password and reboot.

15. Post-Installation

- After reboot, you will be asked to enter your encryption password for the boot
  process to continue. Enter the passphrase you used for the `disko` command.

```bash
sudo mv /etc/nixos/my-flake ~
sudo chown -R $USER:users ~/my-flake
```

- It is possible to use sops to auto decrypt your luks partition but it negates
  many of the benefits in my opinion. It's something you could look into if you
  wanted, as it still does provide some benefits.

- I'm working on the impermanence guide for this setup, had quite a few
  headaches so far. I'm open to suggestions.
