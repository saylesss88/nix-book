---
title: Sops-Nix
date: 2025-12-04
author: saylesss88
collection: "blog"
tags: ["secrets", "sops-nix", "hardening"]
draft: false
---

# Sops-Nix encrypted secrets

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

[SOPS](https://github.com/getsops/sops?ref=blog.gitguardian.com), short for
**S**ecrets**OP**eration**S**, is an editor of encrypted files that supports
quite a few BINARY formats and encrypts with AWS KMS, GCP KMS, Azure Key Vault,
age, and PGP.

Managing secrets—like API keys, SSH deploy keys, and password hashes is a
critical part of system configuration, but it’s also one of the trickiest to do
securely and reproducibly. Traditionally, secrets might be stored in ad hoc
locations, referenced by absolute paths, or managed manually outside of version
control. This approach makes it hard to share, rebuild, or audit your
configuration, and increases the risk of accidental leaks or inconsistencies
between systems.

`sops-nix` solves these problems by integrating Mozilla SOPS directly into your
NixOS configuration. Instead of relying on hardcoded file paths or copying
secrets around, you declare your secrets in your Nix code, encrypt them with
strong keys, and let `sops-nix` handle decryption and placement at activation
time.

Encryption with strong keys, as used by sops-nix, makes brute force attacks
computationally unfeasible with current technology—the time and resources
required to try every possible key would be astronomically high. However, this
protection relies on using strong, secret keys and good security practices;
advances in technology or poor key management can weaken this defense.

> ❗ **CRITICAL SECURITY NOTE:** While the encryption itself is robust, this
> protection fundamentally relies on using **strong, secret keys** and
> **diligent security practices**. If your PGP passphrase is weak, your Age
> private key is easily guessable, or the cleartext secret itself is very short
> and has low entropy (e.g., "12345", "true", "admin"), an attacker might be
> able to compromise your secrets regardless of the encryption.

1. Add sops to your `flake.nix`:

```nix
{
  inputs.sops-nix.url = "github:Mic92/sops-nix";
  inputs.sops-nix.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, sops-nix }: {
    # change `yourhostname` to your actual hostname
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      # customize to your system
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        sops-nix.nixosModules.sops
      ];
    };
  };
}
```

2. Add `sops` and `age` to your `environment.systemPackages`:

```nix
environment.systemPackages = [
    pkgs.sops
    pkgs.age
];
```

3. Generate a key (This is your **private key** and **MUST NEVER BE COMMITTED TO
   GIT OR SHARED**):

```bash
mkdir -p ~/.config/sops/age
age-keygen -o ~/.config/sops/age/keys.txt
```

To get the Public Keys Value, run the following command:

```bash
age-keygen -y ~/.config/sops/age/keys.txt
age12zlz6lvcdk6eqaewfylg35w0syh58sm7gh53q5vvn7hd7c6nngyseftjxl
```

Copy the `age` value it gives you back.

4. Create a `.sops.yaml` in the same directory as your `flake.nix`:

```yaml
# .sops.yaml
keys:
  # Your personal age public key (from age-keygen -y ~/.config/sops/age/keys.txt)
  - &personal_age_key age12zlz6lvcdk6eqaewfylg35w0syh58sm7gh53q5vvn7hd7c6nngyseftjxl

  # You can also use PGP keys if you prefer, but age is often simpler
  # - &personal_pgp_key 0xDEADBEEFCAFE0123

creation_rules:
  # This rule applies to any file named 'secrets.yaml' directly in the 'secrets/' directory
  # or 'secrets/github-deploy-key.yaml' etc.
  - path_regex: "secrets/.*\\.yaml$"
    key_groups:
      - age:
          - *personal_age_key
        # Add host keys for decryption on the target system
        # sops-nix will automatically pick up the system's SSH host keys
        # as decryption keys if enabled in your NixOS config.
        # So you typically don't list them explicitly here unless you
        # want to restrict it to specific fingerprints, which is rare.
        # This part ensures your *personal* key can decrypt it.
```

Save it and move on, this file and `sops.nix` are safe to version control.

5. sops-nix's automatic decryption feature using system SSH host keys only works
   with ed25519 host keys for deriving Age decryption keys. Therefore, for
   system decryption, ensure you're using ed25519 not rsa keys:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# for multiple keys run something like
ssh-keygen -t ed25519 -f ~/nix-book-deploy-key -C "deploy-key-nix-book-repo"
```

6. Copy the **PRIVATE** key for each and add them to your secrets directory:

While in your flake directory:

```bash
mkdir secrets
sops secrets/github-deploy-key.yaml  # For your github ssh key
```

When you call a `sops` command, it will handle the encryption/decryption
transparently and open the cleartext file in an editor.

Editing will happen in the editor that `$SOPS_EDITOR` or `$EDITOR` is set to,
sops will wait for the editor to exit, and then try to reencrypt the file.

The above command will open a default sops `github-deploy-key.yaml` in your
`$EDITOR`:

Erase the default `sops` filler and type `github_deploy_key_ed25519: |`, move
your cursor 1 line down and type `:r ~/.ssh/id_ed25519` to read the private key
into the file and repeat as needed.

```yaml
github_deploy_key_ed25519: |
  -----BEGIN OPENSSH PRIVATE KEY-----
  ...
  -----END OPENSSH PRIVATE KEY-----

github_deploy_key_ed25519_nix-book: |
  -----BEGIN OPENSSH PRIVATE KEY-----
  ...
  -----END OPENSSH PRIVATE KEY-----
```

The `-----BEGIN` and the rest of the private key **must** be indented 2 spaces

Ensure sops can decrypt it:

```bash
sops -d secrets/github-deploy-key.yaml
```

> ❗ WARNING: Only ever enter your private keys through the `sops` command. If
> you forget and paste them in without the `sops` command then run `git add` at
> any point, your git history will have contained an unencrypted secret which is
> a nono. Always use the `sops` command when dealing with files in the `secrets`
> directory, save the file and inspect that it is encrypted on save. If not
> something went wrong with the `sops` process, **do not add it to Git**. If you
> do, you will be required to rewrite your entire history which can be bad if
> you're collaborating with others. `git-filter-repo` is one such solution that
> rewrites your history. Just keep this in mind. This happens because Git has a
> protection that stops you from doing stupid things.

Generate an encrypted password hash with:

```bash
mkpasswd --method=yescrypt > /tmp/password-hash.txt
# Enter your chosen password and copy the encrypted hash it gives you back
```

```bash
sops secrets/password-hash.yaml      # For your `hashedPasswordFile`
```

The above command will open your `$EDITOR` with the file `password-hash.yaml`,
add the following content to it. Replace `PasteEncryptedHashHere` with the
output of the `mkpasswd` command above:

Delete the default `sops` filler, type `password_hash:` and leave your cursor
after the `:` and type `:r /tmp/password-hash.txt`

```yaml
password_hash: PasteEncryptedHashHere
```

Ensure sops can decrypt it:

```bash
sops -d secrets/password-hash.yaml
```

7. Create a `sops.nix` and import it or add this directly to your
   `configuration.nix`:

My `sops.nix` is located at `~/flake/hosts/hostname/sops.nix` and the secrets
directory is located at `~/flake/secrets` so the path from `sops.nix` to
`secrets/pasword-hash.yaml` would be `../../secrets/password-hash.yaml`

Another step you can take is to copy your key to a persistent location,
preparing for impermanence:

```bash
sudo mkdir /persist/sops/age
sudo cp ~/.config/sops/age/keys.txt /persist/sops/age/keys.txt
```

Then you would change the `age.keyFile = "/persist/sops/age/keys.txt"` to match
this location below.

```nix
# ~/flake/hosts/magic/sops.nix  # magic is my hostname
# hosts/magic/ is also where my configuration.nix is
{...}: {
  sops = {
    defaultSopsFile = ../../.sops.yaml; # Or the correct path to your .sops.yaml
    age.sshKeyPaths = [];
    age.keyFile = "/home/jr/sops/age/keys.txt";

    secrets = {
      "password_hash" = {
        sopsFile = ../../secrets/password-hash.yaml; # <-- Points to your password hash file
        owner = "root";
        group = "root";
        mode = "0400";
        neededForUsers = true;
      };
      "github_deploy_key_ed25519_nix-book" = {
        sopsFile = ../../secrets/github-deploy-key.yaml;
        key = "github_deploy_key_ed25519_nix-book";
        owner = "root";
        group = "root";
        mode = "0400";
      };
      "github_deploy_key_ed25519" = {
        sopsFile = ../../secrets/github-deploy-key.yaml;
        key = "github_deploy_key_ed25519";
        owner = "root";
        group = "root";
        mode = "0400";
      };
    };
  };
}
```

Import `sops.nix` into your `configuration.nix` or equivalent:

```nix
# configuration.nix
imports = [
  ./sops.nix # Assuming sops.nix is in the same directory as configuration.nix, adjust path as needed
  # ... other imports
];
```

> ❗ NOTE: You may see in the sops quickstart guide that if you're using
> impermanence, the key used for secret decryption (`sops.age.keyFile`) must be
> in a persistent directory, loaded early enough during the boot process. If you
> are using the btrfs subvolume layout you don't need to worry about this
> because your home will be on its own partition when only the root partition is
> wiped on reboot. Adding `neededForUsers = true;` tells `sops-nix` to decrypt
> and make that secret available earlier in the boot process--specifically,
> before user and group accounts are created.

You typically use `age.sshKeyPaths` for **system-level secrets** with a
persistent SSH host key

For **user-level secrets**, use `age.keyFile` pointing to your Age private key,
stored in a safe persistent location.

For reproducibility, keep your key files in a persistent, predictable path and
document which keys are used for which secrets in your `.sops.yaml`.

If you don't need both `age.keyFile` and `age.sshKeyPaths` it can reduce
complexity to use one or the other. Although most people may choose one, it's
not bad to use both it just adds complexity.

And finally use the password-hash for your `hashedPasswordFile` for your user,
my user is `jr` so I added this:

```nix
# ... snip ...
    users.users = {
      # ${username} = {
      jr = {
        homeMode = "755";
        isNormalUser = true;
        # description = userVars.gitUsername;
        hashedPasswordFile = config.sops.secrets.password_hash.path;
  # ...snip...
```

8. Rebuild your configuration and you should see something like this:

```bash
sops-install-secrets: Imported /etc/ssh/ssh_host_ed25519_key as age key with fingerprint age1smamdkzrwpdxw63hrxxcq8kmejsm4olknsrg72vd0qtfpmlzlvnf8uws38mzuj
```

By integrating SOPS with NixOS through `sops-nix`, you gain a modern, secure,
and reproducible way to manage sensitive secrets. Unlike traditional
approaches—where secrets are often scattered in ad hoc locations, referenced by
absolute paths, or managed outside version control—sops-nix keeps your secrets
encrypted, declarative, and version-control friendly.
