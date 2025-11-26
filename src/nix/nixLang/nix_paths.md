---
title: Nix Paths
date: 2025-11-22
author: saylesss88
---

# Nix Paths

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

The following examples are done with a local `nixpkgs` clone located at
`~/src/nixpkgs`

Paths in Nix always need a `/` in them and always expand to absolute paths
relative to your current directory.

```bash
nix repl
nix-repl> ./.
/home/jr/src/nixpkgs
nix-repl> ./. + "/lib"
/home/jr/src/nixpkgs/lib
```

Nix does _path normalization_ every time you append strings, so if you just add
a slash `/` its not actually there:

```bash
nix-repl> ./.
/home/jr/src/nixpkgs
nix-repl> ./. + "/"
/home/jr/src/nixpkgs
nix-repl> ./. + "/" + "lib"
/home/jr/src/nixpkgslib
nix-repl> "${./.}/lib"
# using ${./.} causes a store copy
copying '/homr/jr/src/nixpkgs' to the store
"/nix/store/3z9fzx8z03wslxvri5syv3jnnhn5fkbd-nixpkgs/lib"
nix-repl> "${toString ./.}/lib"
# using toString avoids making a store copy
"/home/jr/src/nixpkgs/lib"
nix-repl> ./lib/..             # nix removes all `..` to avoid redundant path resolutions
/home/jr/src/nixpkgs
nix-repl> :q
```

```bash
realpath ./lib/..
/home/jr/src/nixpkgs
ln -s pkgs/applications lib-symlink
realpath ./lib-symlink/..
/home/jr/src/nixpkgs/pkgs
nix repl
nix-repl> ./lib-symlink/..   # Nix doesn't read this file at all like realpath did
/home/jr/src/nixpkgs
nix-repl> builtins.readDir ./. # listing of all entries in current dir and their types
{
  ".devcontainer" = "directory";
  ".editorconfig" = "regular";
  ".git" = "directory";
  ".git-blame-ignore-revs" = "regular";
  ".gitattributes" = "regular";
  ".github" = "directory";
  ".gitignore" = "regular";
  ".mailmap" = "regular";
  ".mergify.yml" = "regular";
  ".version" = "symlink";
  "CONTRIBUTING.md" = "regular";
  COPYING = "regular";
  "README.md" = "regular";
  ci = "directory";
  "default.nix" = "regular";
  doc = "directory";
  "flake.nix" = "regular";
  lib = "directory";
  maintainers = "directory";
  nixos = "directory";
  pkgs = "directory";
  "shell.nix" = "regular";
}
nix-repl> builtins.readFile ./default.nix
"let\n  requiredVersion = import ./lib/minver.nix;\nin\n\nif !builtins ? nixVersion
 || builtins.compareVersions requiredVersion builtins.nixVersion == 1 then\n\n  abort
 ''\n\n    This version of Nixpkgs requires Nix >= \${requiredVersion}, please
 upgrade:\n\n    - If you are running NixOS, `nixos-rebuild' can be used to upgrade
 your system.\n\n    - Alternatively, with Nix > 2.0 `nix upgrade-nix' can be used
 to imperatively\n      upgrade Nix. You may use `nix-env --version' to check which
 version you have.\n\n    - If you installed Nix using the install script (https://nixos.org/nix/install),\n
  it is safe to upgrade by running it again:\n\n          curl -L https://nixos.org/nix/install | sh\n\n
For more information, please see the NixOS release notes at\n    https://nixos.org/nixos/manual
 or locally at\n    \${toString ./nixos/doc/manual/release-notes}.\n\n    If you need further help,
 see https://nixos.org/nixos/support.html\n  ''\n\nelse\n\n  import ./pkgs/top-level/impure.nix\n"
nix-repl> :l <nixpkgs/lib>
nix-repl> importJSON ./pkgs/development/python-modules/notebook/missing-hashes.json # Return the nix value for JSON
{
  "@nx/nx-darwin-arm64@npm:16.10.0" = "aabcc8499602b98c9fc3b768fe46dfd4e1b818caa84b740bd4f73a2e4528c719b979ecb1c10a0d793a1fead83073a08bc86417588046aa3e587e80af880bffd3";
  "@nx/nx-darwin-x64@npm:16.10.0" = "9dd20f45f646d05306f23f5abb7ade69dcb962e23a013101e93365847722079656d30a19c735fdcfa5c4e0fdf08691f9d621073c58aef2861c26741ff4638375";
  "@nx/nx-freebsd-x64@npm:16.10.0" = "35b93aabe3b3274d53157a6fc10fec7e341e75e6818e96cfbc89c3d5b955d225ca80a173630b6aa43c448c6b53b23f06a2699a25c0c8bc71396ee20a023d035f";
  "@nx/nx-linux-arm-gnueabihf@npm:16.10.0" = "697b9fa4c70f84d3ea8fe32d47635864f2e40b0ceeb1484126598c61851a2ec34b56bb3eeb9654c37d9b14e81ce85a36ac38946b4b90ca403c57fe448be51ccb";
  "@nx/nx-linux-arm64-gnu@npm:16.10.0" = "001e71fedfc763a4dedd6c5901b66a4a790d388673fb74675235e19bb8fe031ff3755568ed867513dd003f873901fabda31a7d5628b39095535cb9f6d1dc7191";
  "@nx/nx-linux-arm64-musl@npm:16.10.0" = "58e3b71571bdadd2b0ddd24ea6e30cd795e706ada69f685403412c518fba1a2011ac8c2ac46145eab14649aa5a78e0cedcdb4d327ccb3b6ec12e055171f3840b";
  "@nx/nx-linux-x64-gnu@npm:16.10.0" = "97729a7efb27301a67ebf34739784114528ddb54047e63ca110a985eaa0763c5b1ea7c623ead1a2266d07107951be81e82ffa0a30e6e4d97506659303f2c8c78";
  "@nx/nx-linux-x64-musl@npm:16.10.0" = "442bdbd5e61324a850e4e7bd6f54204108580299d3c7c4ebcec324da9a63e23f48d797a87593400fc32af78a3a03a3c104bfb360f107fe732e6a6c289863853a";
  "@nx/nx-win32-arm64-msvc@npm:16.10.0" = "b5c74184ebfc70294e85f8e309f81c3d40b5cf99068891e613f3bef5ddb946bef7c9942d9e6c7688e22006d45d786342359af3b4fc87aadf369afcda55c73187";
  "@nx/nx-win32-x64-msvc@npm:16.10.0" = "c5b174ebd7a5916246088e17d3761804b88f010b6b3f930034fa49af00da33b6d1352728c733024f736e4c2287def75bafdc3d60d8738bd24b67e9a4f11763f8";
}
nix-repl> builtins.toJSON  # serialize
«primop toJSON»
nix-repl> builtins.fromTOML
«primop fromTOML»
nix-repl> builtins.toXML
```

For more serialization formats see `nixpkgs/lib/generators.nix` as well as in
`nixpkgs/pkgs/pkgs-lib/formats/` we can see them with the `nix repl` as follows:

```bash
cd ~/src/nixpkgs
nix repl
nix-repl> :l .
nix-repl> lib.generators.toYAML {} { a = 10; }
"{\"a\":10}"
nix-repl> lib.generators.toYAML {} { a.b.c = 10; }
"{\"a\":{\"b\":{\"c\":10}}}"
nix-repl> builtins.trace (lib.generators.toYAML {} { a.b.c = 10; }) null
trace: {"a":{"b":{"c":10}}}
null
nix-repl> yamlFormat = pkgs.formats.yaml {}

nix-repl> yamlFormat
{
  generate = «lambda generate @ /home/jr/src/nixpkgs/pkgs/pkgs-lib/formats.nix:111:9»;
  type = { ... };
}
```

- We can see that it provides a `generate` function that we can use. `generate`
  doesn't just generate a string anymore because if we want to lift the
  restriction at evaluation time we can't return the formatted form at
  evaluation time anymore. We need a name to return a derivation continued
  below:

```bash
yamlFormat.generate "name" { a.b.c = 10; }
«derivation /nix/store/xakajb2rzbmqqkjbh08bxwqdf0xqvjly-name.drv»
nix-repl> :b yamlFormat.generate "name" { a.b.c = 10; }
This derivation produced the following outputs:
out -> /nix/store/y4c5029k6w3l0qmdw7cq396zrdy5x8yj-name
nix-repl> :q
```

Let's cat the result to see if it's formatted correctly as YAML:

```bash
cat /nix/store/y4c5029k6w3l0qmdw7cq396zrdy5x8yj-name
───────┬───────────────────────────────────────────────────────────────
       │ File: /nix/store/y4c5029k6w3l0qmdw7cq396zrdy5x8yj-name
───────┼──────────────────────────────────────────────────────────────
   1   │ a:
   2   │   b:
   3   │     c: 10
───────┴───────────────────────────────────────────────────────────
```

Looks good. There is also a `type`:

```bash
nix repl
nix-repl> :l .
nix-repl> yamlFormat = pkgs.format.yaml {}
nix-repl> yamlFormat.type
{
  _type = "option-type";
  check = «lambda check @ /home/jr/src/nixpkgs/lib/types.nix:1029:19»;
  deprecationMessage = null;
  description = "YAML value";
  descriptionClass = "conjunction";
  emptyValue = { ... };
  functor = { ... };
  getSubModules = null;
  getSubOptions = «lambda @ /home/jr/src/nixpkgs/lib/types.nix:214:25»;
  merge = «lambda merge @ /home/jr/src/nixpkgs/lib/types.nix:1031:13»;
  name = "nullOr";
  nestedTypes = { ... };
  substSubModules = «lambda substSubModules @ /home/jr/src/nixpkgs/lib/types.nix:1046:29»;
  typeMerge = «lambda defaultTypeMerge @ /home/jr/src/nixpkgs/lib/types.nix:115:10»;
}
nix-repl> lib.modules.mergeDefinitions [] yamlFormat.type [ { value = null; } ]
{
  defsFinal = [ ... ];
  defsFinal' = { ... };
  isDefined = true;
  mergedValue = null;
  optionalValue = { ... };
}
nix-repl> (lib.modules.mergeDefinitions [] yamlFormat.type [ { value = null; } ]).mergedValue
null
nix-repl> :p (lib.modules.mergeDefinitions [] yamlFormat.type [ { value = { a.b.c = 10; }; } ]).mergedValue
{
  a = {
    b = { c = 10; };
  };
}
nix-repl> :p (lib.modules.mergeDefinitions [] yamlFormat.type [ { value = { a.b.c = 10; }; } { value = { a.b.d = 20; }; } ]).mergedValue
{
  a = {
    b = {
      c = 10;
      d = 20;
    };
  };
}
```

- `lib` can't access any packages, it is entirely at evaluation time. It can't
  access any formatters or things like that. If we lift that restriction as is
  done in `pkgs.formats` we can make it look much nicer.

```bash
cd ~/src/nixpkgs
nix-build -A hello
warning: Nix search path entry '/nix/var/nix/profiles/per-user/root/channels' does not exist, ignoring
this path will be fetched (0.06 MiB download, 0.26 MiB unpacked):
  /nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2
copying path '/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2' from 'https://cache.nixos.org'...
/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2
```

Say we rely on this store path in a derivation:

```bash
nix-repl> thePath = "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2"
nix-repl> thePath + "/bin/hello"
"/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello"
```

```bash
hx ~/src/nixpkgs/test2.nix
```

```nix
# test2.nix
with import ./. {};

runCommand "test" {
    nativeBuildInputs = [
        hello
    ];
}''
  hello > $out
''
```

Try building it:

```bash
nix-build test2.nix && cat result
warning: Nix search path entry '/nix/var/nix/profiles/per-user/root/channels' does not exist, ignoring
/nix/store/m55p4vpb8s7s28s20vs89i467kxbrdac-test
Hello, world!
```

Now if we try it with the store path:

```nix
# test2.nix
with import ./. {};

runCommand "test" {
}''
  /nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello > $out
''
```

This doesn't work

```bash
nix-build test2.nix
last 1 log lines:
> /build/.attr-0l2nkwhif96f51f4amnlf414lhl4rv9vh8iffyp431v6s28gsr90: line 1: /nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello: No such file or directory
For full logs, run:
nix log /nix/store/58zcp9xwgf1sirmzf9sh61j8gz9lkw34-test.drv
nix-instantiate test2.nix
/nix/store/58zcp9xwgf1sirmzf9sh61j8gz9lkw34-test.drv
nix derivation show /nix/store/58zcp9xwgf1sirmzf9sh61j8gz9lkw34-test.drv | jq
{
  "/nix/store/58zcp9xwgf1sirmzf9sh61j8gz9lkw34-test.drv": {
    "args": [
      "-e",
      "/nix/store/vj1c3wf9c11a0qs6p3ymfvrnsdgsdcbq-source-stdenv.sh",
      "/nix/store/shkw4qm9qcw5sc5n1k5jznc83ny02r39-default-builder.sh"
    ],
    "builder": "/nix/store/xy4jjgw87sbgwylm5kn047d9gkbhsr9x-bash-5.2p37/bin/bash",
    "env": {
      "__structuredAttrs": "",
      "buildCommand": "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello > $out\n",
      "buildInputs": "",
      "builder": "/nix/store/xy4jjgw87sbgwylm5kn047d9gkbhsr9x-bash-5.2p37/bin/bash",
      "cmakeFlags": "",
      "configureFlags": "",
      "depsBuildBuild": "",
      "depsBuildBuildPropagated": "",
      "depsBuildTarget": "",
      "depsBuildTargetPropagated": "",
      "depsHostHost": "",
      "depsHostHostPropagated": "",
      "depsTargetTarget": "",
      "depsTargetTargetPropagated": "",
      "doCheck": "",
      "doInstallCheck": "",
      "enableParallelBuilding": "1",
      "enableParallelChecking": "1",
      "enableParallelInstalling": "1",
      "mesonFlags": "",
      "name": "test",
      "nativeBuildInputs": "",
      "out": "/nix/store/ljrkx5midby3j7p4g96d74jrq8f9rpya-test",
      "outputs": "out",
      "passAsFile": "buildCommand",
      "patches": "",
      "propagatedBuildInputs": "",
      "propagatedNativeBuildInputs": "",
      "stdenv": "/nix/store/aq801xbgs98nxx3lckrym06qfvl8mfsf-stdenv-linux",
      "strictDeps": "",
      "system": "x86_64-linux"
    },
    "inputDrvs": {
      "/nix/store/bmncp7arkdhrl6nkyg0g420935x792gl-stdenv-linux.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      },
      "/nix/store/rfkzz952hz2d58d90mscxvk87v5wa5bz-bash-5.2p37.drv": {
        "dynamicOutputs": {},
        "outputs": [
          "out"
        ]
      }
    },
    "inputSrcs": [
      "/nix/store/shkw4qm9qcw5sc5n1k5jznc83ny02r39-default-builder.sh",
      "/nix/store/vj1c3wf9c11a0qs6p3ymfvrnsdgsdcbq-source-stdenv.sh"
    ],
    "name": "test",
    "outputs": {
      "out": {
        "path": "/nix/store/ljrkx5midby3j7p4g96d74jrq8f9rpya-test"
      }
    },
    "system": "x86_64-linux"
  }
}
```

You see the `"inputDrvs"`, they are the derivations that we depend on and it
doesn't know about the `hello.drv`. In Nix for the builder sandbox it creates a
sandbox that only contains the derivations that you depend on which ensures that
you can't depend on any derivation that you haven't explicitly decalred.

Nix does have `builtins.storePath` that allows you to do this, otherwise it's
kind of an anti pattern.

```nix
# test2.nix
# test2.nix
# test2.nix
with import ./. {};
  runCommand "test" {
  } ''
    ${builtins.storePath "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2"}/bin/hello > $out
  ''
```

`builtins.storePath`: Turns a store path into the thing that it represents in
the store.

```bash
nix-build test2.nix && cat result
/nix/store/x48741w0k9hgqywzv6wc7rk90r1y75js-test
Hello, world!
```

To demonstrate what `builtins.storePath` does:

```bash
nix-repl> builtins.storePath "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello"
"/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2/bin/hello"
nix-repl> builtins.getContext "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2"
{ }
nix-repl> builtins.getContext (builtins.storePath "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2")
{
  "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2" = { ... };
}
nix-repl> :p builtins.getContext (builtins.storePath "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2")
{
  "/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2" = { path = true; };
}
```

```bash
nix-repl> :l .
warning: Nix search path entry '/nix/var/nix/profiles/per-user/root/channels' does not exist, ignoring
Added 24878 variables.

nix-repl> hello.outPath
# this is the output path of the hello derivation
"/nix/store/29mhfr5g4dsv07d80b7n4bgs45syk3wl-hello-2.12.2"
nix-repl> :p builtins.getContext hello.outPath
# we see that this is a `.drv`, this is because derivations can have multiple outputs
{
  "/nix/store/ljxsxdy1syy03b9kfnnh8x7zsk21fdcq-hello-2.12.2.drv" = {
    outputs = [ "out" ];
  };
}
# for example
nix-repl> openssl.outputs
[
  "bin"
  "dev"
  "out"
  "man"
  "doc"
  "debug"
]
nix-repl> openssl.all
# a list of all the derivations
[
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
  «derivation /nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv»
]
nix-repl> lib.concatStringsSep " " openssl.all
"/nix/store/rjzx8v679rwd6dsb6s08iy3j2rrax72s-openssl-3.4.1-bin /nix/store/kcgqglb4iax0zh5jlrxmjdik93wlgsrq-openssl-3.4.1-dev /nix/store/8pviily4fgsl02ijm65binz236717wfs-openssl-3.4.1 /nix/store/1l5b31cnswnbcdcac9rzs9xixnc2n9r5-openssl-3.4.1-man /nix/store/9fz5qmj0z70cbzy7mapml0sbi8z6ap0a-openssl-3.4.1-doc /nix/store/yk2g2gfcj2fy1ffyi1g91q7jmp4h8pxa-openssl-3.4.1-debug"
nix-repl> :p builtins.getContext (builtins.unsafeDiscardOutputDependency (lib.concatStringsSep " " openssl.all))
{
  "/nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv" = {
    outputs = [
      "bin"
      "debug"
      "dev"
      "doc"
      "man"
      "out"
    ];
  };
}
nix-repl> :p builtins.getContext openssl.drvPath
{
  "/nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv" = { allOutputs = true; };
}
# useful if you need to create a derivation that copies this derivation to another machine
# remote builders usually take care of this but you may need it occasionally
nix-repl> :p builtins.getContext (builtins.unsafeDiscardOutputDependency openssl.drvPath)
{
  "/nix/store/rw3y8k94ib37dc86n0wivr551wyzxgsk-openssl-3.4.1.drv" = { path = true; };
}
```

Relying on paths outside of the nix store is generally not recommended because
of garbage collection and it's considered unsafe.
