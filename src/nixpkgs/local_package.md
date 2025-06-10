# Creating and Building a Local Package within a Nixpkgs Clone

While an actual submission to Nixpkgs involves more steps, this chapter
demonstrates the fundamental pattern for creating a package. Every package
recipe is a file that declares a function. This function takes the packages
dependencies as argument.

In this example we'll make a simple package with `coreutils` and build it.
Demonstrating the process of building and testing a local package.

## Clone Nixpkgs

First, we'll clone Nixpkgs and try to find a good spot to put our package. We're
just building a test package so `nixpkgs/pkgs/misc` could be a good place to
start. We'll call our package `testPackage`.

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

Ad-hoc semi-regular structure, if you need to make a new package we first make a
directory with the name of the package and a `default.nix` in said directory:

## Create your Package directory and a `default.nix`

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

## Tie it in with Nixpkgs top-level package bundle

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

> `callPackage` is a core utility in Nixpkgs. It takes a Nix expression (like
> our `default.nix` file, which defines a function) and automatically provides
> the function with any arguments it declares, by looking them up within the
> `pkgs` set (or the scope where `callPackage` is invoked). This means you only
> need to list the dependencies your package needs in its `default.nix` function
> signature, and `callPackage` will "inject" the correct versions of those
> packages. This is what the `callPackage` Nix Pill demonstrates at a lower
> level.

## Try Building the Package

Move to the root directory of Nixpkgs:

```bash
cd ~/src/nixpkgs
```

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

Tools like `nix search` and the Nixpkgs website use the `meta` information for
documentation and discoverability. It can also be useful for debugging and helps
to provide better error messages. The above command shows that the
`meta.position` attribute points to the file and line where the package
definition begins, which is very useful for debugging.

Typically a file will have a `meta` attribute that looks similar to the
following:

```nix
meta = with lib; {
    homepage = "https://www.openssl.org/";
    description = "A cryptographic library that implements the SSL and TLS protocols";
    license = licenses.openssl;
    platforms = platforms.all;
} // extraMeta;
```

For example, the following shows how Nix is able to discover different parts of
your configuration:

Launch the `nix repl` and load your local flake:

```bash
cd /src
nix repl
nix-repl> :lf nixpkgs
nix-repl> outputs.legacyPackages.x86_64-linux.openssl.meta.position
"/nix/store/syvnmj3hhckkbncm94kfkbl76qsdqqj3-source/pkgs/development/libraries/openssl/default.nix:303"
nix-repl> builtins.unsafeGetAttrPos "description" outputs.legacyPackages.x86_64-linux.openssl.meta
{
  column = 9;
  file = "/nix/store/syvnmj3hhckkbncm94kfkbl76qsdqqj3-source/pkgs/development/libraries/openssl/default.nix";
  line = 303;
}
```

Lets create just the `meta.description` for demonstration purposes.

## Adding the meta attribute

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

Now it points us to the 11'th line, right where our `meta.description` is.

Let's stage our package so nix recognises it:

```bash
cd ~/nixpkgs
git add pkgs/misc/testPackage/
nix edit .#testPackage
```

The `default.nix` that we've been working on should open in your `$EDITOR`
