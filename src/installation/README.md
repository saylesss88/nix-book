# Installation

Choose either the **encrypted** or **unencrypted** installation method. After
installation, you can choose to you can add features `sops` (encrypted secrets)
and `lanzaboote` (Secure Boot) to further secure your system.

1. Unencrypted Installation

- Best for: Users who want a simple setup, or who plan to use the impermanence
  feature (which, by default, only works with the unencrypted disk
  configuration).

- Note: If you intend to use impermanence as described in the impermanence
  chapter, you must use the unencrypted layout. The scripts and configuration
  for impermanence assume this setup and may need careful adjustment for other
  disk layouts.

2. Encrypted Installation

- Best for: Users who want full disk encryption for maximum security.

- Recommended: This is the recommended setup if you plan to use sops or
  lanzaboote, as encryption provides a strong foundation for these security
  features.

- Note: You can add sops and lanzaboote after installation if you wish, but
  encrypted setup is required for the highest level of protection.

3. Adding Optional Security Features

- sops: For managing encrypted secrets in your configuration.

- lanzaboote: For enabling Secure Boot support (requires UEFI and custom keys).

- impermanence: Only available with the unencrypted layout as described in the
  impermanence chapter. If you want to use impermanence with encryption, you
  will need to carefully adjust the scripts and configuration.
