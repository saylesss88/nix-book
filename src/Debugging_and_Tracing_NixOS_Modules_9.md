---
title: Debugging NixOS modules
date: 2025-11-22
author: saylesss88
---

# Chapter 11

This chapter covers debugging NixOS modules, focusing on tracing module options
and evaluating merges.

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

## Debugging and Tracing NixOS Modules

![404](images/coding4.png)

<!-- ![gruv17](images/gruv17.png) -->

- Other related post if you haven't read my previous post on modules, that may
  be helpful before reading this one:
  - [nix-modules-explained](https://saylesss88.github.io/posts/nix_modules_explained/)

  - This post is my notes following Nix Hour 40. If it seems a little chaotic,
    try watching one. They are hard to follow if you're not extremely familiar
    with the concepts.

  - [Nix Hour 40](https://www.youtube.com/watch?v=aLy8id4wr-M&t=2120s)

Nix Code is particularly hard to **debug** because of (e.g. lazy evaluation,
declarative nature, layered modules)

- The following simple Nix code snippet illustrates a basic NixOS module
  definition and how options are declared and configured. We'll use this example
  to demonstrate fundamental debugging techniques using `nix-instantiate`.

```nix
let
  lib = import <nixpkgs/lib>;
in
lib.evalModules {
  modules = [
    ({ lib, ... }: {
      options.foo = lib.mkOption {
        # type = lib.types.raw;
        type = lib.types.anything;
        # default = pkgs;
      };
      config.foo = {
        bar = 10;
        list = [1 2 3 ];
        baz = lib.mkDefault "baz";
      };
    })
    {
      foo.baz = "bar";
    }
  ];
}
```

- In the above code, adding `lib` to the function arguments isn't required but
  if you were to move the module to another file it would fail without it
  because `lib` comes from outside of it. So it's good practice to refer to
  `lib` in the modules themselves.

- You should **always** assign a type to your options, if you don't know which
  type to use you could use `raw`. `raw` is a type that doesn't do any
  processing. So if you were to assign the entire packages set to the option
  e.g. `default = pkgs;` it wouldn't recurseinto all the packages and try to
  evaluate them. There is also `anything`, that is useful if you do want to
  recurse into the values.

- The following is an example of how you would run this inside vim/neovim, the
  rest of the examples will be from the command line:

```vim
:!nix-instantiate --eval -A config.foo --strict
```

**Output**:

<details>
<summary> Click to Expand the Output </summary>

```bash
{ bar = 10; baz = "bar"; list = [ 1 2 3 ]; }
```

To show the difference you could uncomment the `raw` type and comment the
`anything` type and run the above command again you'll see that you get an
error:

```bash
error: The option 'foo' is defined multiple times while it's expected to be
unique
```

To execute this command on the command line:

```bash
nix-instantiate --eval --strict -A config.foo
```

It will show you the start of a trace. To get the full trace add:

```bash
nix-instantiate --eval --strict -A config.foo --show-trace
```

</details>

## Example 2

<details>
<summary> Click to Expand Example 2 </summary>

In the previous example, we looked at a simplified module. Now, let's examine a
more realistic scenario involving a basic NixOS configuration file
(`configuration.nix`).

This example will demonstrate how to use `nix-instantiate` to evaluate an entire
system configuration and how `--show-trace` helps in diagnosing errors within
this context.

Consider the following `configuration.nix` file:

```nix
# configuration.nix
{ lib, ... }: {
  boot.loader.grub.device = "nodev";
  fileSystems."/".device = "/devst";
  system.stateVersion = "24.11";
}
```

- This configuration snippet sets the GRUB bootloader device, defines a root
  filesystem, and specifies the expected NixOS state version. To evaluate this
  entire system configuration, you can use `nix-instantiate` and point it to the
  `<nixpkgs/nixos>` entrypoint, providing our `configuration.nix` file as an
  argument. The `-A system` flag selects the top-level `system` attribute, which
  represents the instantiated system configuration.

**Run** it in with:

```bash
nix-instantiate '<nixpkgs/nixos>' --arg configuration ./configuration.nix -A system
```

**Output**:

```bash
/nix/store/kfcwvvpdbsb3xcks1s76id16i1mc3l5k-nixos-system-nixos-25.05pre-git.drv
```

Ok, we can see that this successfully _instantiates_. Let's introduce an error
to trace:

```nix
{ lib, ... }: {
  boot.loader.grub.device = "nodev";
  fileSystems."/".device = "/devst";
  system.stateVersion = builtins.genList "24.11" null;
}
```

**Output**:

```bash
(stack trace truncated; use '--show-trace' to show the full, detailed trace)
error: expected an integer but found null: null
```

Rerun the command with `--show-trace` appended:

Or on the command line

```bash
nix-instantiate '<nixpkgs/nixos>' --arg configuration ./configuration.nix -A system --show-trace
```

- This outputs a much longer trace than the first example. It shows you the file
  the error occured in and you can see that in this case they are a lot of
  internal functions. (e.g.
  `at /nix/store/ccfwxygjrarahgfv5865x2f828sjr5h0- source/lib/attrsets.nix:1529:14:`)

To show your own error message you could do something like this:

```nix
{lib, ...}: {
  boot.loader.grub.device = "nodev";
  fileSystems."/".device = "/devst";
  system.stateVersion = builtins.addErrorContext "AAAAAAAAAAAAAAAAA" (builtins.genList "24.11" null);
}
```

Run it:

```bash
nix-instantiate '<nixpkgs/nixos>' --arg configuration ./configuration.nix -A system --show-trace`
```

**Output**:

```bash
 … while evaluating the attribute 'value'
     at /nix/store/ccfwxygjrarahgfv5865x2f828sjr5h0-source/lib/modules.nix:770:21:
      769|             inherit (module) file;
      770|             inherit value;
         |                     ^
      771|           }) module.config

   … AAAAAAAAAAAAAAAAA

   … while calling the 'genList' builtin
     at /home/jr/tests/configuration.nix:4:71:
        3|   fileSystems."/".device = "/devst";
        4|   system.stateVersion = builtins.addErrorContext "AAAAAAAAAAAAAAAAA"
         (builtins.genList "24.11" null);
         |                                                                       ^
        5| }

   … while evaluating the second argument passed to builtins.genList

   error: expected an integer but found null: null
```

- In the latest nix they actually inverted the error messages so the most
  relevant parts will be at the bottom.

</details>

## Example 3

<details>
<summary> Click to Expand Example 3 </summary>

Let's consider another example, this time demonstrating the definition of
configuration options using `lib.mkOption` within a module structure.

```nix
# default.nix
let
  lib = import <nixpkgs/lib>;
in
lib.evalModules {
  modules = [
    ({ lib, ... }: {
      options.ints = lib.mkOption {
        type = lib.types.attrsOf lib.types.int;
      };
      options.strings = lib.mkOption {
        type = lib.types.string;
        # type = lib.types.attrsOf lib.types.string;
        default = "foo";
      };
    })
  ];
}
```

**Instantiate** this with:

```bash
nix-instantiate --eval --strict -A config.strings
```

**Output**:

```bash
evaluation warning: The type `types.string` is deprecated.
See https://github.com/NixOS/nixpkgs/pull/66346 for better alternative types.
"foo"
```

- Unfortunately you won't get the same depreciation warning from `lib.attrsOf`

Below is an interesting way to provide nixpkgs run it on the command line:

```bash
export NIX_PATH=nixpkgs=channel:nixpkgs-unstable
echo $NIX_PATH
```

**Output**:

```bash
nixpkgs=channel:nixpkgs-unstable
```

The next two commands are to check that after using the above way to provide
`nixpkgs-unstable` that they both point to the same store path, the following
command will fetch nixpkgs from the channel above:

```bash
nix-instantiate --find-file nixpkgs
```

**Output** 1️⃣

```bash
/nix/store/ydrgwsibghsyx884qz97zbs1xs93yk11-source
```

```bash
nix-instantiate --eval channel:nixpkgs-unstable -A path
```

**Output**: 2️⃣

```bash
/nix/store/ydrgwsibghsyx884qz97zbs1xs93yk11-source
```

- As you can see both commands produce the same store path

## Example 4

In our previous example, we encountered a deprecation warning for
`lib.types.string`. This next example delves deeper into why that type was
deprecated and demonstrates the consequences of its behavior, along with the
recommended fix.

```nix
# default.nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ({lib, ...}: {
        options.ints = lib.mkOption {
          type = lib.types.attrsOf lib.types.int;
        };
        options.strings = lib.mkOption {
          # type = lib.types.string;
          type = lib.types.attrsOf lib.types.string;
          default = {
            x = "foo";
          };
        };
        config = {
          strings = lib.mkOptionDefault {
            x = "bar";
          };
        };
      })
    ];
  }
```

Evaluate it with:

```bash
nix-instantiate --eval --strict -A config.strings
```

- `types.string` depricated because it silently concatenates strings

- The above command has two options with the same priority level and evaluates
  to `{ x = "foobar"; }`

**Output:**

```bash
evaluation warning: The type `types.string` is deprecated. See https://github.
com/NixOS/nixpkgs/pull/66346 for better alternative types.
{ x = "foobar"; }
```

- `types.str` was the replacement for the depricated `types.string`:

```nix
# default.nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ({lib, ...}: {
        options.ints = lib.mkOption {
          type = lib.types.attrsOf lib.types.int;
        };
        options.strings = lib.mkOption {
          # type = lib.types.string;
          type = lib.types.attrsOf lib.types.str;
          # Sets the value with a lower priority: lib.mkOptionDefault
          default = {
            x = "foo";
          };
        };
        config = {
          strings = lib.mkOptionDefault {
            x = "bar";
          };
        };
      })
    ];
  }
```

**Output:**

```bash
error:
… while evaluating the attribute 'x'

… while evaluating the attribute 'value'
 at /nix/store/ydrgwsibghsyx884qz97zbs1xs93yk11-source/lib/modules.nix:1148:41:
 1147|
 1148|     optionalValue = if isDefined then { value = mergedValue; } else { };
     |                                         ^
 1149|   };

… while calling the 'foldl'' builtin
 at /nix/store/ydrgwsibghsyx884qz97zbs1xs93yk11-source/lib/options.nix:508:8:
  507|     else
  508|       (foldl' (
     |        ^
  509|         first: def:

(stack trace truncated; use '--show-trace' to show the full, detailed trace)

error: The option `strings.x' has conflicting definition values:
- In `<unknown-file>': "foo"
- In `<unknown-file>': "bar"
Use `lib.mkForce value` or `lib.mkDefault value` to change the priority on any of these definitions.

shell returned 1
```

</details>

## Summary

- So types in the module system aren't just types in the conventional sense but
  they also specify the emerging behavior of these values.

- If we switch the type in the above example to `types.lines` you get this
  returned, `{ x = "foo\nbar"; }`

- `mkOptionDefault` isn't typically something you should generally use, instead
  options have a `default` setting

- If you want to make sure that you set a default but if the user specifies it,
  it shouldn't get overridden. You should not set it in the following:

```nix
options.strings = lib.mkOption {
  type = lib.types.attrsOf lib.types.lines;
  default = {
    x = "foo";
  };
}
```

Because the above uses `mkOptionDefault` but instead in under the `config`
attribute like the following:

```nix
# ...snip...
options.strings = lib.mkOption {
  type = lib.types.attrsOf lib.types.lines;
  # default = {
    # x = "foo";
  # };
};
config = {
  strings = {
    x = lib.mkDefault "foo";
  };
};
# ...snip...
```

```nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ({lib, ...}: {
        options.ints = lib.mkOption {
          type = lib.types.attrsOf lib.types.int;
        };
        options.strings = lib.mkOption {
          # type = lib.types.string;
          type = lib.types.attrsOf lib.types.str;
          # Sets the value with a lower priority: lib.mkOptionDefault
          #default = {
          #  x = "foo";
          #};
        };
        config.strings = {
          x = "foo";
        };
      })
      {
        config.strings = {
          y = "bar";
        };
      }
    ];
  }
```

**Output**:

- This works now because there's no difference between `x` and `y`

```bash
{ x = "foo"; y = "bar"; }
```

## More Functionality between modules

```nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ({lib, ...}: {
        options.ints = lib.mkOption {
          type = lib.types.attrsOf lib.types.int;
        };
        options.strings = lib.mkOption {
          # type = lib.types.string;
          type = lib.types.attrsOf lib.types.str;
          # Sets the value with a lower priority: lib.mkOptionDefault
          #default = {
          #  x = "foo";
          #};
        };
        config.strings = {
          x = lib.mkDefault "foo";
        };
      })
      {
        config.strings = {
          x = "x";
          y = "bar";
        };
      }
    ];
  }
```

- The above command would cause a conflict without the `x = lib.mkDefault foo`
  And this is typically what you want to do for defaults and modules in things
  like nested configuration.

**Output:**

```bash
{ x = "x"; y = "bar"; }
```

### Infinite recursion error

1. A common pitfall is to introduce a hard to debug error `infinite recursion`
   when shadowing a name. The simplest example for this is:

> ```nix
> let a = 1; in rec { a = a; }
> ```

> 💡**TIP**: Avoid `rec`. Use `let ... in` Example:
>
> ```nix
> let
>  a = 1;
> in {
>  a = a;
>  b = a + 2;
> }
> ```

<details>
<summary> Click to Expand a more involved infinite recursion error </summary>

We'll separate the logic for this example, this will be the `default.nix` this
is where having `lib` defined in your inline modules is helpful because you can
just delete the section and paste it into your `modules.nix`:

```nix
# default.nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ./module.nix
    ];
  }
```

And in the `module.nix`:

```nix
# module.nix
{ lib, pkgs, ...}: {
  options.etc = lib.mkOption {
    type = lib.types.attrsOf lib.types.path;
    default = { };
    description = ''
      Specifies which paths are is /etc/
    '';
  };

  config._module.args.pkgs = import <nixpkgs> {
    config = {};
    overlays = [];
  };
  config.etc.foo = pkgs.writeText "foo" ''
    foo configuration
  '';
}
```

- If you evaluate this with the following you will get an infinite recursion
  error.

```bash
nix-instantiate --eval --strict -A config.etc
```

- This happens because `--strict` evaluates the `etc`, then it goes into the
  `attrsOf`, and the `path`

```bash
nix repl
nix-repl> :l <nixpkgs>
nix-repl> hello.out.out.out
```

In this example:

- `:l <nixpkgs>` loads the Nixpkgs library into the repl environment, making its
  definitions available.

- `hello` refers to the `hello` package definition within Nixpkgs. Packages in
  Nixpkgs are defined as _derivations_.

- `.out` is a common attribute name for the _main output_ of a derivation (e.g.,
  the installed package). Some packages, especially those with complex build
  processes or multiple outputs, might have nested output attributes. In the
  case of `hello`, accessing `.out.out.out` ultimately leads us to the
  _derivation_ itself.

The key takeaway here is that when you evaluate a package in the `nix repl`,
you're often interacting with its derivation or one of its output paths in the
Nix store. The `«derivation ...»` indicates that `hello.out.out.out` evaluates
to a derivation – the blueprint for building the `hello` package. This is in
contrast to `--eval --strict`, which tries to fully evaluate values, potentially
leading to infinite recursion if it encounters a derivation that refers back to
itself indirectly during attribute evaluation.

**Output:**

```bash
«derivation /nix/store/b1vcpm321dwbwx6wj4n13l35f4y2wrfv-hello-2.12.1.drv»
```

- So it recurses through the entire thing and tries to evaluate its string.

So we want to change the command from `--eval --strict` which is only based on
evaluation to at least `nix-instantiate` which is based on derivations:

```bash
nix-instantiate -A config.etc
```

**Output:**

```bash
warning: you did not specify '--add-root'; the result might be removed by the garbage collector
/nix/store/abyfp1rxk73p0n5kfilv7pawxwvc7hsg-foo.drv
```

- We don't really have a derivation yet for example:

```nix
# module.nix
{
  lib,
  pkgs,
  ...
}: {
  options.etc = lib.mkOption {
    type = lib.types.attrsOf (lib.types.attrsOf lib.types.path);
    default = {};
    description = ''
      Specifies which paths are in /etc/
    '';
  };

  config._module.args.pkgs = import <nixpkgs> {
    config = {};
    overlays = [];
  };
  config.etc.foo.bar = pkgs.writeText "foo" ''
    foo configuration
  '';
}
```

Try to evaluate the above command with `nix-instantiate -A config.etc` and Nix
doesn't even try to build it. With nested `attrsOf`

```bash
nix repl -f default.nix
nix-repl> config.etc
{
  foo = { ... };
}
nix-repl> config.etc.foo
{
  bar = «derivation /nix/store/abyfp1rxk73p0n5kfilv7pawxwvc7hsg-foo.drv»;
}
```

- So `config.foo` is an attribute set and `config.etc.foo` is also an attribute
  set but it's not a derivation by itself. So `nix-instantiate` does this one
  level of recursion here and it would have built `foo` value if it were a
  derivation.

</details>

### Example 5

<details>
<summary> Click to Expand Example 5 </summary>

We'll use the same `module.nix` and `default.nix` from the previous example.

Building More Complex Configurations with Modules In this next example, we'll
focus on a common task in system configuration: managing files within the
`/etc/` directory. We'll define a module that allows us to specify the content
of arbitrary files in `/etc/` and then use a special Nix function to combine
these individual file definitions into a single, manageable entity.

We'll introduce a new option, `options.etc`, which will allow us to define the
content of files within `/etc/`. Then, we'll use `pkgs.linkFarm` to create a
derivation that represents the entire `/etc/` directory as a collection of
symbolic links pointing to the individual file contents we've defined. This
demonstrates how modules can abstract away the details of creating complex
system configurations, providing a declarative and reproducible way to manage
even fundamental aspects of the operating system.

Let's show how we can use Nix modules to declaratively manage the `/etc/`
directory

```nix
# default.nix
let
  lib = import <nixpkgs/lib>;
in
  lib.evalModules {
    modules = [
      ./module.nix
    ];
  }

```

```nix
# module.nix
{
  lib,
  pkgs,
  config,
  ...
}: {
  options.etc = lib.mkOption {
    type = lib.types.attrsOf (lib.types.attrsOf lib.types.path);
    default = {};
    description = ''
      Specifies which paths are in /etc/
    '';
  };
  options.etcCombined = lib.mkOption {
    type = lib.types.package;
    default =
      pkgs.linkFarm "etc"
      (lib.mapAttrsToList (name: value: {
        name = name;
        path = value;
      }) config.etc);
  };

  config._module.args.pkgs = import <nixpkgs> {
    config = {};
    overlays = [];
  };
  config.etc.foo = pkgs.writeText "foo" ''
    foo configuration
  '';
  config.etc.bar = pkgs.writeText "bar" ''
    bar configuration
  '';
}

```

Run it with:

```bash
nix-instantiate -A config.etcCombined
```

**Output**:

```bash
/nix/store/3da61nmfk546qn2zpxsm57mq6vz6fjx8-etc.drv
```

- So we can see that it will instantiate, lets see if it will build:

```bash
nix-build -A config.etcCombined
```

**Output**:

```bash
these 3 derivations will be built:
/nix/store/41yfxq4af1vrs0rrgfk5gc36kmjc7270-bar.drv
/nix/store/abyfp1rxk73p0n5kfilv7pawxwvc7hsg-foo.drv
/nix/store/3da61nmfk546qn2zpxsm57mq6vz6fjx8-etc.drv
building '/nix/store/41yfxq4af1vrs0rrgfk5gc36kmjc7270-bar.drv'...
building '/nix/store/abyfp1rxk73p0n5kfilv7pawxwvc7hsg-foo.drv'...
building '/nix/store/3da61nmfk546qn2zpxsm57mq6vz6fjx8-etc.drv'...
/nix/store/ca3wyk5m3qhy8n1nbn0181m29qvp1klp-etc
```

```bash
nix-build -A config.etcCombined && ls result/ -laa
```

**Output**:

```bash
/nix/store/ca3wyk5m3qhy8n1nbn0181m29qvp1klp-etc
dr-xr-xr-x - root 31 Dec  1969  .
drwxrwxr-t - root 16 May 15:13  ..
lrwxrwxrwx - root 31 Dec  1969  bar -> /nix/store/1fsjyc2hmilab1qw6jfkf6cb767kz858-bar
lrwxrwxrwx - root 31 Dec  1969  foo -> /nix/store/wai5dycp0zx1lxg0rhpdxnydhiadpk05-foo
```

- We can see that `foo` and `bar` link to different derivations

- When trying to figure out which `default` to use for `etcCombined` infinisil
  went to the Nixpkgs Reference Manual. Make sure to go to the correct version.
  - [24.11pre-git](https://nixos.org/manual/nixpkgs/stable/)

  - [25.05pre-git](https://nixos.org/manual/nixpkgs/unstable/) (i.e. unstable)

  - Once at the website press `Ctrl+f` and type `symlinkjoin` and hit enter.

Or in your local copy of Nixpkgs you could go to
`nixpkgs/pkgs/build-support/ trivial-builders/default.nix`. Then use your
editors search feature, with nvim and helix you press `/symlinkjoin` or
`/linkFarm` hit enter then press `n` to cycle to the next match. It will bring
you to comments and up to date information.

```bash
# linkFarm "myexample" [ { name = "hello-test"; path = pkgs.hello; }
# { name = "foobar"; path = pkgs.stack; } ]
```

</details>

### Tests

<details>
<summary> Click to Expand Test Example </summary>

- How to create a Derivation with `passthru.tests` outside of Nixpkgs and then
  run tests available to your package set?

```bash
mkdir passthru-tests && cd passthru-tests
```

Create a `default.nix` with the following:

```nix
# default.nix
let
  pkgs = import <nixpkgs> {};

  package = pkgs.runCommand "foo" {
    passthru.tests.simple = pkgs.runCommand "foo-test" {} ''
      if [[ "$(cat ${package})" != "foo" ]]; then
        echo "Result is not foo"
        exit 1
      fi
      touch $out
  '';
  } ''
    echo foo > $out
  '';
in
package
```

See if it will build:

```bash
nix-build
```

Try running the test:

```bash
nix-build -A passthru.tests
```

```bash
this derivation will be built:
/nix/store/pqpqq9x1wnsabzbsb52z4g4y4zy6p7yx-foo-test.drv
building '/nix/store/pqpqq9x1wnsabzbsb52z4g4y4zy6p7yx-foo-test.drv'...
/nix/store/7bbw2ban0mgkh4d59yz3cnai4aavwvb6-foo-test
```

### Test 2

- `passthru.tests` is the convention for defining tests associated with a
  derivation. The attributes in `passthru` are preserved and accessible after
  the derivation is built.

```nix
let
  pkgs = import <nixpkgs> {};

  package =
    pkgs.runCommand "foo" {
      passthru.tests.simple = pkgs.runCommand "foo-test" {} ''
        if [[ "$(cat ${package})" != "foo" ]]; then
          echo "Result is not foo"
          exit 1
        fi
        touch $out
      '';

      passthru.tests.version = pkgs.testers.testVersion {
         package = package;
         version = "1.2";
     };

      # pkgs.writeShellApplication
      script = ''
        #!${pkgs.runtimeShell}
        echo "1.2"
      '';
      passAsFiles = [ "script" ];

    } ''
      cp "$scriptPath" "$out"
    '';
in
  package
```

Try to build it:

```bash
nix-build -A passthru.tests
```

- `testers.testVersion` checks if an executable outputs a specific version
  string.

- `nix-build -A passthru.tests` specifically targets the derivations defined
  within the tests attribute of the main derivation.

```bash
these 3 derivations will be built:
  /nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv
  /nix/store/s4iawjy5zpv89dbkc3zz7z3ngz4jq2cv-foo-test.drv
  /nix/store/z3gi4pb8jn2h9rvk4dhba85fiphp5g4z-foo-test-version.drv
building '/nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv'...
cp: cannot stat '': No such file or directory
error: builder for '/nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv'
 failed with exit code 1;
     last 1 log lines:
     > cp: cannot stat '': No such file or directory
     For full logs, run:
       nix log /nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv
error: 1 dependencies of derivation '/nix/store/z3gi4pb8jn2h9rvk4dhba85fiphp5g4z
-foo-test-version.drv' failed to build
error: build of '/nix/store/s4iawjy5zpv89dbkc3zz7z3ngz4jq2cv-foo-test.drv',
 '/nix/store/z3gi4pb8jn2h9rvk4dhba85fiphp5g4z-foo-test-version.drv' failed
```

Run `nix-build` with no arguments:

```bash
nix-build
```

```bash
nix derivation show /nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv | jq '.[].env'
```

**Output**:

```json
{
  "__structuredAttrs": "",
  "buildCommand": "cp \"$scriptPath\" \"$out\"\n",
  "buildInputs": "",
  "builder": "/nix/store/xg75pc4yyfd5n2fimhb98ps910q5lm5n-bash-5.2p37/bin/bash",
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
  "name": "foo",
  "nativeBuildInputs": "",
  "out": "/nix/store/9mcrnddb6lf1md14v4lj6s089i99l5k7-foo",
  "outputs": "out",
  "passAsFile": "buildCommand",
  "passAsFiles": "script",
  "patches": "",
  "propagatedBuildInputs": "",
  "propagatedNativeBuildInputs": "",
  "script": "#!/nix/store/xg75pc4yyfd5n2fimhb98ps910q5lm5n-bash-5.2p37/bin/bash\necho \"1.2\"\n",
  "stdenv": "/nix/store/lgydi1gl5wqcw6k4gyjbaxx7b40zxrsp-stdenv-linux",
  "strictDeps": "",
  "system": "x86_64-linux"
}
```

```bash
nix derivation show /nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv | jq
 '.[].env.buildCommand'
```

**Output:**

```bash
"cp \"$scriptPath\" \"$out\"\n"
```

- raw mode below

```bash
nix derivation show /nix/store/lyz86bd78p7f3yjy1qky6annmggymcwd-foo.drv | jq
 '.[].env.buildCommand' -r
```

**Output**:

```bash
cp "$scriptPath" "$out"
```

- It turns out the correct command was `passAsFile` not `passAsFiles` but that
  change wasn't enough to fix it. `passAsFiles` expects a list of files, not a
  single file path. Running `nix-build -A passthru.tests` failed saying
  `> foo --version returned a non-zero exit code.`

```nix
let
  pkgs = import <nixpkgs> {};

  package =
    pkgs.runCommand "foo" {
      #passthru.tests.simple = pkgs.runCommand "foo-test" {} ''
      #  if [[ "$(cat ${package})" != "foo" ]]; then
      #    echo "Result is not foo"
      #    exit 1
      #  fi
      #  touch $out
      #'';

      passthru.tests.version = pkgs.testers.testVersion {
        package = package;
        version = "1.2";
      };

      # pkgs.writeShellApplication
      script = ''
        #!${pkgs.runtimeShell}
        echo "1.2"
      '';
      passAsFile = ["script"];
    } ''
      mkdir -p "$out/bin"
      cp "$scriptPath" "$out/bin/foo"
      chmod +x "$out/bin/foo"
    '';
in
  package
```

Build it:

```bash
nix-build -A passthru.tests
```

**Output:**

```bash
these 2 derivations will be built:
  /nix/store/lqrlcd64dmpzkggcfzlnsnwjd339czd3-foo.drv
  /nix/store/c3kw4xbdlrig08jrdm5wis1dmv2gnqsd-foo-test-version.drv
building '/nix/store/lqrlcd64dmpzkggcfzlnsnwjd339czd3-foo.drv'...
building '/nix/store/c3kw4xbdlrig08jrdm5wis1dmv2gnqsd-foo-test-version.drv'...
1.2
/nix/store/zsbk5zawak68ailvkwi2gad2bqbqmdz9-foo-test-version
```

</details>

### Key Takeaways for Debugging NixOS Modules

- **`nix-instantiate` is Your Friend:** Use `nix-instantiate` to evaluate your
  NixOS modules and pinpoint errors.

- **Unlock Details with `--show-trace`:** When errors occur, always append
  `--show-trace` to get a comprehensive stack trace, revealing the origin of the
  problem. Remember that in newer Nix versions, the most relevant parts of the
  trace are often at the bottom.

- **Understand Option Types:** Nix option types (`raw`, `anything`,
  `string`/`str`, `lines`, `attrsOf`) are not just about data types; they also
  dictate how values are merged and processed within the module system.

- **Be Mindful of `mkOptionDefault`:** While useful in specific scenarios,
  `mkOptionDefault` sets a lower priority default. For standard defaults that
  can be overridden by user configuration, define them directly within the
  `config` attribute using `lib.mkDefault`.

- **Use `builtins.addErrorContext`:** Enhance your custom error messages by
  providing specific context relevant to your module's logic using
  `builtins.addErrorContext`.

- **Derivations vs. Evaluation:** Be aware of the difference between evaluating
  expressions (`--eval --strict`) and instantiating derivations
  (`nix-instantiate`). Strict evaluation can trigger infinite recursion if it
  encounters unevaluated derivations with cyclic dependencies during attribute
  access.

- **Explore with `nix repl`:** The `nix repl` allows you to interactively
  explore Nix expressions and the outputs of derivations, providing insights
  into the structure and values within Nixpkgs.

#### Conclusion

This chapter has equipped you with essential techniques for debugging and
tracing NixOS modules. We've explored how to use `nix-instantiate` and
`--show-trace` to pinpoint errors, how to interpret Nix's often-verbose error
messages, and how to leverage the `nix repl` for interactive exploration.
Understanding option types and the nuances of `mkOptionDefault` is crucial for
writing robust and predictable modules. We've also touched upon the distinction
between evaluation and instantiation, and how that impacts debugging.

While these tools and techniques are invaluable for understanding and
troubleshooting your own Nix configurations, they also become essential when you
want to contribute to or modify the vast collection of packages and modules
within **Nixpkgs** itself. Nixpkgs is where the majority of Nix packages and
NixOS modules reside, and learning how to navigate and contribute to it opens up
a whole new level of control and customization within the Nix ecosystem.

In the next chapter,
[Working with Nixpkgs Locally](https://saylesss88.github.io/Working_with_Nixpkgs_Locally_10.html),
we'll shift our focus to exploring and modifying Nixpkgs. We'll cover how to
clone Nixpkgs, how to make changes to package definitions, and how to test those
changes locally before contributing them back upstream. This chapter will
empower you to not just use existing Nix packages, but also to customize and
extend them to fit your specific needs.
