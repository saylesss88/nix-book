---
title: Nix Lang
date: 2025-11-22
author: saylesss88
---

# Nix Language

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

<!-- ![lambda1](../images/lambda1.png) -->

## Nix Expression Language Syntax Overview

The Nix language is designed for conveniently creating and composing
_derivations_ precise descriptions of how contents of files are used to derive
new files. --[Nix Reference Manual](https://nix.dev/manual/nix/2.28/language/)

Nix is often described as “JSON with functions.” It’s a declarative language
where you define outcomes, not step-by-step instructions. Instead of writing
sequential code, you create expressions that describe data structures,
functions, and dependencies. These expressions are evaluated lazily, meaning Nix
computes values only when needed, making it efficient for managing large
systems.

You can plug most of the following into the `nix repl` I'm showing it in a
single code block here for brevity:

```nix
# Comments Look Like This!

# Strings
"This is a string"          # String literal

''
one
two                        # multi-line String
three
''

("foo" + "bar")           # => "foobar"

"foo" != "bar"   # Inequality test  # => true

!false      # => true

("Home dir is ${builtins.getEnv "HOME"}")  # String Interpolation
# => "Home dir is /home/jr"

"3 6 ${builtins.toString 9}"
# => "3 6 9"

"goodbye ${ { d = "world";}.d}"
# => "goodbye world"

# Booleans

(false && true)    # AND         # => false

(true || false)    # OR         # => true

(if 6 < 9 then "yay" else "nay")  # => "yay"

null      # Null Value

679       # Integer

(6 + 7 + 9) # => 22   # Addition

(9 - 3  - 2) # => 4   # Subtraction

(6 / 3)  # => 2       # Division
6.79      # Floating Point

/etc/nixos      # Absolute Path

../modules/nixos/boot.nix    # relative

# Let expressions

(let a = "2"; in                   # Let expressions are a way to create variables
a + a + builtins.toString "4")
# => "224"

(let first = "firstname"; in
"lastname " first)
# => "lastname firstname"

# Lists

[ 1 2 "three" "bar" "baz" ]   # lists are whitespace separated

builtins.elemAt [ 1 2 3 4 5 ] 3
# => 4

builtins.length [ 1 2 3 4 ]
# => 4

# Attrsets

{ first = "Jim"; last = "Bo"; }.last # Attribute selection
# => "Bo"

{ a = 1; b = 3; } // { c = 4; b = 2; }   # Attribute Set merging
# => { a = 1; b = 2; c = 4; }               # Right Side takes precedence

builtins.listToAttrs [ { name = "Jr"; value = "Jr Juniorville"; } {name = "$"; value = "JR"; } { name = "jr"; value = "jr
ville"; }]
# => { "$" = "JR"; Jr = "Jr Juniorville"; jr = "jrville"; }

# Control Flow

if 2 * 2 == 4
then "yes!"
else "no!"
# => "yes!"

assert 2 * 2
== 4; "yes!"
# => "yes!"

with builtins;
head [ 5 6 7 ]
# => 5

# or

builtins.head[ 5 6 7 ]

inherit pkgs     # pkgs = pkgs;
src;             # src = src;
```

### Understanding Laziness

Nix expressions are evaluated lazily, meaning Nix computes values only when
needed. This is a powerful feature that makes Nix efficient for managing large
systems, as it avoids unnecessary computations.

For example, observe how `a` is never evaluated in the following `nix-repl`
session:

```nix
nix-repl> let a = builtins.div 4 0; b = 6; in b
6
```

- Since `a` isn't used in the final result, there's no division by zero error.

### Strings and String Interpolation

**Strings**: Strings are enclosed in double quotes (`"`) or two single quotes
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

[string interpolation](https://nix.dev/manual/nix/2.24/language/string-interpolation).
is a language feature where a string, path, or attribute name can contain an
expressions enclosed in `${ }`. This construct is called an _interpolated
string_, and the expression inside is an _interpolated expression_.

Rather than writing:

```nix
let path = "/usr/local"; in "--prefix=${path}"
```

This evaluates to `"--prefix=/usr/local"`. Interpolated expressions must
evaluate to a string, path, or an attribute set with an `outPath` or
`__toString` attribute.

### Attribute Sets

**Attribute sets** are all over Nix code and deserve their own section, they are
name-value pairs wrapped in curly braces, where the names must be unique:

```nix
{
  string = "hello";
  int = 8;
}
```

Attribute names usually don't need quotes.

You can access attributes using _dot notation_:

```nix
let person = { name = "Alice"; age = 30; }; in person.name
"Alice"
```

You will sometimes see attribute sets with `rec` prepended. This allows access
to attributes within the set:

```nix
rec {
  x = y;
  y = 123;
}.x
```

**Output**: `123`

or

```nix
rec {
  one = 1;
  two = one + 1;
  three = two + 1;
}
```

**Output**:

```nix
 {
  one = 1;
  three = 3;
  two = 2;
 }
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
```

Will crash with an `infinite recursion encountered` error message.

The
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

However, names on the right take precedence, and updates are shallow.

**Example**:

```nix
{ a = { b = 1; }; } // { a = { c = 3; }; }
```

**Output**:

```nix
{ a = { c = 3; }; }
```

Above, key `b` was completely removed, because the whole `a` value was replaced.

**Inheriting Attributes**

- Click to see Output:

```nix
let x = 123; in
{
  inherit x;
  y = 456;
}
```

is equivalent to

```nix
let x = 123; in
{
  x = x;
  y = 456;
}
```

which are both equivalent to

```nix
{
  x = 123;
  y = 456;
}
```

> ❗: This works because `x` is added to the lexical scope by the `let`
> construct.

Now that we understand attribute sets lets move on to functions, a powerful
feature of the Nix language that gives you the ability to reuse and share
logical pieces of code.

### Functions(lambdas):

Functions in Nix help you build reusable components and are the the building
blocks of Nix. In the next chapter we'll go even further into Nix functions and
how to use them but I will touch on them here.

Nix functions have this form:

```nix
pattern: body
```

The following is a function that expects an integer and returns it increased by
1:

```nix
x: x + 1   # lambda function, not bound to a variable
```

The pattern tells us what the argument of the function has to look like, and
binds variables in the body to (parts of) the argument.

```nix
(x: x + 5) 200
205
```

They are all lambdas (i.e. anonymous functions without names) until we assign
them to a variable like the following example.

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
5
```

The body of the function automatically returns the result of the function.
Functions are called by spaces between it and its parameters. No commas are
needed to separate parameters.

The following is a function that expects an attribute set with required
attributes `a` and `b` and concatenates them:

```nix
{ a, b }: a + b
```

**Default Values in Functions**:

Functions in Nix can define **default values** for their arguments. This allows
for more flexible function calls where some arguments are optional.

```nix
{ x, y ? "foo", z ? "bar" }: z + y + x
```

- Specifies a function that only requires an attribute named `x`, but optionally
  accepts `y` and `z`.

**@-patterns in functions**:

An `@-pattern` provides a means of referring to the whole value being matched by
the function's argument pattern, in addition to destructuring it. This is
especially useful when you want to access attributes that are not explicitly
destructured in the pattern:

```nix
args@{ x, y, z, ... }: z + y + x + args.a
# or
{ x, y, z, ... } @ args: z + y + x + args.a
```

- Here, `args` is bound to the argument as _passed_, which is further matched
  against the pattern `{ x, y, z, ... }`. The `@-pattern` makes mainly sense
  with an ellipsis(`...`) as you can access attribute names as `a`, using
  `args.a`, which was given as an additional attribute to the function.

- We will expand on Functions in
  [This Chapter](https://saylesss88.github.io/Understanding_Nix_Functions_2.html)

### If, Let, and With Expressions

Nix is a pure expression language, meaning every construct evaluates to a value
— there are no statements. Because of this, **if expressions** in Nix work
differently than in imperative languages, where conditional logic often relies
on statements (`if`, `elsif`, etc.).

**If expressions in Nix**:

Since everything in Nix is an expression, an `if` expression must always produce
a value:

```nix
nix-repl> a = 6
nix-repl> b = 10
nix-repl> if a > b then "yes" else "no"
"no"
```

Here, `"no"` is the result because `a`(6) is not greater than `b`(10). Notice
that there's no separate conditional statement -- the entire construct evaluates
to a value.

Another example, integrating built-in functions:

```nix
{
  key = if builtins.pathExists ./path then "YES" else "NO!";
}
```

If `./path` exists it will evaluate to the value `"YES"` or else it will
evaluate to `"NO!"`.

Thus, the final result of the expression would be:

```nix
{ key = "YES"; }
# or
{ key = "NO!"; }
```

Since Nix does not have statements, Nix's `if` statements behave more like
[ternary operators](https://en.wikipedia.org/wiki/Ternary_conditional_operator)
(`condition ? value_if_true : value_if_false`) in other languages.

**Let expressions**:

Let expressions in Nix is primarily a mechanism for local variable binding and
scoping. It allows you to define named values that are only accessible within
the `in` block of the `let` expression. This is useful for keeping code clean
and avoiding repitition.

For example:

```nix
let
  a = "foo";
  b = "fighter";
in a + b
"foofighter"
```

Here, `a` and `b` are defined inside the `let` block, and their values are used
in the `in` expression. Since everything in Nix is an expression, `a + b`
evaluates to `"foofighter"`

**Using Let Expressions Inside Attribute Sets**

Let expressions are commonly used when defining attribute sets (Click for
output):

```nix
let
  appName = "nix-app";
  version = "1.0";
in {
  name = appName;
  fullName = appName + "-" + version;
}
~{
~  name = "nix-app";
~  fullName = "nix-app-1.0";
~}
```

This allows you to reuse values within an attribute set, making the code more
modular and preventing duplication.

**Let Expressions in Function Arguments**

You can also use let expressions within function arguments to define
intermediate values before returning an output:

```nix
{ pkgs, lib }:
let
  someVar = "hello";
  otherVar = "world";
in
{ inherit pkgs lib someVar otherVar; }
```

Result:

```nix
{
  pkgs = <value>;
  lib = <value>;
  someVar = "hello";
  otherVar = "world";
}
```

Here, `inherit` brings `pkgs` and `lib` into the resulting attribute set,
alongside the locally defined variables `someVar` and `otherVar`.

**Key Takeaways**:

- Let expressions allow local variable bindings that are only visible inside the
  in block. They also help avoid repitition and improve readability.

- Commonly used inside attribute sets or function arguments.

- Their scope is limited to the expression in which they are declared.

**With expressions**:

A `with` expression in Nix is primarily used to simplify access to attributes
within an attribute set. Instead of repeatedly referring to a long attribute
path, with temporarily brings the attributes into scope, allowing direct access
without prefixing them.

**Basic Example: Reducing Attribute Path Usage**

Consider the following expressions:

```nix
nix-repl> longName = { a = 3; b = 4; }
nix-repl> longName.a + longName.b
7
```

Here, we must explicitly reference `longName.a` and `longName.b`. Using a `with`
expression simplifies this:

```nix
nix-repl> with longName; a + b
7
```

Now, within the scope of the with expression, `a` and `b` are accessible without
prefixing them with `longName`.

**Practical Use Case: Working with `pkgs`**

One of the most common uses of `with` that you'll see is when dealing with
packages from `nixpkgs` is writing the following:

```nix
{ pkgs }:
with pkgs; {
  myPackages = [ vim git neofetch ];
}
```

Instead of writing this:

```nix
{ pkgs }:
{
  myPackages = [ pkgs.vim pkgs.git pkgs.neofetch ];
}
```

> Tip: Overusing `with lib;` or `with pkgs;` can reduce clarity, it may be fine
> for smaller modules where the scope is limited. For larger configurations,
> explicit references (`pkgs.something`) often make dependencies clearer and
> prevent ambiguity.

### Nix Language Quirks

1. `with` gets less priority than `let`. This can be confusing, especially if
   you like to write `with pkgs;`:

```nix
nix-repl> pkgs = { x = 2; }

nix-repl> with pkgs; x
2

nix-repl> with pkgs; let x = 4; in x
4
```

This shows us that the `let` binding overrides the `with` binding.

```nix
let x = 4; in with pkgs; x
4
```

Still returns `4`, but the reasoning is different. The `with` expression doesn't
define new bindings; it simply makes attributes from `pkgs` available as
unqualified names. However, because `let x = 4` is **outside** the `with`, it
already extablished `x = 4`, so when `with pkgs; x` is evaluated inside, `x`
still refers to the **outer** `let` binding, not the one from `pkgs`.

2. Default values aren't bound in `@-patterns`

In the following example, calling a function that binds a default value `"baz"`
to the attribute `b` of an argument using an alias (`@`) pattern, with an empty
attribute set as argument, results in the alias variable inputs being bound to
the original empty attribute set instead of including the default value:

```nix
(inputs@(b ? "baz"): inputs) {}
```

Output:

```nix
{}
```

This happens because the alias `inputs@` binds to the argument as passed, before
the default value for `b` is applied.

The syntax requires curly brackets around the attribute set pattern for
correctness, so the fixed syntax would be:

```nix
(inputs@{b ? "baz"}: inputs) {}
```

However, even with this fix, the inputs alias still refers to the original
argument without defaults applied. So the quirk persists, showing how default
values in `@-patterns` do not propagate into the aliased variable.

3. Destructuring function arguments:

```nix
nix-repl> f = { x ? 2, y ? 4 }: x + y

nix-repl> f { }
6
```

The function `f` takes an attribute set with default values (`x = 2`, `y = 4`)

When called with `{}` (an empty set), it falls back to the default values
(`2 + 4` -> `6`)

Using `@args` to capture the entire input set:

The `@args` syntax allows us to retain access to the full attribute set, even
after destructuring:

```nix
nix-repl> f = { x ? 1, y ? 2, ... }@args: with args; x + y + z

nix-repl> f { z = 3; }
6
```

The `{ x ? 1, y ? 2, ... }` syntax means `x` and `y` have defaults, while `...`
allows additional attributes.

`@args` binds the entire attribute set (`args`) so that we can access `z`, which
wouldn't be destructured by default.

When calling `f { z = 3; }`, we pass an extra attribute (`z = 3`), making
`x + y + z` → `1 + 2 + 3 = 6`.

4. Imports and namespaces

There is a keyword import, but it's equivalent in other languages is eval. It
can be used for namespacing too:

```nix
let
  pkgs = import <nixpkgs> {};
  lib = import <nixpkgs/lib>;
in
  pkgs.runCommand (lib.strings.removePrefix "....
```

consider using `import` here as using `qualified import ...` in Haskell or
`import ...` in Python.

Another way of importing is with `import ...;`, which corresponds to Python
`from ... import *`.

But because of not very great IDE support in Nix, `with import ...;` is
discouraged. Rather use inherit, especially if you are targeting source code for
Nix newcomers:

```nix
let
  lib = import <nixpkgs/lib>;
  inherit (lib.strings)
    removePrefix removeSuffix
  ;
  inherit (lib.lists)
    isList init drop
  ;
in
  removePrefix ...
```

`inherit` has higher priority than `with`, and conflicts with `let`

```nix
nix-repl> let pkgs = { x = 1; }; x = 2; x = 3; inherit (pkgs) x; in x
error: attribute ‘x’ at (string):1:31 already defined at (string):1:24
```

This makes it a sane citizen of Nix lanugage... except it has a twin, called
`{ inherit ...; }`. They DON'T do the same - `let inherit ...` adds
let-bindings, and `{ inherit ...; }` adds attributes to a record.
--<https://nixos.wiki/wiki/Nix_Language_Quirks>

5. Only attribute names can be interpolated, not Nix code:

```nix
nix-repl> let ${"y"} = 4; in y
4

nix-repl> with { ${"y"} = 4; }; y
4

let y = 1; x = ${y}; in x
error: syntax error, unexpected DOLLAR_CURLY
```

**Conclusion**

- `let` bindings introduce new local values and override anything from `with`.

- `with` doesn't create bindings - it only makes attributes available within its
  scope.

- The order matters: If `let x = 4` is outside `with`, then `x = 4` already
  exists before `with` runs, so `with pkgs; x` resolves to `4`, not the value
  from `pkgs`.

#### Resources

<details>
<summary> ✔️ Resources (Click to Expand) </summary>

A few resources to help get you started with the Nix Language, I have actually
grown to love the language. I find it fairly simple but powerful!

- [nix.dev nixlang-basics](https://nix.dev/tutorials/nix-language.html)

- [Nix Language Overview](https://nix.dev/manual/nix/2.24/language/)

- [learn nix in y minutes](https://learnxinyminutes.com/nix/)

- [nix onepager](https://github.com/tazjin/nix-1p)

- [zero-to-nix nix lang](https://zero-to-nix.com/concepts/nix-language/)

- [nix-pills basics of nixlang](https://nixos.org/guides/nix-pills/04-basics-of-language.html)

- [Basics of the Language Pill](https://nixos.org/guides/nix-pills/04-basics-of-language)

</details>

```nix repl
builtins.head[ 5 6 7 ]
```
