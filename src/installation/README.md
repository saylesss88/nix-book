# Installation Guides

This section provides detailed guides for installing NixOS. You'll choose
between an **unencrypted** or **encrypted** base setup. After your core
installation, you can explore adding optional features like `sops` for encrypted
secrets, `lanzaboote` for Secure Boot, or `impermanence` for a stateless system.

---

## 1. Unencrypted Installation

- **Guide:**
  [Minimal Btrfs-Subvol Install with Disko and Flakes](./unencrypted/minimal_install.md)
- **Best for:**

  - Users who want a straightforward and quick setup.

  - Users who plan to implement the `impermanence` feature, which is currently
    designed and tested for this unencrypted Btrfs layout.

- **Note on Impermanence:** If you intend to use `impermanence` as described in
  its dedicated chapter, you **must** follow this unencrypted layout. The
  provided scripts and configurations for `impermanence` assume this specific
  setup and would require significant, careful adjustment for other disk
  layouts.

---

## 2. Encrypted Installation

- **Manual Encrypted Install Guide:**
  [Manual Encrypted Install](https://github.com/saylesss88/nix-book/blob/main/src/installation/encrypted_manual.md)

- **Important Considerations:**

  - [Secure Boot with Lanzaboote](https://saylesss88.github.io/nix/lanzaboote.html)
    For the full benefit of Secure Boot (with Lanzaboote), it's highly
    recommended to have a second stage of protection, such as an encrypted disk.

  - [Adding Sops](https://saylesss88.github.io/nix/sops-nix.html) You can easily
    add `sops` (for managing encrypted secrets) to your configuration _after_
    the initial encrypted installation and reboot. This can simplify the initial
    setup process. However, always remember the core goal of using encrypted
    secrets: **never commit unencrypted or even hashed sensitive data directly
    into your Git repository.** With modern equipment brute force attacks are a
    real threat.

---

## 3. Post-Installation Security & Features

Once your base NixOS system is installed, consider these powerful additions:

- **`sops-nix`:** For managing encrypted secrets directly within your NixOS
  configuration, ensuring sensitive data is never stored in plain text.

- **`lanzaboote`:** For enabling Secure Boot, verifying the integrity of your
  boot chain (requires UEFI and custom keys).

- **`impermanence`:** For setting up a stateless NixOS system, where the root
  filesystem reverts to a clean state on every reboot.

  - **Note:** `impermanence` is currently only available and fully supported
    with the **unencrypted** Btrfs layout as described above. I am actively
    working on an `impermanence` script for the encrypted setup.
