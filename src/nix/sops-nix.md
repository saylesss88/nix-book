# Sops-Nix encrypted secrets

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

Copy the Public Keys Value, it will look something like this:

```bash
age-keygen -y ~/.config/sops/age/keys.txt
age12zlz6lvcdk6eqaewfylg35w0syh58sm7gh53q5vvn7hd7c6nngyseftjxl
```

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

Save it and move on.

5. sops-nix's automatic decryption feature using system SSH host keys only works
   with ed25519 host keys for deriving Age decryption keys. Therefore, for
   system decryption, ensure your using ed25519 not rsa keys:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# for multiple keys run something like
ssh-keygen -t ed25519 -f ~/nix-book-deploy-key -C "deploy-key-nix-book-repo"
```

5. Copy the **PRIVATE** key for each and add them to your secrets directory:

While in your flake directory:

```bash
mkdir secrets
sops secrets/github-deploy-key.yaml  # For your github ssh key
```

The above command will open a default sops `github-deploy-key.yaml` in your
`$EDITOR`:

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

Ensure sops can decrypt it:

```bash
sops -d secrets/github-deploy-key.yaml
```

The `-----BEGIN` and the rest of the private key **must** be indented 2 spaces

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

Generate a hashedPassword:

```bash
mkpasswd -m SHA-512 -s
# Enter your chosen password and copy it
```

```bash
sops secrets/password-hash.yaml      # For your `hashedPasswordFile`
```

```yaml
password_hash: PasteGeneratedPasswordHere
```

```bash
sops -d secrets/password-hash.yaml
```

6. Create a `sops.nix` and import it or add this directly to your
   `configuration.nix`:

My `sops.nix` is located at `~/flake/hosts/hostname/sops.nix` and the secrets
directory is located at `~/flake/secrets` so the path from `sops.nix` to
`secrets/pasword-hash.yaml` would be `../../secrets/password-hash.yaml`

```nix
# ~/flake/hosts/magic/sops.nix  # magic is my hostname
# hosts/magic/ is also where my configuration.nix is
{...}: {
  sops = {
    defaultSopsFile = ../../.sops.yaml; # Or the correct path to your .sops.yaml
    age.sshKeyPaths = ["/etc/ssh/ssh_host_ed25519_key"];
    age.keyFile = "/home/jr/sops/age/keys.txt";

    secrets = {
      "password_hash" = {
        sopsFile = ../../secrets/password-hash.yaml; # <-- Points to your password hash file
        owner = "root";
        group = "root";
        mode = "0400";
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

7. Rebuild your configuration and you should see something like this:

```bash
sops-install-secrets: Imported /etc/ssh/ssh_host_ed25519_key as age key with fingerprint age1smamkzrwpdxw63hrxxcq8kmejsm4olknsrg72vd0qtfpmlzlvnf8uws38mzuj
```

If all goes well you should now have a more reproducible setup that doesn't rely
on file paths for your secrets.
