# Build a local package

In this example we'll make a simple package with `coreutils` and build it.
Demonstrating the process of building and testing a local package.

```bash
cd ~
mkdir src && cd src
git clone git@github.com:NixOS/nixpkgs.git
cd nixpkgs/pkgs
ls # Try to find a catagory that your pkg fits in
╭────┬────────────────┬──────┬─────────┬─────────────╮
│  # │      name      │ type │  size   │  modified   │
├────┼────────────────┼──────┼─────────┼─────────────┤
│  0 │ README.md      │ file │ 50.6 kB │ 2 hours ago │
│  1 │ applications   │ dir  │   398 B │ 2 hours ago │
│  2 │ build-support  │ dir  │  2.5 kB │ 2 hours ago │
│  3 │ by-name        │ dir  │  2.9 kB │ 2 hours ago │
│  4 │ common-updater │ dir  │   286 B │ 2 hours ago │
│  5 │ data           │ dir  │    82 B │ 2 hours ago │
│  6 │ desktops       │ dir  │   164 B │ 2 hours ago │
│  7 │ development    │ dir  │   882 B │ 2 hours ago │
│  8 │ games          │ dir  │  1.5 kB │ 2 hours ago │
│  9 │ kde            │ dir  │   116 B │ 2 hours ago │
│ 10 │ misc           │ dir  │   390 B │ 2 hours ago │
│ 11 │ os-specific    │ dir  │    42 B │ 2 hours ago │
│ 12 │ pkgs-lib       │ dir  │    68 B │ 2 hours ago │
│ 13 │ servers        │ dir  │  1.0 kB │ 2 hours ago │
│ 14 │ shells         │ dir  │    46 B │ 2 hours ago │
│ 15 │ stdenv         │ dir  │   178 B │ 2 hours ago │
│ 16 │ test           │ dir  │   702 B │ 2 hours ago │
│ 17 │ tools          │ dir  │   342 B │ 2 hours ago │
│ 18 │ top-level      │ dir  │  2.3 kB │ 2 hours ago │
╰────┴────────────────┴──────┴─────────┴─────────────╯
```

```bash
cd misc
mkdir testPackage && cd testPackage
hx default.nix
```

```nix
# default.nix
{
  runCommand,
  coreutils,
}:
runCommand "testPackage" {
  nativeBuildInputs = [
    coreutils
  ];
} ''

  echo 'This is a Test' > $out
''
```

Now we need to add our `testPackage` to `all-packages.nix`

```bash
cd pkgs/top-level
hx all-packages.nix
```

`all-packages.nix` is a centralized module that defines all available package
expressions.

We'll add our package in the list alphabetically:

```nix
# all-packages.nix
# `/msc` # editor search inside file
# Scroll down to t's
# snip ...
termusic = callPackage ../applications/autio/termusic { };

# we add our package here
testPackage = callPackage ../misc/testPackage { };

tfk8s = callPackage ../applications/misc/tfk8s { };
# snip ...
```

Move to the root directory of Nixpkgs

Try building it:

```bash
nix-build -A testPackage
this derivation will be built:
this derivation will be built:
  /nix/store/yrbjsxmgzkl24n75sqjfxbpv5cs3b9hc-testPackage.drv
building '/nix/store/yrbjsxmgzkl24n75sqjfxbpv5cs3b9hc-testPackage.drv'...
/nix/store/3012zlv30vn6ifihr1jxbg5z3ysw0hl3-testPackage
```

`runCommand` is a simple builder, it takes 3 arguments. The first is the package
name the second is the derivation attributes, and the third is the script to
run.

```bash
cat ~/src/nixpkgs/result
───────┬──────────────────────────────
       │ File: result
───────┼──────────────────────────────
   1   │ This is a Test
───────┴──────────────────────────────
```

```bash
nix-instantiate --eval -A testPackage.meta.position
"/home/jr/src/nixpkgs/pkgs/misc/testPackage/default.nix:6"
```

Since we don't have a `meta` attribute this points to a default value that's
incorrect.

Let's add the `meta` attribute and try it again:

```nix
# default.nix
{
  runCommand,
  coreutils,
}:
runCommand "testPackage" {
  nativeBuildInputs = [
    coreutils
  ];

  meta = {
    description = "test package";
};
} ''

  echo 'This is a Test' > $out
''
```

```nix
nix-instantiate --eval -A testPackage.meta.position
"/home/jr/src/nixpkgs/pkgs/misc/testPackage/default.nix:11"
```

Now it points us to the 11'th line, right where our meta description is.

Let's stage our package so nix recognises it:

```bash
cd ~/nixpkgs
git add .
nix edit .#testPackage
```

The `default.nix` that we've been working on should open in your `$EDITOR`
