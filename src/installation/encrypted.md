# Encrypted Setups

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

# Minimal LUKS Encrypted Install with Btrfs Subvolumes

Follow this guide to set up a minimal NixOS install with LUKS, Btrfs, and
encrypted secrets. This guide aims for a manually entered LUKS passphrase at
boot and uses SOPS for other system/user secrets.

1. Prepare the Minimal ISO and Networking

```bash
sudo systemctl start wpa_supplicant.service # Ensure wpa_supplicant is running
sudo wpa_cli
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

Test your connection: Bash

```bash
ping google.com
```

2. Enable Experimental Nix Features Bash

```bash
export NIX_CONFIG='experimental-features = nix-command flakes'
```

3. Install Initial Essential Tools & Clone Repo

These tools are needed for the initial configuration and running disko. Bash

```bash
sudo nix-env -iA nixos.{git,helix,yazi} # Install globally for the installer environment
export EDITOR='hx' #  Set your preferred editor for this session
git config --global user.name "YourUsername"
git config --global user.email "YourGitEmail"
```

- You may be put off by the `nix-env` commands, the live installer runs from RAM
  and a read-only filesystem. Any packages you install with `nix-env` are only
  available while you're in the live environment. When you reboot, the system is
  discarded, and everything reverts to the original state.

- The only persistent changes are those you deliberately make to the mounted
  filesystem (e.g., `/mnt` during installation), which is the target for your
  new NixOS installation.

# Clone the Starter Repo

```bash
git clone https://github.com/saylesss88/my-flake.git
```

4. Customize Configuration Files (Pre-Disko)

- Navigate into your cloned flake:

```bash
cd ~/my-flake
```

- Modify `flake.nix`, `users.nix`, and `configuration.nix` to set your desired
  hostname and username.

- Check your disk device with `lsblk` and update `disk-config2.nix` accordingly
  (i.e., the device needs to match your actual disk device, e.g., `/dev/nvme0n1`
  or `/dev/sda`).

5. Apply Initial Configuration & Prepare Disko

This step builds a temporary NixOS system in the installer's RAM disk that
includes your disko configuration.

- Generate your `hardware-configuration.nix` based on the installer's view of
  your hardware.

```bash
sudo nixos-generate-config --no-filesystems --root /mnt
sudo mv /mnt/etc/nixos/hardware-configuration.nix ~/my-flake/hardware-configuration.nix
```

Run Disko to wipe, partition, and format your disk (WARNING: This destroys ALL
data on the target disk, disko doesn't work with dual boot!)

```bash
sudo nix --experimental-features "nix-command flakes" run github:nix-community/disko/latest -- --mode destroy,format,mount ~/my-flake/disk-config2.nix
```

- Crucial: You will be prompted for your LUKS passphrase. This is the password
  you will use to unlock your disk every time you boot. Choose a strong,
  memorable passphrase.

Verify your partitions are mounted to `/mnt`:

```bash
mount | grep /mnt
```

`/mnt` is a temporary mount point used during installation to access and
configure the target filesystem. Disko uses `/mnt` to mount the filesystems it
creates, but these mounts are only for the installation process. After
installation and reboot, `/mnt` is no longer used unless you manually mount
something there.

After installation and reboot:

- All files and directories under `/mnt` (for example, `/mnt/etc/nixos/`) are
  now accessible at their normal system locations.

- So, `/mnt/etc/nixos/` becomes `/etc/nixos/` on your new NixOS system.

- `/mnt` itself is no longer used as a mount point unless you manually mount
  something there again.

6. Install SOPS-Related Tools & Generate Secrets (Post-Disko)

Now that your disk is formatted and mounted to `/mnt`, we can generate and
encrypt your secrets directly onto the target filesystem. This ensures the Age
key is located where the installed system can always find it.

Change into the mounted flake directory:

```bash
sudo mv ~/my-flake /mnt/etc/nixos/
cd /mnt/etc/nixos/my-flake
```

Install SOPS-related tools:

```bash
sudo nix-env -iA nixos.{age,sops,mkpasswd} # Install these into the installer environment
```

6.1. Set Up Age Key for SOPS

This generates your Age private key directly on the target root partition.

Create the directory for your Age key and generate the key pair:

```bash
sudo mkdir -p /mnt/root/.config/sops/age
sudo age-keygen -o /mnt/root/.config/sops/age/keys.txt
```

The above location will persist after the install and reboot.

Display your Public Key and Manually Transcribe:

```bash
sudo age-keygen -y /mnt/root/.config/sops/age/keys.txt
```

Carefully read and manually type the `age1...` public key string into your
`.sops.yaml` file within your flake directory
(`/mnt/etc/nixos/my-flake/.sops.yaml`).

Example `.sops.yaml` (after editing):

```yaml
keys:
  - &personal_age_key age1yuvx83vtxr8rf6t8vmwjeymt9u83h4cwktnqvmn49rhy36chj3tqlgunhz # <-- Your public key goes here
creation_rules:
  - path_regex: "secrets/.*\\.yaml$"
    key_groups:
      - age:
          - *personal_age_key
```

Add `.sops.yaml` to Git

```bash
git add .sops.yaml
```

- Note: If you forget to add the `.sops.yaml` to Git, sops won't be able to
  decrypt your secrets after reboot.

  6.2. Generate SSH Key & Add to SOPS

- Generate your SSH key and save the private key to a temporary file:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com" -f /tmp/ssh_key
```

- Encrypt the private key into `secrets/github-deploy-key.yaml` using SOPS:

```bash
sops secrets/github-deploy-key.yaml
```

Inside the editor (e.g., `helix`, `vim`, `nano`):

1. Add the YAML key: `github_deploy_key_ed25519: |`

2. Move your cursor to the line below, indented by 2 spaces.

3. To read the content from `/tmp/ssh_key`:

- For vim: In Normal mode, type `:r /tmp/ssh_key` and press Enter.

- For nano: Press `Ctrl+R`, type `/tmp/ssh_key`, and press Enter.

4. Save and exit the editor.

- Securely delete the temporary file:

```bash
shred -u /tmp/ssh_key
```

6.3. Add Hashed Password to SOPS

- Generate your hashed password for the user and save it to a temporary file:

```bash
mkpasswd -m SHA-512 -s > /tmp/user_hashed_pass.txt
```

- Encrypt the hashed password into `secrets/password-hash.yaml` using SOPS:

```bash
sops secrets/password-hash.yaml
```

Inside the editor:

1. Add the YAML key: password_hash:

2. Move your cursor to the end of that line (or the line below, if your editor
   prefers).

3. To read the content from `/tmp/user_hashed_pass.txt`:

- For vim: In Normal mode, type `:r /tmp/user_hashed_pass.txt` and press Enter.
  (You may need to join the lines using Shift+J and add 2 spaces of indentation
  if it pastes on a new line).

- For nano: Press `Ctrl+R`, type `/tmp/user_hashed_pass.txt`, and press Enter.
  Save and exit the editor.

Securely delete the temporary file:

```bash
shred -u /tmp/user_hashed_pass.txt
```

7. Configure SOPS in Nix (sops.nix)

Ensure your `sops.nix` looks like this, adjusting `age.keyFile` to the new path
(which is on your root filesystem).

```nix
{ config, ... }: { sops = { defaultSopsFile = ./sops.yaml; # If your host's SSH key is Ed25519 and you want SOPS to use it for decryption, # you can add:
age.sshKeyPaths = ["/etc/ssh/ssh_host_ed25519_key"];
age.keyFile = "/root/.config/sops/age/keys.txt"; # Crucial: This path is now on your installed system's root

    secrets = {
      "password_hash" = {
        sopsFile = ./secrets/password-hash.yaml;
        owner = "root";
        group = "root";
        mode = "0400";
        neededForUsers = true; # Ensures it's available for user creation/login
      };
      "github_deploy_key_ed25519" = {
        sopsFile = ./secrets/github-deploy-key.yaml;
        key = "github_deploy_key_ed25519"; # Specifies the key within the YAML file
        owner = "root";
        group = "root";
        mode = "0400";
      };
    };

}; }
```

8. Final Configuration & Apply

- Update `configuration.nix`: Review and update your `configuration.nix` with
  your hostname, desired packages, services, etc.

Final rebuild to apply all changes:

```bash
sudo nixos-rebuild switch --flake .#your-hostname # Use . since you are in the flake directory
```

9. Install NixOS

Execute the installation command:

```bash
sudo nixos-install --flake /mnt/etc/nixos/my-flake#your-hostname
```

- You will be prompted to set a new password. This is for the root user on your
  newly installed system.

10. Post-Installation

First Boot: After reboot, your system will pause to ask for your encryption
password (the LUKS passphrase you set during the Disko command in Step 5). Enter
it to proceed with booting. Move your flake to your user's home directory (if
desired) and fix permissions:

```bash
sudo mv /etc/nixos/my-flake ~ sudo chown -R $USER:users ~/my-flake
```

Optional: It is possible to use sops to auto-decrypt your LUKS partition, but
this guide focuses on manual entry for enhanced security awareness at boot. If
you're interested, you could explore that option, but it negates some of the
benefits of manually entering your passphrase. I'm currently working on an
impermanence guide for this setup, which has presented some challenges. I'm open
to suggestions!
