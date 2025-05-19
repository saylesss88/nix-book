# Chapter 2

## Understanding Nix Functions

<img src="images/nixLogo.png" width="400" height="300">

Functions are a fundamental concept in Nix and are prevalent throughout Nix code. Grasping how they work is crucial for understanding and writing Nix expressions.

## The Single-Argument Nature of Nix Functions

A key concept to understand is that in Nix, every function conceptually takes **exactly one argument**. What might appear as multi-argument functions are actually achieved through a technique called **currying**, where a series of nested single-argument functions are used.

## Identifying Function Structure The Colon

The colon (`:`) acts as a clear separator within a function definition:

- **Left of the Colon:** This is the function's **argument**. It's a placeholder name for a value that will be provided when the function is called.
- **Right of the Colon:** This is the **function body**. It's the expression that will be evaluated when the function is invoked.

**Think of function arguments as naming values that aren't known in advance.** These names are placeholders that get filled with specific values when the function is used.

**Example:**

```nix
greet = personName: "Hello, ${personName}!";
```

- Here, `personName` is the **argument** (the placeholder).

- `"Hello, ${personName}!"`, is the **function body** (the expression that
  uses the placeholder).

When you call the function:

```nix
greet "Anonymous"  # Evaluates to "Hello, Anonymous!"
```

The value `"Anonymous"` is substituted for the `personName` placeholder within
the function body.

## Function Declarations Single and "Multiple" Arguments

**Single-Argument Functions**

The simplest form of a Nix function takes a single argument:

```nix
inc = x: x + 1;
inc 5  # Evaluates to 6
```

- `x` is the argument.

- `x + 1` is the function body.

**Simulating Multiple Arguments: Currying**

To create functions that appear to take multiple arguments, Nix uses currying.
This involves nesting single-argument functions, where each function takes one
argument and returns another function that takes the next argument, and so on.

```nix
concat = x: y: x + y;
concat 6 6    # Evaluates to 12
```

Nix interprets the colons as separators for this chain of single-argument
functions.

**Understanding the Chain:**

Consider the `greeting` function:

```nix
greeting = prefix: name: "${prefix}, ${name}!";
```

This is effectively a chain:

1. **Outer Function**: `prefix: (name: "${prefix}, ${name}!")`

- Takes one argument: `prefix`.

- Its body is another function definition: name: `"${prefix}, ${name}!"`.

2 **Inner Function:** `name: "${prefix}, ${name}!"`

- Takes one argument: `name`.

- Its body uses both its own argument (`name`) and the argument from the
  outer function's scope (prefix).

**Step-by-Step Evaluation:**

When you call `greeting "Hello" "Alice"`:

1. `greeting "Hello"`:

- The `greeting` function is called with `"Hello"` as the `prefix`.

- The outer function returns the inner function:
  `name: "Hello, ${name}!"` (where `prefix` is now fixed as `"Hello"`` in its
  scope).

2. `(greeting "Hello") "Alice"`:

- The resulting inner function is then called with `"Alice"` as the `name`.

- The inner function evaluates its body: `"Hello, ${"Alice"}!"`, resulting in
  `"Hello, Alice!"`.

**Key Insight**: Every colon in a function definition separates a single
argument from its function body, even if that body is another function
definition.

**Partial Application: Using Functions Incrementally**

Because of currying, you can apply arguments to a Nix function one at a time.
This is called partial application. When you provide only some of the expected
arguments, you get a new function that "remembers" the provided arguments and
waits for the rest.

**Example:**

Using our `greeting` function again:

```nix
greeting = prefix: name: "${prefix}, ${name}!";
```

If we only provide the prefix:

```nix
helloGreeting = greeting "Hello";
```

- `helloGreeting` is now a new function. It has already received the `prefix`
  argument (`"Hello"`) and is waiting for the `name` argument.

Calling `helloGreeting`:

```nix
helloGreeting "Sally" # Evaluates to "Hello, Sally!"
```

**Benefits of Partial Application:**

- **Creating Specialized Functions**: You can create more specific functions
  from general ones by fixing some of their parameters.

- **Adapting to Higher-Order Functions**: Many functions that operate on other
  functions (like `map` and `filter`) expect functions with a certain number of
  arguments. Partial application allows you to adapt existing functions to fit
  these requirements.

### The Function Nature of NixOS and Home Manager Modules

It's crucial to understand that most NixOS and Home Manager modules are
fundamentally **functions**.

- These module functions typically accept a single argument: an
  **attribute set**.

**Example**:

A simplified Nginx service module:

```nix
{ config, lib, pkgs, ... }: {
services.nginx.enable = true;
services.nginx.package = pkgs.nginx;
services.nginx.settings."http-port" = "8080";
}
```

- The entire module definition is a function that takes one argument:
  `{ config, lib, pkgs, ... }`.

- When this module is included in your configuration, the NixOS module system
  calls this function with a specific attribute set. This attribute set contains
  the current system configuration (`config`), the Nix standard library (`lib`),
  the available packages (`pkgs`), and other relevant information. The module
  then uses these values to define parts of your system.

## Conclusion

Having explored the fundamental nature of functions in Nix, we can now see
this concept applies to more complex areas like NixOS configuration. In the next
chapter, [NixOS Modules Explained](https://saylesss88.github.io/NixOS_Modules_Explained_3.html).
We will learn about NixOS Modules which are themselves functions most of the
time.

### Resources

- [nix.dev Nix Lang Basics](https://nix.dev/tutorials/nix-language.html)

- [nix pills Functions and Imports](https://nixos.org/guides/nix-pills/05-functions-and-imports.html)

- [zero-to-nix Nix Lang](https://zero-to-nix.com/concepts/nix-language/)

- [A tour of Nix "Functions"](https://nixcloud.io/tour/?id=functions%2Fintroduction)

- [learn Nix in y minutes](https://learnxinyminutes.com/nix/)

- [noogle function library](https://noogle.dev/)
