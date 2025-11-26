---
title: Nixpkgs Overlays
date: 2025-11-22
author: saylesss88
---

# Nixpkgs Overlays

The following is done with a local clone of Nixpkgs located at `~/src/nixpkgs`.

In this example, we will create an overlay to override the version of
`btrfs-progs`. In the root directory of our local clone of Nixpkgs
(i.e.`~/src/nixpkgs`) we can run the following command to locate `btrfs-progs`
within Nixpkgs:

```bash
fd 'btrfs-progs' .
./pkgs/by-name/bt/btrfs-progs/
```

Open the `package.nix` in the above directory and copy the `src` block within
the `stdenv.mkDerivation` block like so:

```nix
# package.nix
  version = "6.14";

  src = fetchurl {
    url = "mirror://kernel/linux/kernel/people/kdave/btrfs-progs/btrfs-progs-v${version}.tar.xz";
    hash = "sha256-31q4BPyzbikcQq2DYfgBrR4QJBtDvTBP5Qzj355+PaE=";
  };
```

When we use the above `src` block in our overlay we'll need to add
`src = self.fetchurl` for our overlay to have access to `fetchurl`.

We will replace the version with our desired version number. To find another
version that actually exists we need to check their github repos
[btrfs-progs Releases](https://github.com/kdave/btrfs-progs/releases). I can see
that the previous version was `v6.13`, lets try that.

```bash
cd ~/src/nixpkgs
hx overlay.nix
```

We will change the version to `6.13` for demonstration purposes. All that is
really required is changing the version and 1 character in the `hash` which
would cause a refetch and recalculation of the hash. We will use an empty string
to follow convention:

```nix
# overlay.nix
self: super: {
  btrfs-progs = super.btrfs-progs.overrideAttrs (old: rec {
      version = "6.13";

      # Notice the `self` added here
      src = self.fetchurl {
        url = "mirror://kernel/linux/kernel/people/kdave/btrfs-progs/btrfs-progs-v${version}.tar.xz";
        hash = "";
      };
    };
  });
}
```

To build this with the file right from the root of the local Nixpkgs (i.e.
`~/src/nixpkgs`) you could run the following. Running the command this way
avoids the impurity of looking it up in the `~/.config` directory:

```bash
nix-build -A btrfs-progs --arg overlays '[ (import ./overlay.nix) ]'
```

The compiler will give you back the correct `hash`:

```bash
specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
got:    sha256-ZbPyERellPgAE7QyYg7sxqfisMBeq5cTb/UGx01z7po=
```

Replace the empty `hash` with the new hash value we just got from the compiler
so the `overlay.nix` would look like this:

```nix
self: super: {
  btrfs-progs = super.btrfs-progs.overrideAttrs (old: rec {
    version = "6.13";

    src = self.fetchurl {
      url = "mirror://kernel/linux/kernel/people/kdave/btrfs-progs/btrfs-progs-v${version}.tar.xz";
      hash = "sha256-ZbPyERellPgAE7QyYg7sxqfisMBeq5cTb/UGx01z7po=";
    };
  });
}
```

Try building it again:

```bash
nix-build -A btrfs-progs --arg overlays '[ (import ./overlay.nix) ]'
checking for references to /build/ in /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13...
gzipping man pages under /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/share/man/
patching script interpreter paths in /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13
/nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/bin/fsck.btrfs: interpreter directive changed from "#!/bin/sh -f" to "/nix/store/xy4jjgw87sbgwylm5kn047d9gkbhsr9x-bash-5.2p37/bin/sh -f"
stripping (with command strip and flags -S -p) in  /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/lib /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/bin
/nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13
```

We can inspect it with the repl:

```bash
cd ~/src/nixpkgs
nix repl
nix-repl> :a import ./. { overlays = [ (import ./overlay.nix) ]; }
nix-repl> btrfs-progs
«derivation /nix/store/6yxhj84cwcsnrd87rcxbd6w08l9ikc6p-btrfs-progs-6.13.drv»
nix-repl> btrfs-progs.drvAttrs.buildInputs
[
  «derivation /nix/store/yg4llzkcla5rppv8r1iikyamfxg3g4sg-acl-2.3.2.drv»
  «derivation /nix/store/vqczbcwjnid6bs4cv3skl7kyd6kkzcfx-attr-2.5.2.drv»
  «derivation /nix/store/xrvx0azszpdh2x0lnldakqx25vfxab19-e2fsprogs-1.47.2.drv»
  «derivation /nix/store/iil4b8adk615zhp6wmzjx16z1v2f8f4j-util-linux-minimal-2.41.drv»
  «derivation /nix/store/wwld8wp91m26wz69gp8vzh090sh5ygxd-lzo-2.10.drv»
  «derivation /nix/store/w4ncw24gdfkbx9779xpgjli5sagi506m-systemd-minimal-libs-257.5.drv»
  «derivation /nix/store/dmh4lvmq6n8hy56q93kplvnfnlwqzzv5-zlib-1.3.1.drv»
  «derivation /nix/store/h8iwhnr636dwb72qqcyzp111ajjxgzr2-zstd-1.5.7.drv»
]
nix-repl> btrfs-progs.drvAttrs.version
"6.13"
nix-repl> btrfs-progs.drvAttrs.src
«derivation /nix/store/y5nkz1xczxha4xl93qq3adndyc46dcvf-btrfs-progs-v6.13.tar.xz.drv»
```

Using `:a` adds the attributes from the resulting set into scope and avoids
bringing the entire `nixpkgs` set into scope.

To see whats available, you can for example type `btrfs-progs.drvAttrs.` then
hit `TAB`.

Another way to do this is to move our overlay to the
`~/.config/nixpkgs/overlays` directory and rename the file like the following,
agian this adds an impurity because it relies on your `~/.config` directory
which is different from user to user:

```bash
mv overlay.nix ~/.config/nixpkgs/overlays/btrfs-progs.nix
cd ~/src/nixpkgs
nix-build -A btrfs-progs
checking for references to /build/ in /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13...
gzipping man pages under /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/share/man/
patching script interpreter paths in /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13
/nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/bin/fsck.btrfs: interpreter directive changed from "#!/bin/sh -f" to "/nix/store/xy4jjgw87sbgwylm5kn047d9gkbhsr9x-bash-5.2p37/bin/sh -f"
stripping (with command strip and flags -S -p) in  /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/lib /nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13/bin
/nix/store/szd6lizahidjniz85a0g1wsrfknirhwb-btrfs-progs-6.13
```

## Overlays with Flakes

In a flake, overlays are defined in the `outputs.overlays` attribute set of the
`flake.nix`.

They are then applied to `nixpkgs` inputs using
`inputs.nixpkgs.follows = "nixpkgs";` (or similar) and the overlays attribute on
the input.

Example of flake usage:

```nix
# flake.nix
{
  description = "My NixOS flake with custom overlays";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: {

    overlays.myCustomOverlay = final: prev: {
      btrfs-progs = prev.btrfs-progs.overrideAttrs (old: rec {
        version = "6.13";
        src = self.fetchurl {
          url = "mirror://kernel/linux/kernel/people/kdave/btrfs-progs/btrfs-progs-v${version}.tar.xz";
          hash = "sha256-ZbPyERellPgAE7QyYg7sxqfisMBeq5cTb/UGx01z7po=";
        };
      });
    };

    nixosConfigurations.my-system = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # Apply the overlay
        { nixpkgs.overlays = [ self.overlays.myCustomOverlay ]; }
        ./configuration.nix
      ];
    };
  };
}
```

```bash
nix flake show
path:/home/jr/btrfs-progs?lastModified=1749655369&narHash=sha256-ln6dLiqo7TxStQSXgcIwfbdt7STGw4ZHftZRfWpY/JQ%3D
├───nixosConfigurations
│   └───my-system: NixOS configuration
└───overlays
    └───myCustomOverlay: Nixpkgs overlay
```
