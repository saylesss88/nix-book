# Chapter 2

## Understanding Nix Functions

<img src="images/nixLogo.png" width="400" height="300">

Functions are the building blocks of Nix, appearing everywhere in Nix
expressions and configurations. Mastering them is essential for writing
effective Nix code and understanding tools like NixOS and Home Manager.
This chapter explores how Nix functions work, focusing on their single-argument
nature, currying, partial application, and their role in modules.

## What are Nix Functions?

A **Nix Function** is a rule that takes an input (called an **argument**) and
produces an output based on that input. Unlike many programming languages, Nix
functions are designed to take exactly one argument at a time. This unique
approach, combined with a technique called currying, allows Nix to simulate
multi-argument functions in a flexible and reusable way.

First I wanted to explain the structure of Nix Functions, and then we will talk
about their "first-class" nature in Nix.

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

- `"Hello, ${personName}!"`, is the **function body** (which uses the placeholder
  to create the greeting).

When you call the function:

```nix
greet "Anonymous"  # Evaluates to "Hello, Anonymous!"
```

- The value `"Anonymous"` is substituted for the `personName` placeholder within
  the function body.

- This structure is the foundation of all Nix functions, whether simple or
  complex.

## Declaring Functions: Single and Simulated "Multiple" Arguments

**Single-Argument Functions**: The Basics

- In Nix, function definitions like `x: x + 1` or
  `personName: "Hello, ${personName}!";` are **anonymous lambda functions**.
  They exist as values until they are assigned to a variable.

The simplest form of a Nix function takes a single argument:

```nix
# This is an anonymous lambda function value:
# x: x + 1
inc = x: x + 1;          # here we assigned our lambda to a variable `inc`
inc 5  # Evaluates to 6
```

- `x` is the argument.

- `x + 1` is the function body.

- This straightforward design makes single-argument functions easy to understand
  and use. But what if you need a function that seems to take multiple arguments?
  That's where **currying** comes in.

**Simulating Multiple Arguments: Currying**

To create functions that appear to take multiple arguments, Nix uses currying.
This involves nesting single-argument functions, where each function takes one
argument and returns another function that takes the next argument, and so on.

```nix
# concat is equivalent to:
# concat = x: (y: x + y);
concat = x: y: x + y;
concat 6 6    # Evaluates to 12
```

Here, `concat` is actually **two nested functions**

1. The **first function** takes `x` and returns another function.

2. The **second function** takes `y` and performs `x + y`

Nix interprets the colons (`:`) as separators for this chain of single-argument
functions.

Here's how it works step by step:

- When you call `concat 6`, the outer function binds `x` to `6` and returns a
  new function: `y: 6 + y`.

- When you call that function with `6` (i.e., `concat 6 6`), it computes `6 + 6`,
  resulting in `12`.

This chaining is why Nix functions are so powerful—it allows you to build
flexible, reusable functions.

**A More Practical Example: Greetings**:

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

1. The outer function takes `prefix` (e.g. `"Hello"`) and returns a function that
   expects `name`.

2. The inner function takes `name` (e.g. `"Alice"`) and combines it with `prefix`
   to produce the final string.

Thanks to **lexical scope** (where inner functions can access variables from
outer functions), the inner function "remembers" the `prefix` value.

**Why Currying Matters**

- You can partially apply arguments and reuse functions.

- The "first-class" aspect of Nix Functions, explained further down.

- It can help break down complex logic into smaller, manageable functions.

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
nix repl
nix-repl> greeting = prefix: name: "${prefix}, ${name}!";
nix-repl> helloGreeting = greeting "Hello";
nix-repl> helloGreeting "Alice"
"Hello, Alice"
```

- `helloGreeting` is now a new function. It has already received the `prefix`
  argument (`"Hello"`), when we provide the second argument we get `"Hello, Alice!"`

**Benefits of Partial Application:**

- **Creating Specialized Functions**: You can create more specific functions
  from general ones by fixing some of their parameters.

- **Adapting to Higher-Order Functions**: Many functions that operate on other
  functions (like `map` and `filter`) expect functions with a certain number of
  arguments. Partial application allows you to adapt existing functions to fit
  these requirements.

## Nix Functions being "first class citizens"

In the context of Nix, the phrase "Nix treats functions as first-class citizens"
means that functions in Nix are treated as values, just like numbers, strings,
or lists. They can be manipulated, passed around, and used in the same flexible
ways as other data types. This concept comes from functional programming and
has specific implications in Nix.

**What It Means in Nix**

1. Functions Can Be Assigned to Variables:

- You can store a function in a variable, just like you would store a number
  or string.

- Example:

```nix
greet = name: "Hello, ${name}!";
Here, greet is a variable that holds a function.
```

2. Functions Can Be Passed as Arguments:

- You can pass a function to another function as an argument, allowing for
  higher-order functions (functions that operate on other functions).

- Example:

```nix
applyTwice = f: x: f (f x);
inc = x: x + 1;
applyTwice inc 5 # Output: 7 (increments 5 twice: 5 → 6 → 7)
```

- Here, applyTwice takes a function `f` (in this case, `inc`) and applies it to
  `x` twice.

3. Functions Can Be Returned from Functions:

- Functions can produce other functions as their output, which is key to
  currying in Nix.

- Example:

```nix
greeting = prefix: name: "${prefix}, ${name}!";
helloGreeting = greeting "Hello";  # Returns a function
helloGreeting "Alice"  # Output: "Hello, Alice!"
```

- The greeting function returns another function when partially applied with
  prefix.

4. Functions Are Values in Expressions:

- Functions can be used anywhere a value is expected, such as in attribute sets or lists.

- Example:

```nix
myFuncs = {
  add = x: y: x + y;
  multiply = x: y: x * y;
};
myFuncs.add 3 4  # Output: 7
```

- Here, functions are stored as values in an attribute set.

- To try this in the `repl` just remove the semi-colon (`;`)

**Why This Matters in Nix**:

- It increases the flexibility of Functions making them very powerful.

- Many NixOS and Home Manager modules are functions, and their first-class
  status means they can be combined, reused, or passed to other parts of the
  configuration system.

- Now that we understand the "first-class" nature of Nix Functions let's see how
  they fit into NixOS and Home Manager modules.

### The Function Nature of NixOS and Home Manager Modules

It's crucial to understand that most NixOS and Home Manager modules are
fundamentally **functions**.

- These module functions typically accept a single argument: an
  **attribute set**.

**Example**:

A NixOS module to enable Thunar with some plugins that I'm actually using right
now:

```nix
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

- The entire module definition is a function that takes one argument (an
  attribute set):
  `{ pkgs, ... }`.

- When this module is included in your configuration, the NixOS module system
  calls this function with a specific attribute set. This attribute set contains
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
