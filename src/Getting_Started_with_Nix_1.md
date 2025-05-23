# Chapter1

## Getting Started with Nix

![gruv13](images/gruv13.png)

Welcome to the world of Nix, a powerful tool for reproducible and declarative
software management. In this chapter, we’ll explore the basics of the Nix
programming language, a pure, functional, and declarative language that
underpins Nix’s package manager and operating system. By the end, you’ll
understand Nix’s core concepts, syntax, and how to write simple expressions and
derivations.

- ✔️: Will indicate an expandable section, click the little triangle to expand.

- The code blocks have an option to hide code, where I find it reasonable I will
  hide the outputs of the expressions. Click the eye in the right corner of the
  code block next to the copy clipboard.

- **Nix**: is a package manager and a build system that allows you to write
  declarative scripts for reproducible software builds in the **Nix Language**.

- **NixOS** is the natural consequence of using Nix to build Linux
  systems. You can think about NixOS as a bunch of prebaked snippets of
  configuration that you can combine into a running system that does what
  you want. Each of those snippets is called a module. -- xeiaso

The following bulletpoints can help you get started, they are vast resources
that take a while to fully absorb. The documentation isn't necessarily bad it's
just spread out because from my understanding Nix isn't "allowed" to mention
Flakes in it's manual so you have to look elsewhere.

<details>
<summary> ✔️ Nix Ecosystem (Click to Expand) </summary>

- [Nix Core Ecosystem](https://wiki.nixos.org/wiki/Nix_ecosystem), Nix, NixOS,
  Nix Lang, Nixpkgs are all distinctly different; related things which can be
  confusing for beginners this article explains them.

- [nixpkgs](https://github.com/nixos/nixpkgs): Vast package repository

- [How Nix Works](https://nixos.org/guides/how-nix-works/)

- [Nix Reference Manual Data Types](https://nix.dev/manual/nix/2.26/language/types#type-attrs)
  The main Data Types you'll come across in the Nix ecosystem

- [NixOS Wiki](https://wiki.nixos.org/wiki/NixOS_Wiki)

- [nix.dev](https://nix.dev/): Has become the top respected source of information
  in my opinion. There is a lot of great stuff in here, and they actively update
  the information.

</details>

> ❗ If you're new to Nix, think of it as a recipe book for software: you
> describe what you want (declarative), and Nix ensures it’s built the same way
> every time (reproducible).

## Why Learn Nix?

Nix is often described as “JSON with functions.” It’s a declarative language
where you define outcomes, not step-by-step instructions. Instead of writing
sequential code, you create expressions that describe data structures,
functions, and dependencies. These expressions are evaluated lazily, meaning Nix
computes values only when needed, making it efficient for managing large
systems.

Let’s dive into the key characteristics of Nix:

| Concept          | Description                                                   |
| ---------------- | ------------------------------------------------------------- |
| **Pure**         | Functions don't cause side effects.                           |
| **Functional**   | Functions can be passed as arguments and returned as results. |
| **Lazy**         | Not evaluated until needed to complete a computation.         |
| **Declarative**  | Describing a system outcome.                                  |
| **Reproducible** | Operations that are performed twice return same results       |

> ❗ Important: In Nix, everything is an expression, there are no statements.
>
> ❗ Important: Values in Nix are immutable.

## Syntax Basics

![lambda1](images/lambda1.png)

A few resources to help get you started with the Nix Language, I have actually
grown to love the language. I find it fairly simple but powerful!

- [Nix Language Overview](https://nix.dev/manual/nix/2.24/language/)

- [Basics of the Language Pill](https://nixos.org/guides/nix-pills/04-basics-of-language)

- Dashes are allowed as identifiers:

```nix
nix-repl> a-b
error: undefined variable `a-b' at (string):1:1
nix-repl> a - b
error: undefined variable `a' at (string):1:1
~ testing
```

> ❗ Tip `a-b` is parsed as an identifier, not as subtraction.

- **Strings**: Strings are enclosed in double quotes (`"`) or two single quotes
  (`''`).

```nix
nix-repl> "stringDaddy"
"stringDaddy"
nix-repl> ''
  This is a
  multi-line
  string
''
"This is a\nmulti-line\nstring.\n"
```

<details>
<summary> ✔️ String Interpolation (Click to Expand)</summary>

Is a language feature where a string, path, or attribute name can contain
expressions enclosed in `${ }`. This construct is called _interpolated string_,
and the expression inside is an _interpolated expression_.

[string interpolation](https://nix.dev/manual/nix/2.24/language/string-interpolation).

Rather than writing:

```nix
let path = "/usr/local"; in "--prefix=${path}"
```

- This evaluates to `"--prefix=/usr/local"`. Interpolated expressions must
  evaluate to a string, path, or an attribute set with an outPath or
  `__toString` attribute.

</details>

- **Attribute sets** are all over Nix code, they are name-value pairs wrapped in
  curly braces, where the names must be unique:

```nix
{
  string = "hello";
  int = 8;
}
```

- Attribute names usually don't need quotes.

You can access attributes using dot notation:

```nix
let person = { name = "Alice"; age = 30; }; in person.name
"Alice"
```

You will sometimes see attribute sets with `rec` prepended. This allows access
to attributes within the set:

- Click to see the Output:

```nix
rec {
  x = y;
  y = 123;
}.x
~ 123
```

**Output**: `123`

or

```nix
rec {
  one = 1;
  two = one + 1;
  three = two + 1;
}
~ {
~  one = 1;
~  three = 3;
~  two = 2;
~ }
```

```nix
# This would fail:
{
  one = 1;
  two = one + 1;  # Error: undefined variable 'one'
  three = two + 1;
}
```

Recursive sets introduce the danger of _infinite recursion_ For example:

```nix
rec {
  x = y;
  y = x;
}.x
~ error:
~       … while evaluating the attribute 'x'
~         at «string»:2:3:
~            1| rec {
~            2|   x = y;
~             |   ^
~            3|   y = x;
~
~       error: infinite recursion encountered
~       at «string»:2:7:
~            1| rec {
~            2|   x = y;
~             |       ^
~            3|   y = x;
```

- Will crash with an `infinite recursion encountered` error message.

- The
  [attribute set update operator](https://nix.dev/manual/nix/2.24/language/operators.html#update)
  merges two attribute sets.

**Example**:

```nix
{ a = 1; b = 2; } // { b = 3; c = 4; }
```

**Output**:

```nix
{ a = 1; b = 3; c = 4; }
```

- However, names on the right take precedence, and updates are shallow.

**Example**:

```nix
{ a = { b = 1; }; } // { a = { c = 3; }; }
```

**Output**:

```nix
{ a = { c = 3; }; }
```

- Above, key `b` was completely removed, because the whole `a` value was
  replaced.

**Inheriting Attributes**

- Click to see Output:

```nix
let x = 123; in
{
  inherit x;
  y = 456;
}
~{
~  x = 123;
~  y = 456;
~}
```

is equivalent to

```nix
let x = 123; in
{
  x = x;
  y = 456;
}
~{
~  x = 123;
~  y = 456;
~}
```

> ❗: This works because `x` is added to the lexical scope by the `let`
> construct.

- `inherit` is commonly used to pick specific variables from the function's
  arguments, like in:

```nix
{ pkgs, lib }: ...
let someVar = ...; in { inherit pkgs lib someVar; ... }
```

- This shows another common use case beyond just `let` bindings.

## Control Flow with Expressions

**If expressions**:

- Click to see the Output:

```nix
nix-repl> a = 6
nix-repl> b = 10
nix-repl> if a > b then "yes" else "no"
~ "no"
```

**Let expressions**:

- Click to see the Output:

```nix
let
  a = "foo";
  b = "fighter";
in a + b
~ "foofighter"
```

**With expressions**:

```nix
nix-repl> longName = { a = 3; b = 4; }
nix-repl> longName.a + longName.b
7
nix-repl> with longName; a + b
7
```

**Laziness**:

- Nix evaluates expressions only when needed. This is a great feature when
  working with packages.

```nix
nix-repl> let a = builtins.div 4 0; b = 6; in b
6
```

- Since `a` isn't needed, there's no error about division by zero, because the
  expression is not in need to be evaluated. That's why we can have all the
  packages defined on demand, yet have acces to specific packages very quickly.
  Some of these examples came from the Nix pill series.

**Default Values**:

```nix
{ x, y ? "foo", z ? "bar" }: z + y + x
~ «lambda @ «string»:1:1»
```

- Specifies a function that only requires an attribute named `x`, but optionally
  accepts `y` and `z`.

**@-patterns**:

- An `@-pattern` provides a means of referring to the whole value being matched:

```nix
args@{ x, y, z, ... }: z + y + x + args.a
~ «lambda @ «string»:1:1»
# or
{ x, y, z, ... } @ args: z + y + x + args.a
~ «lambda @ «string»:1:1»
```

- Here, `args` is bound to the argument as _passed_, which is further matched
  against the pattern `{ x, y, z, ... }`. The `@-pattern` makes mainly sense
  with an ellipsis(`...`) as you can access attribute names as `a`, using
  `args.a`, which was given as an additional attribute to the function.

## Functions:

Functions are defined using this syntax, where `x` and `y` are attributes passed
into the function:

```nix
{
  my_function = x: y: x + y;
}
```

The code below calls a function called `my_function` with the parameters `2` and
`3`, and assigns its output to the `my_value` field:

```nix
{
  my_value = my_function 2 3;
}
my_value
~ 5
```

- The body of the function automatically returns the result of the function.
  Functions are called by spaces between it and its parameters. No commas are
  needed to separate parameters.

### Derivations

![nix99](images/nix99.png)

<details>
<summary> ✔️ Derivation Overview (Click to Expand) </summary>

- In Nix, the process of managing software starts with **package definitions**.
  These are files written in the Nix language that describe how a particular
  piece of software should be built. These package definitions, when processed
  by Nix, are translated into derivations.

- At its core, a derivation in Nix is a blueprint or a recipe that describes how
  to build a specific software package or any other kind of file or directory.
  It's a declarative specification of:

- **Inputs**: What existing files or other derivations are needed as dependencies.

- **Build Steps**: The commands that need to be executed to produce the desired
  output.

- **Environment**: The specific environment (e.g., build tools, environment
  variables) required for the build process.

- **Outputs**: The resulting files or directories that the derivation produces.

Think of a package definition as the initial instructions, and the derivation as
the detailed, low-level plan that Nix uses to actually perform the build."

Again, a derivation is like a blueprint that describes how to build a specific
software package or any other kind of file or directory.

**Key Characteristics of Derivations:**

- **Declarative**: You describe the desired outcome and the inputs, not the
  exact sequence of imperative steps. Nix figures out the necessary steps based
  on the builder and args.

- **Reproducible**: Given the same inputs and build instructions, a derivation
  will always produce the same output. This is a cornerstone of Nix's
  reproducibility.

- **Tracked by Nix**: Nix keeps track of all derivations and their outputs in
  the Nix store. This allows for efficient management of dependencies and
  ensures that different packages don't interfere with each other.

- **Content-Addressed**: The output of a derivation is stored in the Nix store
  under a unique path that is derived from the hash of all its inputs and build
  instructions. This means that if anything changes in the derivation, the
  output will have a different path.

Here's a simple Nix derivation that creates a file named hello in the Nix store
containing the text "Hello, World!":

</details>

<details>
<summary> ✔️ Hello World Derivation Example (Click to expand):</summary>

```nix
{pkgs ? import <nixpkgs> {}}:
pkgs.stdenv.mkDerivation {
  name = "hello-world";

  dontUnpack = true;

  # No need for src = null; when dontUnpack = true;
  # src = null;

  buildPhase = ''
     # Create a shell script that prints "Hello, World!"
    echo '#!${pkgs.bash}/bin/bash' > hello-output-file # Shebang line
    echo 'echo "Hello, World!"' >> hello-output-file # The command to execute
    chmod +x hello-output-file # Make it executable
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp hello-output-file $out/bin/hello # Copy the file from build directory to $out/bin
  '';

  meta = {
    description = "A simple Hello World program built with Nix";
    homepage = null;
    license = pkgs.lib.licenses.unfree; # Ensure this is pkgs.lib.licenses.unfree
    maintainers = [];
  };
}
```

And a `default.nix` with the following contents:

```nix
{ pkgs ? import <nixpkgs> {} }:

import ./hello.nix { pkgs = pkgs; }
```

- `{ pkgs ? import <nixpkgs> {} }`: This is a function that takes an optional
  argument `pkgs`. We need Nixpkgs to access standard build environments like
  `stdenv`.

- `pkgs.stdenv.mkDerivation { ... }:` This calls the mkDerivation function from
  the standard environment (stdenv). mkDerivation is the most common way to
  define software packages in Nix.

- `name = "hello-world";`: Human-readable name of the derivation

- The rest are the build phases and package metadata.

To use the above derivation, save it as a `.nix` file (e.g. `hello.nix`). Then
build the derivation using,:

```bash
nix-build
this derivation will be built:
  /nix/store/9mc855ijjdy3r6rdvrbs90cg2gf2q160-hello-world.drv
building '/nix/store/9mc855ijjdy3r6rdvrbs90cg2gf2q160-hello-world.drv'...
Running phase: patchPhase
Running phase: updateAutotoolsGnuConfigScriptsPhase
Running phase: configurePhase
no configure script, doing nothing
Running phase: buildPhase
Running phase: installPhase
Running phase: fixupPhase
shrinking RPATHs of ELF executables and libraries in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
checking for references to /build/ in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world...
patching script interpreter paths in /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
stripping (with command strip and flags -S -p) in  /nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world/bin
/nix/store/2ydxh5pd9a6djv7npaqi9rm6gmz2f73b-hello-world
```

- Nix will execute the `buildPhase` and `installPhase`

- After a successful build, the output will be in the Nix store. You can find
  the exact path by looking at the output of the nix build command (it will be
  something like `/nix/store/your-hash-hello-world`).

Run the "installed" program:

```bash
./result/bin/hello
```

- This will execute the `hello` file from the Nix store and print `"Hello,
World!"`.

</details>

### Evaluating Nix Files

Use `nix-instantiate --eval` to evaluate the expression in a Nix file:

```bash
echo 1 + 2 > file.nix
nix-instantiate --eval file.nix
3
```

> **Note:** `--eval` is required to evaluate the file and do nothing else. If
> `--eval` is omitted, `nix-instantiate` expects the expression in the given
> file to evaluate to a derivation.

If you don't specify an argument, `nix-instantiate --eval` will try to read from
`default.nix` in the current directory.

## Conclusion

As we have now seen, this chapter touched on the basic syntax of function
definition and application, including concepts like currying. However, the power
and flexibility of Nix functions extend far beyond what we've covered so far.

In the next chapter,
[Understanding Nix Functions](https://saylesss88.github.io/Understanding_Nix_Functions_2.html)
we will peel back the layers and explore the intricacies of function arguments,
advanced patterns, scope, and how functions play a crucial role in building more
sophisticated Nix expressions and derivations.

Here are some resources that I found helpful when learning the Nix Language.

## Resources

<details>
<summary> ✔️ Resources (Click to Expand)</summary>

- [nix.dev nixlang-basics](https://nix.dev/tutorials/nix-language.html)

- [learn nix in y minutes](https://learnxinyminutes.com/nix/)

- [nix onepager](https://github.com/tazjin/nix-1p)

- [awesome-nix](https://github.com/nix-community/awesome-nix)

- [zero-to-nix nix lang](https://zero-to-nix.com/concepts/nix-language/)

- [nix-pills basics of nixlang](https://nixos.org/guides/nix-pills/04-basics-of-language.html)

</details>
