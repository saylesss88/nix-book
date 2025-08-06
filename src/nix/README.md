# Hardening README

📌 **How to Use This Guide**

**Read warnings**: Advanced hardening can break compatibility or cause data
loss! Pause and research before enabling anything not listed above unless you
understand the consequences.

The guide is broken up into 2 chapters:

- [Hardening NixOS](https://saylesss88.github.io/nix/hardening_NixOS.html)

- [Hardening Networking](https://saylesss88.github.io/nix/hardening_networking.html)

## Getting Started

There is a lot covered in this guide which can get overwhelming when trying to
decide what is worth implementing. Here, I will list some common recommendations
that most users should follow to harden their stance.

### Baseline Hardening

Before diving into advanced or specialized hardening, apply these baseline
security measures suitable for all NixOS users. These settings help protect your
system with minimal risk of breaking workflows or causing admin headaches.

- Use Disk Encryption (LUKS) to protect your data at rest.

- Keep your system up to date (update regularly).

- Use strong, unique passwords.

- Avoid reusing passwords, use a password manager.

- Enable multi-factor authentication (MFA) wherever possible.

- Declare everything everywhere and disable imperative configuration where
  possible. For example, setting `users.mutableUsers = false;` to ensure that
  all users **must** be declared.

- Only enable what you use, and actively disable what's no longer in use.

- Enable at least a basic firewall:

```nix
# configuration.nix
# this denies incoming connections but allows outgoing and established connections
networking.firewall.enable = true;
```

For deeper explanations and more advanced hardening, see the
[Hardening NixOS Chapter](https://saylesss88.github.io/nix/hardening_NixOS.html)
