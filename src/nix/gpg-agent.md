---
title: GnuPG gpg-agent
date: 2026-01-15
author: saylesss88
collection: "blog"
tags: ["Encryption", "GnuPG", "security"]
draft: false
---

# GnuPG & `gpg-agent` on NixOS

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

> ⚠️ **SECURITY WARNING**: This guide involves sensitive cryptographic material.
> **Never share your private key or passphrase**. Backup your keys and handle
> them with extreme care.

![GnuPG](../images/gnupg.png)

## ⚠️ gpg.fail (practical OpenPGP vulnerabilities) (Added on 2026-01-15)

The [gpg.fail](https://gpg.fail) is a write-up of real-world weaknesses in
GPG/OpenPGP implementations and edge cases in the OpenPGP ecosystem—not the
underlying math of modern cryptography. ​ The core idea is that signature
verification needs two things: correct cryptography and confidence that the data
you think was verified is actually the same data the verifier processed, which
can break down when formats are complex and tooling is permissive or ambiguous.

### Why this matters even with good key hygiene

Most of the hardening in this guide (offline primary key, subkeys, strict
permissions, agent separation) is still worth doing because it protects your
private key material and reduces the blast radius if a workstation is
compromised.

But that kind of key hygiene doesn’t automatically protect you from OpenPGP
“sharp edges” like ambiguous parsing rules, weird message constructions, or
implementation bugs because those problems happen at the message/format/tooling
layer, not the key-storage layer.

### How to avoid the sharp edges (actionable)

- Don’t treat “Good signature” as the end of the story. Treat verification as
  “verify + inspect what was verified,” especially if the output will be
  consumed by other tools or humans who may misinterpret it. ​

- Prefer OpenPGP for narrow, well-understood tasks (verifying release artifacts,
  encrypting files) and be extra cautious when dealing with untrusted,
  attacker-controlled OpenPGP inputs that flow through multiple tools (mail
  clients, import pipelines, automation). ​

- Keep GnuPG and related tooling updated; gpg.fail covers vulnerabilities that
  include classic implementation issues and not just “protocol design” pitfalls.

### If you automate verification (important)

If you’re writing automation around signature verification, ensure your pipeline
clearly distinguishes:

1. “signature cryptographically valid” from

2. “the extracted/displayed message is exactly what was verified”. Some gpg.fail
   items specifically call out cases where output does not make this distinction
   obvious enough to prevent misuse.

### NixOS/Home-Manager hardening can break GPG (common failure mode)

Some “hardening” choices can cause GPG to fail in ways that look mysterious but
are just UI/agent integration problems (for example, **No pinentry** / “gpg
failed to sign the data” when the agent can’t prompt). If GPG signing/encryption
suddenly stops working, first confirm pinentry is configured correctly (via your
`services.gpg-agent.pinentryPackage` or `gpg-agent.conf`) and restart the agent
with `gpgconf --kill gpg-agent` then `gpgconf --launch gpg-agent`.

## 🔑 Key Concepts

**GnuPG** is a complete and free implementation of the OpenPGP standard. It
allows you to encrypt and sign your data and communications, has a versatile key
management system, and access modules for many kinds of public key directories.
GnuPG (GPG), is a command line tool for secure communication.

**PGP (Pretty Good Privacy)** and **GPG (GNU Privacy Guard)**. While distinct,
they are deeply interconnected and, for the rest of this section, I'll use the
terms interchangeably.

**PGP** was the original, groundbreaking software that brought robust public-key
cryptography to the masses. It set the standard for secure email communication.
However, PGP later became a commercial product.

To provide a free and open-source alternative that anyone could use and inspect,
**GPG** was created. Crucially, **GPG** is a complete implementation of the
OpenPGP standard. This open standard acts as a universal language for encryption
and digital signatures.

GnuPG uses a more complex scheme in which a user has a primary keypair and then
zero or more additional subordinate keypairs.

Signing public keys with the corresponding private key is called _self-signing_,
and a public key that has self-signed user IDs bound to it is called a
_certificate_.

**Web of Trust**: Rather than validate every single key individually, you can
rely on other factors such as if it has been signed by a key that you fully
trust or if it has been signed by three marginally trusted keys to validate
keys.

`gpg-agent` is a daemon to manage secret (private) keys independently from any
protocol. It is used as a backed for `gpg` and `gpgsm` as well as for a couple
of other utilities. --[man gpg-agent](https://man.cx/gpg-agent)

There are numerous front-ends for gpg as well, i.e., GUI apps that simplify many
of the commands and processes. Two that I touch on in this overview are
`seahorse` and `kleopatra`.

### Asymmetric Encryption (Public-Key cryptography)

E2ee requires that every sender and recipient does a one time preparation, which
involves the generation of personal random numbers. Two such random numbers are
necessary, one will be called your secret key and another one will be called
your public key (together your _personal key_). These numbers are very big, they
consist of hundreds or thousands of digits.

A message can be encrypted using the recipients public key and can only be
decrypted with the matching private key. In other words, if you exchange
**public keys** with someone you both can encrypt messages that only the other
can decrypt with their own **private key**. **You must never share the private
key or the private keys passphrase with anyone else**.

**What’s safe to share?**

- Your public key (used to encrypt files and verify signatures)

- Your key ID (identifies your key, useful for sharing public keys or configs)

- Your keys fingerprint `gpg --fingerprint`

**What must never be shared?**

- Your private (secret) key, usually in your `~/.gnupg/private-keys-v1.d/`
  directory. Usually called your _private keyring_. **Your main goal should be
  the protection of your private key**.

- **Your passphrase for your private key**. Even if someone is able to somehow
  get your private key, they need to break the passphrase to access it
  unencrypted. **Protect this passphrase**!

**Best Practices**

Don't rely on the short KeyID, at least use long OpenPGP Key IDs (for example
0xA1E6148633874A3D), they are 64 bits long and harder to spoof. Even better, use
the fingerprint.This is accomplished in the configuration with
`keyid-format = "0xlong";`, and `with-fingerprint`.

Always sign your public keys before you publish them to prevent man in the
middle attacks and other modifications. When a subkey or userID is generated it
is self-signed automatically, which is why you need to enter your password.

Don't blindly trust keys from keyservers. You should verify the full key
fingerprint with the owner over the phone if possible.

- [Verifying software signatures](https://www.kicksecure.com/wiki/Verifying_Software_Signatures)

Use a strong primary key, don't use 1024-bit DSA, 1024-bit RSA, or SHA-1 for
signing they are no longer recommended.

Choose an expiration date less than 2 years, you can add time if needed.
Remember this date.

Rotate your subkeys.

Keep your primary key offline, this ensures that it can't be stolen by an
attacker allowing him to create new identities. We accomplish this by creating
subkeys and only adding the subkeys keygrip and the subkeys `default-key` to our
configuration keeping the primary key out of it.

Since we will be removing our primary key, even we won't be able to create
additional keys so it's important to think ahead and make all the keys you'll
need. However, it is as easy as reimporting it to give yourself access again.

Many of these best practices come from the following guide:

- [RiseUp gpg-best-practices](https://riseup.net/ru/security/message-security/openpgp/gpg-best-practices)

---

Home Manager module with `gpg-agent`, `gnupg`, and `pinentry-gnome3`:

```nix
# gpg-agent.nix
{
  config,
  lib,
  pkgs,
  ...
}: {
  options = {
    custom.pgp = {
      enable = lib.mkEnableOption {
        description = "Enable PGP Gnupgp";
        default = false;
      };
    };
  };

  config = lib.mkIf config.custom.pgp.enable {
    services = {
      ## Enable gpg-agent with ssh support
      gpg-agent = {
        enable = true;
        enableSshSupport = true;
        enableZshIntegration = true;
        # pinentry is a collection of simple PIN or passphrase dialogs used for
        # password entry
        pinentryPackage = pkgs.pinentry-qt;
      };

      ## We will put our keygrip here
      gpg-agent.sshKeys = [];
    };
    home.packages = [pkgs.gnupg];
    programs = {
      gpg = {
        ## Enable GnuPG
        enable = true;

        # homedir = "/home/userName/.config/gnupg";
        settings = {
          # Default/trusted key ID (helpful with throw-keyids)
          # Example, you will put your own keyid here
          # Use `gpg --list-keys`
          # default-key = "0x37ACBCDA569C5C44788";
          # trusted-key = "0x37ACBCDA569C5C44788";
          # https://github.com/drduh/config/blob/master/gpg.conf
          # https://www.gnupg.org/documentation/manuals/gnupg/GPG-Configuration-Options.html
          # https://www.gnupg.org/documentation/manuals/gnupg/GPG-Esoteric-Options.html
          # Some Best Practices, stronger algos etc
          # Use AES256, 192, or 128 as cipher
          personal-cipher-preferences = "AES256 AES192 AES";
          # Use SHA512, 384, or 256 as digest
          personal-digest-preferences = "SHA512 SHA384 SHA256";
          # Use ZLIB, BZIP2, ZIP, or no compression
          personal-compress-preferences = "ZLIB BZIP2 ZIP Uncompressed";
          # Default preferences for new keys
          default-preference-list = "SHA512 SHA384 SHA256 AES256 AES192 AES ZLIB BZIP2 ZIP Uncompressed";
          # SHA512 as digest to sign keys
          cert-digest-algo = "SHA512";
          # SHA512 as digest for symmetric ops
          s2k-digest-algo = "SHA512";
          # AES256 as cipher for symmetric ops
          s2k-cipher-algo = "AES256";
          # UTF-8 support for compatibility
          charset = "utf-8";
          # Show Unix timestamps
          fixed-list-mode = "";
          # No comments in signature
          no-comments = "";
          # No version in signature
          no-emit-version = "";
          # Disable banner
          no-greeting = "";
          # Long hexidecimal key format
          keyid-format = "0xlong";
          # Display UID validity
          list-options = "show-uid-validity";
          verify-options = "show-uid-validity";
          # Display all keys and their fingerprints
          with-fingerprint = "";
          # Cross-certify subkeys are present and valid
          require-cross-certification = "";
          # Disable caching of passphrase for symmetrical ops
          no-symkey-cache = "";
          # Enable smartcard
          # use-agent = "";
        };
      };
    };
  };
}
```

> 🤔 Fun Fact: Elliot Alderson mentions encrypting Evil Corps files with 256 bit
> AES encryption ensuring that it's impossible to break in `eps1.9_zer0.daY.avi`
> of Mr. Robot.

- The default path is `~/.gnupg`, if you prefer placing it in the `~/.config`
  directory or elsewhere, uncomment the `homedir` line and change `userName` to
  your username.

- I use sway so `pinentry-qt` works for me, there is also the following options
  for this attribute:

- `pinentry-tty`

- `pinentry-gnome3`

- `pinentry-gtk2`

And more, research what you need and use the correct one.

[search.nixos.org pinentry](https://search.nixos.org/packages?channel=unstable&query=pinentry)

Enable in your `home.nix` or equivalent:

```nix
# home.nix
# ... snip ...
imports = [
    ./gpg-agent.nix
];
custom.pgp.enable = true;
# ... snip ...
```

`gpg --full-generate-key` can be used to generate a basic keypair.

`gpg --expert --full-generate-key` can be used for keys that require more
capabilities.

> ❗ NOTE: We will first generate our GPG primary key that is required to
> atleast have sign capabilities, we will then derive subkeys from said primary
> key and use them for signing and encrypting. It is recommended to generate a
> revoke certificate right after creating your primary key.

To generate your gpg primary key you can do the following:

```bash
gpg --full-generate-key
```

- Choose `(10) (sign only)`

- Give it a name and description

- Give it an expiration date, 1y is common

- Use a strong passphrase or password

- Give it a comment, I typically add the date

If you see a warning about incorrect permissions, you can run the following:

```bash
chmod 700 ~/.gnupg
chmod 600 ~/.gnupg/*
```

Verify:

```bash
ls -ld ~/.gnupg
# Should show: drwx------

ls -l ~/.gnupg
# Files should show: -rw-------
```

### Generate a Revocation Certificate

`mykey` must be a key specifier, either the keyID of the primary keypair or any
part of the user ID that identifies the keypair:

Replace `mykeyID` with the keyID of your primary key and store the cert in a
safe place:

```bash
gpg --output revoke.asc --gen-revoke mykeyID
Create a revocation certificate for this key? (y/N)
Please select the reason for the revocation:
  0 = No reason specified
  1 = Key has been compromised
  2 = Key is superseded
  3 = Key is no longer used
  Q = Cancel
(Probably you want to select 1 here)
Your decision?
```

The certificate will be output to a file `revoke.asc`. If the `--output` is
ommitted, the result will be placed on stdout.

Since it's a short certificate, you can print a hardcopy and store it somewhere
safe. The cert shouldn't be somewhere that others can access it since anyone
could publish the revoke cert and render the corresponding public key useless.

To apply the revoke cert, import it:

```bash
gpg --import revoke.asc
# And optionally push the revoked key to public keyservers to notify others:
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEYID
```

---

After fixing, run `gpg --list-keys --with-fingerprint`, which lists your public
keys:

```bash
# Take note of your public key
gpg --list-keys --with-fingerprint
/home/jr/.gnupg/pubring.kbx
---------------------------
pub   ed25519/0x095782A1B124AF15 2025-08-23 [SCA] [expires: 2026-08-23]
Key fingerprint = 5908 9C5B FEC8 0D75 FCB0  E206 0958 82C1 A124 CF15
uid                   [ultimate] Jr (08-23-25) <sayls8@proton.me>
```

- Copy the KeyID, in this example it would be `0x095722B2A123CF15`. We will use
  it for the command below.

The warning should be gone.

Now we will generate 2 subkeys, 1 for encryption and 1 for authentication.

```bash
gpg --expert --edit-key 0x095722B2A123CF15
```

Choose 11 (set your own capabilities) and add A (Authenticate) and type `save`
to save and exit. Repeat this again and choose ECC (encrypt only).

> ❗ `gpg --edit-key` has many more capabilities, after launching type `help`.

**Add Keygrip of Authenticate Subkey to `sshcontrol` for gpg-agent**

```bash
gpg --list-secret-keys --with-keygrip --keyid-format LONG
```

Copy the keygrip of the subkey with Authenticate capabilities

Add the keygrip number to your `gpg-agent.sshKeys` and rebuild, this adds an SSH
key to `gpg-agent`. This is for the SSH key functionality of `gpg-agent`, while
the key ID (`default-key`) is for GPG-specific operations like signing commits:

```nix
# gpg-agent.nix
gpg-agent.sshKeys = ["6BD11826F3845BC222127FE3D22C92C91BB3FB32"];
```

- By itself, a keygrip cannot be used to reconstruct your private key. It's
  derived from the public key material, not from the secret key itself so it's
  safe to version control. Don't put your keygrip in a public repo if you don't
  want people to know you use that key for signing/authentication. It's not a
  security risk, but it leaks a tiny bit of metadata.

The following article mentions the keygrip being computed from public elements
of the key:

- [gnupg-users what-is-a-keygrip](https://gnupg-users.gnupg.narkive.com/q5JtahdV/gpg-agent-what-is-a-keygrip)

Add the KeyId to your `gpg-agent.nix`, this declares your default-key to persist
through rebuilds:

Copy the public key of the same subkey with Authenticate capabilities you will
see something like `[SA]` next to it for Sign and Authenticate:

```nix
# gpg-agent.nix
gpg.settings = {
    # Replace with your own Subkeys KeyID `gpg --list-keys --keyid-format LONG`
    default-key = "Ox37ACA569C5C44787";
    trusted-key = "Ox37ACA569C5C44787";
};
```

This key should be signed automatically, ensure that it is:

```bash
gpg --sign-key Ox37ACA569C5C44787
```

Rebuild, and check that everything is correct with:

```bash
ssh-add -L
# you should see something like:
ssh-ed25519 AABCC3NzaC1lZDI1NTE5ABBAIHyujgyCjjBTqIuFM3EMUSo6RGklmOXQW3uWRhWdJ1Mm (none)
```

- Never version-control your private key files or `.gnupg` contents.

Add the following to your shell config:

```bash
# zsh.nix
# ... snip ...
initContent = ''
    export GPG_TTY=$(tty)
    export SSH_AUTH_SOCK=$(gpgconf --list-dirs agent-ssh-socket)
    gpgconf --launch gpg-agent
'';
# ... snip ...
```

Rebuild and then restart `gpg-agent` if necessary:

```bash
gpgconf --kill gpg-agent
gpgconf --launch gpg-agent
```

Test, these should match:

```bash
echo "$SSH_AUTH_SOCK"
# output
/run/user/1000/gnupg/d.wft5hcsny4qqq3g31c76534j/S.gpg-agent.ssh

gpgconf --list-dirs agent-ssh-socket
# output
/run/user/1000/gnupg/d.wft5hcsny4qqq3g31c76834j/S.gpg-agent.ssh
```

```bash
ssh-add -L
# Copy the entire following line:
ssh-ed25519 AABBC3NzaC1lZDI1NTE5AAAAIGXwhVokJ6cKgodYT+0+0ZrU0sBqMPPRDPJqFxqRtM+I (none)
```

- Mine shows `(none)` because I left the comment field blank when creating the
  key and doesn't affect functionality.

Then, in your server's NixOS configuration (e.g., `configuration.nix`): Change
`yourUser` to your username. This is how you grant access to a remote machine,
and the public key from the GPG subkey is what's added here, the output of
`ssh-add -L`:

```nix
users.users.yourUser = {
openssh = {
  authorizedKeys.keys = [
    # Replace with the output of `ssh-add -L`
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGXwhVokJ6cKgodYT+0+0ZrU0sBqMPPRDPJqFxqRtM+I (none)"
  ];
};
};
```

> ❗ NOTE: Only the **public** key goes here, it's safe to commit to version
> control. If you prefer not to hardcode it in the config, you can reference it
> from a `.pub` file in your repo and read it with
> `builtins.readFile ./mykey.pub`

Rebuild your system and test an SSH connection into the server:

```bash
ssh -p <your-port> user@hostname
```

- `<your-port>` is often `22` so it would be something like:

```bash
ssh -p 22 bill@xps
```

Once you successfully sign in to SSH, it will ask you if you're sure you trust
the remote server's SSH host key. Once you type `yes`, it will automatically be
used for tasks such as file decryption.

### Remove and Store your Primary Key offline

> ❗ NOTE: After you remove your primary key, you will no longer be able to
> derive subkeys from it or sign keys unless you re-import it.

```bash
# extract the primary key
gpg -a --export-secret-key sayls8@proton.me > secret_key
# extract the subkeys, which we will reimport later
gpg -a --export-secret-subkeys sayls8@proton.me > secret_subkeys.gpg
# delete the secret keys from the keyring, so only subkeys are left
gpg --delete-secret-keys sayls8@proton.me
Delete this key from the keyring? (y/N) y
This is a secret key! - really delete? (y/N) y
# reimport the subkeys
gpg --import secret_subkeys.gpg
# verify everything is in order
gpg --list-secret-keys
# remove the subkeys from disk
rm secret_subkeys.gpg
```

I recommend also keeping a `.gpg` version to make it easy to re-import your
primary key: `gpg --export-secret-keys --armor --output private-key-bak.gpg`

Then store `secret_key` on an encrypted USB drive or somewhere offline. If you
want to protect it for now, you can just use the encryption subkey that we
created to encrypt `secret_key` with a passphrase:

```bash
gpg --list-keys --keyid-format LONG
```

Copy the KeyID of the subkey with encrypt capabilities for the following
command:

```bash
# Encrypting your secret key for yourself
gpg --encrypt --recipient Ox37ACA569C5C44787 secret_key
```

You can check that the secret key material is missing with
`gpg --list-secret-keys`, you should see `sec#` instead of `sec`.

```bash
gpg --list-secret-keys
# Output:
sec#  ed25519/0x
# ...snip...
```

The above set of commands are from the
[RiseUp Keep your primary key offline](https://riseup.net/ru/security/message-security/openpgp/gpg-best-practices#keep-your-primary-key-entirely-offline)

## Add your GPG Key to GitHub

Plug your own public key from `gpg --list-keys` in the following command:

```bash
gpg --armor --export <Public-Key>
```

Copy the entire block from `-----BEGIN PGP PUBLIC KEY BLOCK-----` to
`-----END PGP PUBLIC KEY BLOCK-----`

> ❗ You can also paste the above block into a public keyserver such as
> `keys.openpgp.org`. This allows others to find and use your key to encrypt
> messages or verify your signatures. Many tools and users rely on public key
> servers to fetch keys automatically. You can also publish your revocation
> certificates, which help others know if your key is compromised or revoked.
> This can be a privacy concern as key servers publish (and keep) associated
> user IDs and metadata linked to your key, such as your email.

It's the same process as adding an SSH key, Go to Settings, SSH and GPG keys,
`New GPG key` and your all set.

### Sign your Commits for Git

```nix
# git.nix
{...}: {
    programs.git = {
        enable = true;
      extraConfig = {
          commit.gpgsign = true;
          user.signingkey = "0x0666C1A265F156"
      };
    };
}
```

After this, you will be prompted for your Private Keys password on every commit.

If you look at your commits on GitHub, after adding the GPG key and the above
settings to your git setup it will show your commits are `Verified`.

### Backing up Your Keys

```bash
gpg --export-secret-keys --armor --output my-private-key-backup.gpg
```

Your private keys will be encrypted with a passphrase into a .gpg file. Store
this backup in a secure location line an encrypted USB drive. This can prevent
you from losing access to your keys in the case of disk failure or accidents.

You can export your public keys and publish them publicly if you choose:

```bash
gpg --export --armor --output my-public-keys.gpg
```

Now if your keys ever get lost or corrupted, you can import these backups.

## Encrypt a File with PGP

The easy way to do this is with an app like Kleopatra, available as
`pkgs.kdePackages.kleopatra`. Kleopatra will automatically recognize your gpg
keys and enable you to easily encrypt messages by clicking the Notepad, typing
your message and clicking `Sign/Encrypt Notepad`. You can also choose to encrypt
the message with a password, where anyone that has the password can read the
message.

Using the above and below methods enable you to encrypt any message for
basically any service and just copy past the encrypted text into the service for
added privacy.

Encrypting a whole directory is a bit more involved and requires using
compression.

### List your keys and get the key ID

```bash
gpg --list-keys --keyid-format LONG
```

Example output, don't use RSA keys:

```bash
pub   rsa4096/ABCDEF1234567890 2024-01-01 [SC]
uid           [ultimate] Your Name <you@example.com>
sub   rsa4096/1234567890ABCDEF 2024-01-01 [E]
```

- Notice the `sub` and the `[E]` for the subkey with encrypt capabilities.

- The part after the slash on the `pub` line is your key ID (`ABCDEF1234567890`
  in the example)

- You can also use your email or name to refer to the key in most commands.

### Encrypt a file

In order to encrypt a document you must have the public keys of the intended
recipients.

```bash
echo "This file will be encrypted" > file.txt
```

Encrypting for yourself (using a key ID as recipient):

```bash
gpg --encrypt --recipient ABCDEF1234567890 file.txt
```

```bash
ls
│  7 │ file.txt            │ file │     28 B │ now           │
│  8 │ file.txt.gpg        │ file │    191 B │ now           │
```

Encrypting for someone else with their email (public key identifier):

```bash
gpg --output file.gpg --encrypt --recipient jake@proton.me file.txt
```

```bash
ls
file.txt
file.gpg
```

`gpg --encrypt` doesn't modify the original file. It creates a new encrypted
file by default with `gpg` amended to the filename.

```bash
gpg --decrypt file.txt.gpg
gpg: encrypted with cv25519 key, ID 0x4AC131B80CEC833E, created 2025-07-31
      "GPG Key <sayls8@proton.me>"
This file will be encrypted
```

Or, to save the decrypted text to a file:

```bash
gpg --output decrypted_file.txt --decrypt file.txt.gpg
cat decrypted_file.txt
# Output
File: decrypted.txt
This file will be encrypted
```

- You will be asked for the passphrase you used when creating the key in order
  to decrypt the file.

### Sign and Verify Signatures

When you sign a document it is certified and timestamped. If after the doc is
signed, if the doc is further modified in any way the verification of the
signature will fail.

A signature is created using the private key of the signer, and verified with
the corresponding public key. For example, to verify Jakes signature you would
use Jake's public key to see that the work indeed came from him and hasn't been
modified since.

To sign the above `file.txt`:

```bash
gpg output doc.sig --sign file.txt
# To clearsign use:
gpg --clearsign file.txt
# For a detached signature use:
gpg --output doc.sig --detach-sig file.txt
```

You will be prompted for your passphrase.

You can either check the signature or check the signature and recover the
original document.

```bash
# To only check:
gpg --output doc --verify doc.sig
# To verify and extract the document
gpg --output doc --decrypt doc.sig
```

## Email Encryption

Email is inherently insecure, and email-based attacks remain one of the top
vectors for data breaches. Encrypting your email protects your privacy by
ensuring that only the intended recipient can read it. Encrypting your emails
with PGP provides valuable security benefits but also has inherent limitations
that prevent it from being considered truly “secure communication” by modern
standards.

- [Email Self-Defense](https://emailselfdefense.fsf.org/en/)

What its Good for:

- Confidentiality, it prevents unauthorized third parties (like email providers
  or network eavesdroppers) from reading your email content.

- Integrity and authenticity: Digital signatures verify that the email genuinely
  came from the claimed sender and hasn't been altered in transit.

- Long-term confidentiality: Encrypted emails stored on servers or devices
  remain protected even if the storage is later compromised. With companies like
  Gmail giving you a "free" account, that usually means that you are the product
  and you should tread lightly.

**To securely communicate with someone never use email, use a dedicated service
such as Threema, Signal or Brair.** It's hard to recommend any messaging service
at the moment, always do your own research and stay informed on the companies
policies. Signal has taken some heat for the way it has implemented it's
MobileCoin. The biggest issue I've faced is getting other people to care or use
the same e2ee app.

With Thunderbird you can go to settings, Privacy and Security, and scroll to the
bottom where it says "End to End Encryption", Click the Settings tab there,
finally click End-To-End Encryption on the left.

From there, you can click `+ Add Key` next to your email address and either
generate a new key through Thunderbird. If you use this, choose the Curve
protocol or whatever isn't RSA.

Or import your own key which is definitely more secure since you're not trusting
someone else with your private key:

```bash
gpg --export --armor sayls8@gmail.com > publickey.asc
```

Then select `+ Add Key` and choose import your own, this didn't work for me.
What did work was to start composing an email and click on the `OpenPGP` button,
Go to `Key Manager`, `File`, `Import Public Key from a File` and choose your
`publickey.asc`. This way, only you have access to your private key.

- [How e2ee with OpenPGP works in general](https://support.mozilla.org/en-US/kb/introduction-to-e2e-encryption#w_how-e2ee-with-openpgp-works-in-general)

**Import your recipient's public key**

When you start composing an email, you'll see that you need to resolve key
issues if you don't already have the recipients public key. Click `Resolve`, and
either Discover Public Keys Online... or Import Public Keys From File...

Thunderbird has the option to use the OpenPGP Key Manager to view or manage
public keys of your correspondents.

If you're sending encrypted emails to someone you'll need their public key,
there are a few methods of doing this just ensure you verify the Fingerprint
with the person your talking to.

Exporting your public key means creating a copy of the public part of your
cryptographic key pair that you can share with others.

For example, say that Jake wants to send you his public key. First he has to
export his key:

```bash
gpg --output jake.gpg --export jake@proton.me
```

The key is exported in binary format, to export in ASCII-armored format use:

```bash
gpg --armor --export jake@proton.me
```

Now, once he sends this to you, you'll need to import and validate it:

```bash
gpg --import jake.gpg
```

Once the key is imported it should be validated. If you need to validate a key
manually, it is done by verifying the key's fingerprint and then signing the key
to certify it as a valid key.

Check that it exists in your keychain:

```bash
gpg --list-keys
```

You should see Jakes key in the above list.

To certify the key you need to edit it:

```bash
gpg --edit-key jake@proton.me
# List the fingerprint
gpg> fpr
# once the fingerprint is verified with the owner, sign it
gpg> sign
# once signed, you can check the key to list signatures on it
gpg> check
```

> ❗ NOTE: You can use PGP to encrypt any message and paste it into **any**
> software and send it. As long as only you and your recipient are the only
> people to have the private keys, you will be the only people able to decrypt
> the messages. Implementing this correctly is a good way to stop government
> mass surveillance.

## Make your Public Key Highly Available

You should always make sure that you sign your public key before you publish it.
When you distribute your public key, you're distributing the public components
of your master and subkeys as well as the user IDs. If unsigned, this is a
security risk because it's possible for an attacker to tamper with it. The
public key can be modified by adding or substituting keys, or changing user IDs.

Signing the keys provides a web of trust, only the corresponding public key can
be used to verify the signature and ensure it hasn't been modified. Since we are
already only using subkeys for public keys, they are automatically self-signed.

```bash
gpg --output ~/mygpg.key --armor --export your_email@address.com
```

You can then send this file to the other party.

You can also use the GPG interface to upload your key to a key server:

```bash
gpg --list-keys your_email@address.com
```

Copy the key ID for the following command, remember its on the `pub` line after
the `/`.

```bash
gpg --send-keys --keyserver pgp.mit.edu key_id
```

The key will be uploaded to the server and likely be distributed to other key
servers around the world. This is why expiration dates are important, if your
key is lost or stolen, the damage window is limited to the expiration period.
Also remember, you can add more time even after the key has expired.

### Example: Verifying Arch Linux Download

<details>
<summary>

✔️ Click to Expand Example of verifying and signing the archlinux public key

</summary>

First, download both the arch `.iso` and `.sig` files.

I tried a few different methods from <https://archlinux.org/download/#checksums>
and the easiest by far was using Sequoia available in Nixpkgs as
`pkgs.sequoia-sq`:

Download the archlinux public key:

```bash
sq network wkd search pierre@archlinux.org --output release-key.pgp

Found 2 certificates related to the query:

 - 3E80CA1A8B89F69CBA57D98A76A5EF9054449A5C
   - Pierre Schmitz <pierre@archlinux.org> (UNAUTHENTICATED)
   - created 2022-10-31 09:11:51 UTC
   - found via: WKD

 - 4AA4767BBC9C4B1D18AE28B77F2D434B9741E8AC
   - Pierre Schmitz <pierre@archlinux.de> (UNAUTHENTICATED)
   - created 2011-04-10 09:35:33 UTC
   - found via: WKD

Hint: To extract a particular certificate from release-key.pgp, use any of:

  $ sq cert export --keyring=release-key.pgp --cert=3E80CA1A8B89F69CBA57D98A76A5EF9054449A5C

  $ sq cert export --keyring=release-key.pgp --cert=4AA4767BBC9C4B1D18AE28B77F2D434B9741E8AC
```

Export the chosen key to a `.pgp` file:

```bash
sq cert export --keyring=release-key.pgp --cert=3E80CA1A8B89F69CBA57D98A76A5EF9054449A5C > pierre-archlinux.pgp
```

Import into your keychain:

```bash
 gpg --import pierre-archlinux.pgp
gpg: key 0x76A5EF9054449A5C: 9 signatures not checked due to missing keys
gpg: key 0x76A5EF9054449A5C: public key "Pierre Schmitz <pierre@archlinux.org>" imported
gpg: Total number processed: 1
gpg:               imported: 1
gpg: marginals needed: 3  completes needed: 1  trust model: pgp
gpg: depth: 0  valid:   3  signed:   0  trust: 0-, 0q, 0n, 0m, 0f, 3u
gpg: next trustdb check due at 2026-08-23
```

Now, you should see `<pierre@archlinux.org>` and his keys when you run
`gpg --list-keys`

Finally, verify the signature:

```bash
sq verify --signer-file release-key.pgp --signature-file archlinux-2025.08.01-x86_64.iso.sig archlinux-2025.08.01-x86_64.iso
Authenticated signature made by 3E80CA1A8B89F69CBA57D98A76A5EF9054449A5C (Pierre Schmitz <pierre@archlinux.org>)

1 authenticated signature.
```

This shows that the signature was made by the key with the ID
`3E80CA1A8B89F69CBA57D98A76A5EF9054449A5C` (Pierre Schmitz).

GPG authenticated that the signature is valid and that the key used to sign is
trusted in our keyring.

1 authenticated signature confirms the files integrity and authenticity.

We have successfully verified that the file was signed by Pierr's official Arch
Linux key and has not been tampered with.

Since the key has been verified we can now sign it. We will have to import our
primary key to do so since we are keeping it offline for safety.

```bash
gpg --import backup.gpg
```

List your keys to get the arch keyID:

```bash
gpg --list-keys
# ... snip ...
pub   ed25519/0x76A5EF9054449A5C 2022-10-31 [SC] [expires: 2037-10-27]
      Key fingerprint = 3E80 CA1A 8B89 F69C BA57  D98A 76A5 EF90 5444 9A5C
uid                   [  full  ] Pierre Schmitz <pierre@archlinux.org>
uid                   [  full  ] Pierre Schmitz <pierre@archlinux.de>
sub   ed25519/0xD6D13C45BFCFBAFD 2022-10-31 [A] [expires: 2037-10-27]
sub   cv25519/0x7F56ADE50CA3D899 2022-10-31 [E] [expires: 2037-10-27]
```

Sign the key:

```bash
gpg --sign-key 0x76A5EF9054449A5C
```

Now you can Export and publish the new public key and send it to a keyserver:

```bash
gpg --export --armor 0x76A5EF9054449A5C > archlinux-signed.asc
gpg --send-keys 0x76A5EF9054449A5C
```

The more people that verify, sign, and re-export and publish their keys the
better for the web of trust that gpg uses making the network more secure for
everyone.

### Edit your trust level of the key

```bash
gpg --edit-key pierre@archlinux.org
gpg> trust
Please decide how far you trust this user to correctly verify other users' keys
(by looking at passports, checking fingerprints from different sources, etc.)

  1 = I don't know or won't say
  2 = I do NOT trust
  3 = I trust marginally
  4 = I trust fully
  5 = I trust ultimately
  m = back to the main menu

Your decision? 3
# Output:
pub  ed25519/0x76A5EF9054449A5C
     created: 2022-10-31  expires: 2037-10-27  usage: SC
     trust: marginal      validity: full
```

You can see that the trust is `marginal` and validity is `full`.

</details>
