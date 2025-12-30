---
title: My Chapter
date: 2025-11-22
author: saylesss88
---

# Installation Guides

This section provides detailed guides for installing NixOS. You'll choose
between an **unencrypted** or **encrypted** base setup. After your core
installation, you can explore adding optional features like `sops` for encrypted
secrets, `lanzaboote` for Secure Boot, or `impermanence` for a stateless system.

---

## 1. Unencrypted Disko Btrfs Subvol Installation

- **Guide:**
  [Minimal Btrfs-Subvol Install with Disko and Flakes](https://saylesss88.github.io/installation/unenc/unencrypted_setups.html)

- **Best for:**
  - Users who want a straightforward and quick setup.

  - [Unencrypted Impermanence](https://saylesss88.github.io/installation/unenc/unenc_impermanence.html)

  - You can still add Lanzaboote and sops secrets after the install for a more
    secure system. To get the full benefits of Lanzaboote it is recommended to
    use full disk encryption.

---

## 2. Encrypted Disko Btrfs Subvol Installation

- **Encrypted Install Guide:**
  [Encrypted Install](https://saylesss88.github.io/installation/enc/enc_install.html)

- [Encrypted Impermanence](https://saylesss88.github.io/installation/enc/encrypted_impermanence.html)

- **Important Considerations:**
  - [Secure Boot with Lanzaboote](https://saylesss88.github.io/installation/enc/lanzaboote.html)
    For the full benefit of Secure Boot (with Lanzaboote), it's highly
    recommended to have a second stage of protection, such as an encrypted disk.

  - [Adding Sops](https://saylesss88.github.io/installation/enc/sops-nix.html)
    You can easily add `sops` (for managing encrypted secrets) to your
    configuration _after_ the initial encrypted installation and reboot. This
    can simplify the initial setup process. However, always remember the core
    goal of using encrypted secrets: **never commit unencrypted or even hashed
    sensitive data directly into your Git repository.** With modern equipment
    brute force attacks are a real threat.

---

## 3. Post-Installation Security & Features

Once your base NixOS system is installed, consider these powerful additions:

- **`sops-nix`:** For managing encrypted secrets directly within your NixOS
  configuration, ensuring sensitive data is never stored in plain text.

- **`lanzaboote`:** For enabling Secure Boot, verifying the integrity of your
  boot chain (requires UEFI and custom keys).

- **`impermanence`:** For setting up a stateless NixOS system, where the root
  filesystem reverts to a clean state on every reboot.
