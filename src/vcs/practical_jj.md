---
title: JJ VCS
date: 2026-03-15
author: saylesss88
description: Practical Jujutsu
collection: "blog"
tags: ["vcs", "jj"]
---

# Practical Jujutsu

JJ simplifies keeping a linear history and makes it easy to break down big
changes into smaller atomic changes.

<details>
<summary> Atomic commits & Linear History explained </summary>

1. Atomic Commits

- An atomic commit is a single unit of work that cannot be broken down further
  without losing its meaning.

- One commit should do one thing.

- If you had to "undo" that commit later, would it break unrelated features? If
  "Yes," it's not atomic.

If you find a bug, you can pinpoint the exact 10 lines of code that caused it.
In `jj`, the `split` command is the ultimate tool for "atomizing" a messy
afternoon of coding.

2. Linear History

A linear history is a straight line of commits without "merge bubbles" (those
criss-crossing lines you see in Git logs when people use git merge).

- Every commit has exactly one parent and one child.

- It reads like a story. You can follow the evolution of the project from bottom
  to top without getting lost in a maze of branches.

- `jj` defaults to a rebase-heavy workflow. Instead of "merging" your work and
  creating a mess, you are constantly "sliding" your changes on top of the
  latest work, keeping that line perfectly straight.

- `❯` will indicate a command that I ran, the rest is output.

</details>

Let's learn about `jj` by using it to version control a system Nix Flake.

### The edit workflow

Initialize and colocate the repository:

```bash
❯  mkdir learn-jj

  ~/projects
❯  cd learn-jj

  ~/projects/learn-jj
❯  nix flake new . -t github:nix-community/home-manager#nixos
wrote: "/home/jr/projects/learn-jj/flake.nix"

  ~/projects/learn-jj  ✗
❯  jj git init --colocate
Initialized repo in "."
Hint: Running `git clean -xdf` will remove `.jj/`!

  learn-jj   main [?]
❯  git remote add origin git@github.com:sayls8/learn-jj.git

  learn-jj   main [?]
❯  jj bookmark create main -r @
Done importing changes from the underlying Git repo.
Created 1 bookmarks pointing to l bd3847c0 main | (no description set)

  learn-jj   refs/jj/root [!]
❯  jj bookmark track main --remote=origin
Started tracking 1 remote bookmarks.
```

Let's give it our current change a description:

```bash
❯  jj desc -m "chore: Initialize system flake"
Working copy  (@) now at: l 743b170d main* | chore: Initialize system flake
Parent commit (@-)      : z 00000000 (empty) (no description set)
```

- With this workflow, your working copy is typically at your current change.

If we wanted to push right now we could with `jj git push`, but let's first
learn a bit more about how jj works.

Let's say we're done with the current change and we're ready to make it
immutable and start a new change:

```bash
❯ jj new -m "chore: change username & hostname in flake.nix"
Working copy  (@) now at: p 6524f35a (empty) chore: change username & hostname in flake.nix
Parent commit (@-)      : l 743b170d main* | chore: Initialize system flake
```

- Now our Working copy `@` is at an `(empty)` change with the description
  "chore: change username & hostname in flake.nix".

- As you can see, running `jj new`, does not move the `main` bookmark. This is
  the hardest part to grasp when coming from Git IMO. Let's make some more
  changes to hammer this home.

I've added my hostname and username to the `flake.nix` template, let's make them
a part of the permanent record.

I create a minimal `configuration.nix`, check my status and notice that I forgot
to run `jj new -m "feat: create minimal configuration.nix"`. Let's see how to
recover from this and keep our commits atomic.

```bash
jj split -i
```

- This opens up a diff editor, I'll only press `y` for the changes related to
  username and hostname. After you pass on what you don't want in this change
  and press `y` on what you do want, your $EDITOR will open with your previous
  commit message. Save it and another commit message will open up in $EDITOR,
  this is whatever you didn't press `y` on i.e., the `configuration.nix`
  changes, just give the second set of changes a different description and
  you're all set.

Another cool thing about `jj` is that you can add a description whenever you
want. Running `jj desc -m "add configuration.nix"` doesn't finalize your commit
like it does with Git. So, you can put the description first, last, or in the
middle of a current change with no issue.

Let's see what the `jj split -i` command left us with:

```bash
❯  jj
Working copy changes:
A configuration.nix
Working copy  (@) : m b3cc09db feat: create minimal configuration.nix
Parent commit (@-): p cddde3b4 chore: change username & hostname in flake.nix
```

And our `log`:

```bash
❯  jj log
@  m sayles8@proton.me 2026-03-15 13:43:31 b3cc09db
│  feat: create minimal configuration.nix
○  p sayls8@proton.me 2026-03-15 13:41:19 cddde3b4
│  chore: change username & hostname in flake.nix
○  l sayls8@proton.me 2026-03-15 13:37:29 main* 743b170d
│  chore: Initialize system flake
◆  z root() 00000000
```

As you can see, `main*` is all the way back at change `l`. Let's move our `main`
bookmark to our current Working copy.

```bash
❯  jj bookmark set main -r @
Moved 1 bookmarks to m b3cc09db main* | feat: create minimal configuration.nix
```

```bash
❯  jj log
@  m sayls8@proton.me 2026-03-15 13:43:31 main* b3cc09db
│  feat: create minimal configuration.nix
○  p sayls8@proton.me 2026-03-15 13:41:19 cddde3b4
│  chore: change username & hostname in flake.nix
○  l sayls8@proton.me 2026-03-15 13:37:29 743b170d
│  chore: Initialize system flake
◆  z root() 00000000
```

```bash
❯  jj git push
Changes to push to origin:
  Add bookmark main to b3cc09dba32c
git: Enumerating objects: 9, done.
git: Counting objects: 100% (9/9), done.
git: Delta compression using up to 16 threads
git: Compressing objects: 100% (7/7), done.
git: Writing objects: 100% (9/9), 1.99 KiB | 1019.00 KiB/s, done.
git: Total 9 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (1/1), done.
Warning: The working-copy commit in workspace 'default' became immutable, so a new commit has been created on top of it.
Working copy  (@) now at: u 551e83ad (empty) (no description set)
Parent commit (@-)      : m b3cc09db main | feat: create minimal configuration.nix
```

```bash
❯  jj log
@  u sayls8@proton.me 2026-03-15 13:47:27 551e83ad
│  (empty) (no description set)
◆  m sayls8@proton.me 2026-03-15 13:43:31 main b3cc09db
│  feat: create minimal configuration.nix
```

- Notice `jj log` now shows `main` instead of `main*`, indicating that `main`
  and `origin@main` are in sync!

- Also notice the `◆` next to the `m` change, this indicates that this change is
  now immutable. This is mentioned in the output of `jj git push` above.

I now need to add a minimal `home.nix`, then run `nix flake check` to see if I
forgot anything.

If something isn't being picked up by `jj` try running `jj st` and check again.
Running any `jj` command updates the Working copy.

Since when running `jj git push` `jj` automatically creates a new commit on top
of the last one, the next step is to describe this change.

I ran `nix flake check` and needed to add a `hardware-configuration.nix`, and
`networking.hostId` required by ZFS, if I wanted to be a stickler about atomic
commits I'd run `jj split -i` again but it's fine by me to make 2 small changes
to get the flake to pass the `check`.

## The squash Workflow

The last section left me with:

```bash
❯  jj st
Working copy changes:
M configuration.nix
A flake.lock
A hardware-configuration.nix
A home.nix
Working copy  (@) : u e57a7a39 feat: add minimal home.nix
Parent commit (@-): m b3cc09db main | feat: create minimal configuration.nix
```

Let's push what we have:

```bash
❯  jj bookmark set main -r @
Moved 1 bookmarks to u e57a7a39 main* | feat: add minimal home.nix

  learn-jj   HEAD [!]
❯  jj git push
Changes to push to origin:
  Move forward bookmark main from b3cc09dba32c to e57a7a39957a
git: Enumerating objects: 8, done.
git: Counting objects: 100% (8/8), done.
git: Delta compression using up to 16 threads
git: Compressing objects: 100% (6/6), done.
git: Writing objects: 100% (6/6), 1.98 KiB | 1.98 MiB/s, done.
git: Total 6 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
Warning: The working-copy commit in workspace 'default' became immutable, so a new commit has been created on top of it.
Working copy  (@) now at: y 53e8a3d9 (empty) (no description set)
Parent commit (@-)      : u e57a7a39 main | feat: add minimal home.nix
```

```bash
❯  jj st
The working copy has no changes.
Working copy  (@) : y 53e8a3d9 (empty) (no description set)
Parent commit (@-): u e57a7a39 main | feat: add minimal home.nix
```

Great, just what we need, an empty change! Let's describe what we plan on doing:

```bash
jj desc -m "refactor: restructure flake to multi-host layout in hosts/magic"
```

Now we create a new change on top of this one:

```bash
❯  jj new
Working copy  (@) now at: w 43195106 (empty) (no description set)
Parent commit (@-)      : y c66bc991 (empty) refactor: restructure flake to multi-host layout in hosts/magic
```

Now we make our changes to the descriptionless Working copy and `squash` our
changes into the parent commit.

```bash
mkdir -p hosts/magic
```

```bash
jj st
The working copy has no changes.
Working copy  (@) : w 43195106 (empty) (no description set)
Parent commit (@-): y c66bc991 (empty) refactor: restructure flake to multi-host layout in hosts/magic
```

Ahh, `jj` doesn't pick up empty directories...

```bash
mv configuration.nix home.nix hosts/magic
```

```bash
 jj st
Working copy changes:
R {configuration.nix => hosts/magic/configuration.nix}
R {home.nix => hosts/magic/home.nix}
Working copy  (@) : w 8c49fc64 (no description set)
Parent commit (@-): y c66bc991 (empty) refactor: restructure flake to multi-host layout in hosts/magic
```

- `R` = Renamed. `jj` is pretty clever here. Since I moved the files but their
  contents stayed the same, `jj` detected that I didn't just "delete" one file
  and "add" a new one, I actually moved an object from point A to point B.
  - The fact that `jj` shows `R {home.nix => hosts/magic/home.nix}` means it is
    keeping the history of that file intact. If you were to look at the log for
    `hosts/magic/home.nix` later, `jj` would know to look back into the history
    of the old `home.nix` as well.

I'm happy with the changes so far:

```bash
❯  jj squash
Working copy  (@) now at: k 41deb985 (empty) (no description set)
Parent commit (@-)      : y 2bee669a refactor: restructure flake to multi-host layout in hosts/magic
```

- Notice how change `w` disappeared and `y` is no longer empty? That's because
  we squashed the changes from our Working copy into the parent commit!

### Pushing from the squash workflow

Let's look at what we have:

```bash
❯  jj st
The working copy has no changes.
Working copy  (@) : k 41deb985 (empty) (no description set)
Parent commit (@-): y 2bee669a refactor: restructure flake to multi-host layout in hosts/magic
```

Since the working copy is at an `(empty)` change, it wouldn't make sense to push
it. We have to move our bookmark to the parent commit, and then push!

```bash
❯  jj bookmark set main -r @-
Moved 1 bookmarks to y 2bee669a main* | refactor: restructure flake to multi-host layout in hosts/magic
```

```bash
❯  jj git push
Changes to push to origin:
  Move forward bookmark main from e57a7a39957a to 2bee669ac276
git: Enumerating objects: 5, done.
git: Counting objects: 100% (5/5), done.
git: Delta compression using up to 16 threads
git: Compressing objects: 100% (3/3), done.
git: Writing objects: 100% (4/4), 452 bytes | 452.00 KiB/s, done.
git: Total 4 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
```

This was the biggest Aha moment I had. It makes perfect sense that you wouldn't
want to push a change that changes nothing. Since we squashed the contents of
the Working copy into `@-`, that is where we need `main` to point.

I have an alias:

```nix
  la = [
    "log"
    "-r"
    "all()"
  ];
```

Let's check out our full history so far:

```bash
❯  jj la
@  k sayls8@proton.me 2026-03-15 14:30:43 41deb985
│  (empty) (no description set)
◆  y sayls8@proton.me 2026-03-15 14:30:43 main 2bee669a
│  refactor: restructure flake to multi-host layout in hosts/magic
◆  u sayls8@proton.me 2026-03-15 14:07:28 e57a7a39
│  feat: add minimal home.nix
◆  m sayls8@proton.me 2026-03-15 13:43:31 b3cc09db
│  feat: create minimal configuration.nix
◆  p sayls8@proton.me 2026-03-15 13:41:19 cddde3b4
│  chore: change username & hostname in flake.nix
◆  l sayls8@proton.me 2026-03-15 13:37:29 743b170d
│  chore: Initialize system flake
◆  z root() 00000000
```

- Textbook linear history. Every single commit has exactly one parent, forming a
  single, unbroken chain from the `root()` up to the current working copy.

- The diamonds `◆` show that everything from `y` down is now part of the
  permanent record (pushed to the remote).

In Git, achieving this usually requires `git add`, `git commit --amend`, or an
interactive rebase. In jj, you just worked in the working copy and pushed, the
tool handled the "shaping" of the history for you.
