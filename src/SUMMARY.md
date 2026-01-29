# Landing Page

[Introduction](index.md)

---

# Core Concepts

- [Getting Started with Nix](./Getting_Started_with_Nix_1.md)
- [The Nix Language](nix/nix_language.md)
- [Nix Package Manager](nix/nix_package_manager.md)
- [Nix Paths](nix/nixLang/nix_paths.md)
- [Understanding Functions](./Understanding_Nix_Functions_2.md)
  - [Practical Functions](functions/practical_functions_2.1.md)
  - [Functions & Modules](functions/functions_and_modules_2.2.md)
- [NixOS Modules](./NixOS_Modules_Explained_3.md)
- [Top-Level Attributes](./Understanding_Top-Level_Attributes_5.md)
- [Derivations](./Intro_to_Nix_Derivations_7.md)

# Flakes & Modern Workflows

- [Nix Flakes Explained](./Nix_Flakes_Explained_4.md)
  - [Inputs](flakes/flake_inputs_4.1.md)
  - [Outputs](flakes/flake_outputs_4.2.md)
  - [Examples](flakes/flake_examples_4.3.md)
- [Overlays & Custom Packages](flakes/overlays_4.5.md)
- [Multiple Profiles (Specialisations)](flakes/specialisations_4.6.md)
- [Case Study: Helix Flake](flakes/helix_flake_4.4.md)
- [Flakes vs. Traditional Nix](./Comparing_Flakes_and_Traditional_Nix_8.md)

---

# Installation & Storage

- [Installation Guide](installation/README.md)
  - [Unencrypted Setups](installation/unenc/unencrypted_setups.md)
  - [Encrypted Setups](installation/enc/enc_install.md)
  - [USB Keyfile Unlock](installation/enc/USB_keyfile.md)
  - [Impermanence (BTRFS Unencrypted)](installation/unenc/unenc_impermanence.md)
  - [Impermanence (BTRFS Encrypted)](installation/enc/encrypted_impermanence.md)
  - [ZFS with LUKS](installation/enc/encrypted_ZFS.md)
- [Secrets Management (Sops-Nix)](installation/enc/sops-nix.md)

---

# Security & Hardening

- [Hardening Philosophy](nix/README.md)
- [Hardening NixOS](nix/hardening_NixOS.md)
- [Network Security](nix/hardening_networking.md)
- [Browser Privacy](nix/browsing_security.md)
- [GnuPG & Agent](nix/gpg-agent.md)
- [Secure Boot (Lanzaboote)](installation/enc/lanzaboote.md)

---

# Virtualization & Isolation

- [Secureblue Host + NixOS Guest VM](nix/kvm.md)
- [NixOS Host Whonix KVM Guest](nix/whonix_kvm.md)
- [Secure Boot (libvirt)](nix/secureboot_libvirt.md)
- [NixOS Containers](./nixos_containers.md)

---

# Development & Packaging

- [Git](vcs/git.md)
- [Jujutsu (JJ)](vcs/jujutsu.md)
- [Nushell on NixOS](./intro_to_nushell_on_NixOS.md)
- [Cachix & Devour](nix/cachix_devour.md)
- [Working with Nixpkgs](./Working_with_Nixpkgs_Locally_10.md)
  - [Fork, Clone, Contribute](nixpkgs/fork_clone_contribute.md)
  - [Building Local Packages](nixpkgs/local_package.md)
  - [Packaging Rust Crates](nixpkgs/rust_crate_to_nixpkgs.md)
  - [Version Overrides](nixpkgs/overlay.md)
  - [Debugging Modules](./Debugging_and_Tracing_NixOS_Modules_9.md)
  - [Pull Requests](./Nix_Pull_Requests_11.md)
- [Package Definition](./Package_Definitions_Explained_6.md)
