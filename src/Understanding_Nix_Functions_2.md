---
title: Understanding Nix Functions
date: 2025-11-21
author: saylesss88
---

# Chapter 2

<details>
<summary> ✔️ Table of Contents</summary>

<!-- toc -->

</details>

<!-- <img src="images/nixLogo.png" width="400" height="300"> -->

![trees2](images/trees2.cleaned.png)

## Understanding Nix Functions

**Functions** are the building blocks of Nix, appearing everywhere in Nix
expressions and configurations. Mastering them is essential for writing
effective Nix code and understanding tools like NixOS and Home Manager. This
chapter explores how Nix functions work, focusing on their **single-argument
nature**, **currying**, **partial application**, and their role in **modules**.

## What are Nix Functions?

A **Nix Function** is a rule that takes an input (called an **argument**) and
produces an **output** based on that input. Unlike many programming languages,
Nix functions are designed to take exactly one argument at a time. This unique
approach, combined with a technique called currying, allows Nix to simulate
multi-argument functions in a flexible and reusable way.

## Builtins

<details>
<summary> ✔️ Nix Builtin Functions (Click to Expand)</summary>

The Nix expression evaluator has a bunch of functions and constants built in:

- `toString e`: (Convert the expression `e` to a string)

- `import path`: (Load, parse and return the Nix expression in the file `path`)

- `throw x`: (Throw an error message `x`. Usually stops evaluation)

- `map f list`: (Apply the function `f` to each element in the `list`)

- [Built-in Functions](https://nix.dev/manual/nix/2.18/language/builtins)

- [Nix Operators](https://nix.dev/manual/nix/2.26/language/operators)

</details>

## Lambdas

Nix functions are anonymous (lambdas) (e.g., `x: x + 2`), and technically take a
single parameter. However, that single parameter is very often an attribute set,
allowing you to effectively pass multiple named inputs by destructuring (e.g.,
`{ arg1, arg2 }: arg1 + arg2`).

Type the parameter name, followed by a colon, and finally the body of the
function.

```nix
nix-repl> param: param * 2
<<lambda @ <<string>>:1:1>>

nix-repl> (param: param * 2) 2
4
```

The above example shows that everything in Nix returns a value. When you call a
function directly (without first assigning the function itself to a variable),
the result of that call is immediately evaluated and displayed/used.

In order to make our function reusable and be able to pass different values at
different times we have to assign our function to a variable:

```nix
nix-repl> twoTimes = param: param * 2
```

Now, we can reference our function by it's name and pass our required parameter:

```nix
nix-repl> twoTimes
«lambda @ «string»:1:2»
nix-repl> twoTimes 2
4
nix-repl> twoTimes 4
8
```

We defined a function `param: param * 2` takes one parameter `param`, and
returns `param * 2`. We then assigned this function to the variable `twoTimes`.
Lastly, we called the function with a few different arguments showing it's
reusability.

## Understanding Function Structure: The Role of the Colon

The colon (`:`) acts as a clear separator within a function definition:

- **Left of the Colon:** This is the function's **argument**. It's a placeholder
  name for a value that will be provided when the function is called.

- **Right of the Colon:** This is the **function body**. It's the expression
  that will be evaluated when the function is invoked.

**Think of function arguments as naming values that aren't known in advance.**
These names are placeholders that get filled with specific values when the
function is used.

**Example:**

```nix
greet = personName: "Hello, ${personName}!";
```

- Here, `personName` is the **argument** (the placeholder).

- `"Hello, ${personName}!"`, is the **function body** (which uses the
  placeholder to create the greeting).

When you call the function, (click to see Output):

```nix
greet "Anonymous"
~ "Hello, Anonymous!"
```

- The value `"Anonymous"` is substituted for the `personName` placeholder within
  the function body.

- This structure is the foundation of all Nix functions, whether simple or
  complex.

### Single-Argument Functions: The Basics

The simplest form of a Nix function takes a single argument. In Nix, function
definitions like `x: x + 1` or `personName: "Hello, ${personName}!";` are
**anonymous lambda functions**. They exist as values until they are assigned to
a variable.

- Click to see Output:

```nix
# This is an anonymous lambda function value:
# x: x + 1
inc = x: x + 1;          # here we assigned our lambda to a variable `inc`
inc 5
~ 6
```

- `x` is the argument.

- `x + 1` is the function body.

This straightforward design makes single-argument functions easy to understand
and use. But what if you need a function that seems to take multiple arguments?
That's where **currying** comes in.

### Simulating Multiple Arguments: Currying

To create functions that appear to take multiple arguments, Nix uses currying.
This involves nesting single-argument functions, where each function takes one
argument and returns another function that takes the next argument, and so on.

```nix
nix-repl> multiply = x: (y: x*y)
nix-repl> multiply
«lambda»
nix-repl> multiply 4
«lambda»
nix-repl> (mul 4) 5
20
```

We defined a function that takes the parameter `x`, the body returns another
function. This other function takes a parameter `y` and returns `x*y`.
Therefore, calling `multiply 4` returns a function like: `x: 4*y`. In turn, we
call the returned function with `5`, and get the expected result.

#### Currying example 2

```nix
# concat is equivalent to:
# concat = x: (y: x + y);
concat = x: y: x + y;
concat 6 6    # Evaluates to 12
12
```

Here, `concat` is actually **two nested functions**

1. The **first function** takes `x` and returns another function.

2. The **second function** takes `y` and performs `x + y`

Nix interprets the colons (`:`) as separators for this chain of single-argument
functions.

Here's how it works step by step:

- When you call `concat 6`, the outer function binds `x` to `6` and returns a
  new function: `y: 6 + y`.

- When you call that function with `6` (i.e., `concat 6 6`), it computes
  `6 + 6`, resulting in `12`.

This chaining is why Nix functions are so powerful—it allows you to build
flexible, reusable functions.

Currying is a powerful feature in Nix that enables you to partially apply
arguments to functions, leading to increased reusability. This behavior is a
direct consequence of Nix functions being "first-class citizens" (a concept
we'll delve into later), and it proves invaluable for decomposing intricate
logic into a series of smaller, more focused functions.

**Key Insight**: Every colon in a function definition separates a **single
argument** from its **function body**, even if that body is another function
definition.

#### Greeting Example

Let's explore currying with a more relatable example in the `nix repl`:

```nix
nix repl
nix-repl> greeting = prefix: name: "${prefix}, ${name}!";

nix-repl> greeting "Hello"
<<lambda @ <<string>>:1:10>> # partial application returns a lambda

nix-repl> greeting "Hello" "Alice"
"Hello, Alice!"         # providing both arguments returns the expected result
```

This function is a chain of two single-argument functions:

1. The outer function takes `prefix` (e.g. `"Hello"`) and returns a function
   that expects `name`.

2. The inner function takes `name` (e.g. `"Alice"`) and combines it with
   `prefix` to produce the final string.

Thanks to **lexical scope** (where inner functions can access variables from
outer functions), the inner function "remembers" the `prefix` value.

#### Partial Application: Using Functions Incrementally

Because of **currying**, you can apply arguments to a Nix function one at a
time. This is called _partial application_. When you provide only some of the
expected arguments, you get a new function that "remembers" the provided
arguments and waits for the rest.

**Example:**

Using our `greeting` function again:

```nix
nix repl
nix-repl> greeting = prefix: name: "${prefix}, ${name}!";
nix-repl> helloGreeting = greeting "Hello";
nix-repl> helloGreeting "Alice"
"Hello, Alice"
```

- `helloGreeting` is now a new function. It has already received the `prefix`
  argument (`"Hello"`), when we provide the second argument we get
  `"Hello, Alice!"`

**Benefits of Partial Application:**

Partial application provides significant benefits by enabling you to derive
specialized functions from more general ones through the process of fixing
certain parameters. Additionally, it serves as a powerful tool for adapting
existing functions to fit the precise argument requirements of higher-order
functions like `map` and `filter`.

#### Nix Functions being "first class citizens"

In the context of Nix, the phrase "Nix treats functions as first-class citizens"
means that functions in Nix are treated as values, just like numbers, strings,
or lists. They can be manipulated, passed around, and used in the same flexible
ways as other data types. This concept comes from functional programming and has
specific implications in Nix.

**What It Means in Nix**

1. Functions Can Be **Assigned to Variables**:

- You can store a function in a variable, just like you would store a number or
  string.

- Example:

```nix
greet = name: "Hello, ${name}!";
```

- Here, greet is a variable that holds a function.

2. Functions Can Be **Passed as Arguments**:

- You can pass a function to another function as an argument, allowing for
  higher-order functions (functions that operate on other functions).

- Example:

```nix
applyTwice = f: x: f (f x);
inc = x: x + 1;
applyTwice inc 5 # Output: 7 (increments 5 twice: 5 → 6 → 7)
~ 7
```

- Here, applyTwice takes a function `f` (in this case, `inc`) and applies it to
  `x` twice.

3. Functions Can Be **Returned from Functions**:

- Functions can produce other functions as their output, which is key to
  currying in Nix.

- Example:

```nix
greeting = prefix: name: "${prefix}, ${name}!";
helloGreeting = greeting "Hello";  # Returns a function
helloGreeting "Alice"  # Output: "Hello, Alice!"
~ "Hello, Alice!"
```

- The greeting function returns another function when partially applied with
  prefix.

4. Functions **Are Values in Expressions**:

- Functions can be used anywhere a value is expected, such as in attribute sets
  or lists.

- Example:

```nix
myFuncs = {
  add = x: y: x + y;
  multiply = x: y: x * y;
};
myFuncs.add 3 4  # Output: 7
~ 7
```

- Here, functions are stored as values in an attribute set.

- To try this in the `repl` just remove the semi-colon (`;`)

**Why This Matters in Nix**:

This functional approach is fundamental to Nix's unique build system. In Nix,
**package builds (called derivations)** are essentially functions. They take
specific **inputs** (source code, dependencies, build scripts) and
deterministically produce **outputs** (a built package).

This design ensures **atomicity**: if a build does not succeed completely and
perfectly, it produces no output at all. This prevents situations common in
other package managers where partial updates or corrupted builds can leave your
system in an inconsistent or broken state.

Many NixOS and Home Manager modules are functions, and their first-class status
means they can be combined, reused, or passed to other parts of the
configuration system.

Now that we understand the "first-class" nature of Nix Functions let's see how
they fit into NixOS and Home Manager modules.

#### The Function Nature of NixOS and Home Manager Modules

It's crucial to understand that most NixOS and Home Manager modules are
fundamentally **functions**.

These module functions typically accept a single argument: **an attribute set**
(remember this, it's important to understand).

**Example**:

A practical NixOS module example for Thunar with plugins:

```nix
# thunar.nix
{pkgs, ...}: {
  programs = {
    thunar = {
      enable = true;
      plugins = with pkgs.xfce; [
        thunar-archive-plugin
        thunar-volman
      ];
    };
  };
}
```

- To use this module I would need to import it into my `configuration.nix` or
  equivalent, shown here for completeness.

```nix
# configuration.nix
# ... snip ...
imports = [ ../nixos/thunar.nix ];
# ... snip ...
```

- This is actually a pretty good example of `with` making it a bit harder to
  reason where the plugins are from. You might instinctively try to trace a path
  like `programs.thunar.plugins.pkgs.xfce` because you saw `pkgs.xfce` in the
  `with` statement. But that's now how `with` works. The `pkgs.xfce` path exists
  _outside_ the `plugins` list, defining the source of the items, not their
  nested structure within the list.

- To follow best practices you could write the above plugins section as:

```nix
plugins = [
  pkgs.xfce.thunar-archive-plugin
  pkgs.xfce.thunar-volman
];
```

- Now it's clear that each plugin comes directly from `pkgs` and each will
  resolve to a derivation.
  - To be clear either way is fine, especially in such a small self contained
    module. If it were in a single file `configuration.nix` it would be a bit
    more confusing to trace. Explicitness is your friend with Nix and
    maintaining reproducability. `with` isn't always bad but should be avoided
    at the top of a file for example to bring `nixpkgs` into scope, use `let`
    instead.

The entire module definition is a function that takes one argument (an attribute
set):`{ pkgs, ... }`. When this module is included in your configuration, the
NixOS module system calls this function with a specific attribute set. This
attribute set contains the available packages (`pkgs`), and other relevant
information. The module then uses these values to define parts of your system.

### Understanding passing and getting back arguments

For this example we will build the Hello derivation from the Nix Pills series.

Create an `autotools.nix` with the following contents:

```nix
pkgs: attrs: let
  defaultAttrs = {
    builder = "${pkgs.bash}/bin/bash";
    args = [./builder.sh];
    baseInputs = with pkgs; [
      gnutar
      gzip
      gnumake
      gcc
      coreutils
      gawk
      gnused
      gnugrep
      binutils.bintools
    ];
    buildInputs = [];
    system = builtins.currentSystem;
  };
in
  derivation (defaultAttrs // attrs)
```

Let's create the hello derivation:

```nix
let
  pkgs = import <nixpkgs> {};
  mkDerivation = import ./autotools.nix pkgs;
in
  mkDerivation {
    name = "hello";
    src = ./hello-2.12.1.tar.gz;
  }
```

- You can get the tarball
  [here](https://ftp.gnu.org/gnu/hello/hello-2.12.1.tar.gz), place it in the
  same directory as `autotools.nix`

And finally the `builder.sh` that `autotools.nix` declares for the `args`
attribute:

```bash
#!/bin/bash
set -e
unset PATH
for p in $buildInputs $baseInputs; do
    export PATH=$p/bin${PATH:+:}$PATH
done

tar -xf $src

for d in *; do
    if [ -d "$d" ]; then
        cd "$d"
        break
    fi
done

./configure --prefix=$out
make
make install
```

When you write:

```nix
mkDerivation = import ./autotools.nix pkgs;
```

- `import ./autotools.nix`: This evaluates the `autotools.nix` file. Because it
  starts with `pkgs: attrs: ...`, it means that `autotools.nix` evaluates to a
  function that expects one argument named `pkgs`.

- `... pkgs`: We are immediately calling that function (the one returned by
  `import ./autotools.nix`) and passing it our `pkgs` variable (which is the
  result of `import <nixpkgs> {}`).

**This illustrates the concept of Currying in Nix**:

The function defined in `autotools.nix` (`pkgs: attrs: ...`) is a curried
function. It's a function that, when given its first argument (`pkgs`), returns
another function (which then expects `attrs`).

The result of import `./autotools.nix pkgs` is that second, inner function:
`attrs: derivation (defaultAttrs // attrs)`. This inner function is then bound
to the `mkDerivation` variable, making it ready to be called with just the
specific attributes for your package (like `name` and `src`).

**Understanding the `attrs` Argument**

Now let's focus on the second argument of our `autotools.nix` function: `attrs`.

Recall the full function signature in `autotools.nix`:

```nix
pkgs: attrs: let
  # ... defaultAttrs definition ...
in
  derivation (defaultAttrs // attrs)
```

1. What `attrs` Represents:

- Once `autotools.nix` has received its `pkgs` argument (and returned the inner
  function), this inner function is waiting for its final argument, which we
  call `attrs`.

- `attrs` is simply an attribute set (a key-value map in Nix). It's designed to
  receive all the specific properties of the individual package you want to
  build using this helper.

2. How `attrs` is Used:

- Look at the final line of `autotools.nix`:
  `derivation (defaultAttrs // attrs)`.

- The `//` operator in Nix performs an attribute set merge. It takes all
  attributes from `defaultAttrs` and combines them with all attributes from
  `attrs`.

- Crucially, if an attribute exists in both `defaultAttrs` and `attrs`, the
  value from `attrs` (the second operand) takes precedence and overrides the
  default value.

3. Applying attrs in the hello Derivation:

- In the `hello` derivation, we call `mkDerivation` like this:

```nix
        mkDerivation {
          name = "hello";
          src = ./hello-2.12.1.tar.gz;
        }
```

- The attribute set `{ name = "hello"; src = ./hello-2.12.1.tar.gz; }` is what
  gets passed as the `attrs` argument to the `mkDerivation` function (which,
  remember, is the inner function returned by `autotools.nix`).

- When derivation `(defaultAttrs // attrs)` is evaluated for "hello", the `name`
  and `src` provided in the `attrs` set will be merged with all the
  `defaultAttrs` (like `builder`, `args`, `baseInputs`, etc.).

In summary:

- The `pkgs` argument configures the general environment and available tools for
  the builder.

- The `attrs` argument is where you provide the unique details for each specific
  package you intend to build using this `autotools.nix` helper. It allows you
  to specify things like the package's name, source code, version, and any
  custom build flags, while still benefiting from all the sensible defaults
  provided by `autotools.nix`. This separation makes `autotools.nix` a reusable
  and flexible "template" for creating derivations.

#### Conclusion

Having explored the fundamental nature of functions in Nix, we can now see this
concept applies to more complex areas like NixOS configuration and derivations.
In the next chapter,
[NixOS Modules Explained](https://saylesss88.github.io/NixOS_Modules_Explained_3.html).
We will learn about NixOS Modules which are themselves functions most of the
time.

#### Resources

<details>
<summary> ✔️ Resources (Click to Expand) </summary>

- [nix.dev Nix Lang Basics](https://nix.dev/tutorials/nix-language.html)

- [nix pills Functions and Imports](https://nixos.org/guides/nix-pills/05-functions-and-imports.html)

- [zero-to-nix Nix Lang](https://zero-to-nix.com/concepts/nix-language/)

- [A tour of Nix "Functions"](https://nixcloud.io/tour/?id=functions%2Fintroduction)

- [learn Nix in y minutes](https://learnxinyminutes.com/nix/)

- [noogle function library](https://noogle.dev/)

</details>
