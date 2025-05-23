# Intro to Jujutsu (Using jj in an existing Git Repo)

![Jujutsu Logo](../images/jujutsu.png)

- You can use jujutsu (jj) with existing Git repositories with one command.
  `jj git init --colocate` or `jj git init --git-repo /path/to/git_repository`.
  The native repository format for jj is still a work in progress so people
  typically use a `git` repository for backend.

- Unlike `git`, `jj` has no index "staging area". It treats the working copy
  as an actual commit. When you make changes to files, these changes are
  automatically recorded to the working commit. There's no need to explicitly
  stage changes because they are already part of the commit that represents
  your current working state.

  - This means that you don't need to worry about making a change, running
    `git add .`, running `git commit -m "commit message"` because it's
    already done for you. This is handy with flakes by preventing a
    "dirty working tree" and can instantly be rebuilt after making a change.

## Here's an example

Say I have my configuration flake in the `~/flakes/` directory that is an
existing Git repository. To use JJ as the front-end I could do something like:

```bash
cd ~/flakes
jj git init --colocate
jj describe -m "first jj commit"
jj commit
```

**Or** to do this in a directory that isn't already a git repo you can do
something like:

```bash
cargo new hello-world --vcs=none
cd hello-world
jj git init
Initialized repo in "."
```

### JJ and Git Side by Side

Or for example, with Git if you wanted to move to a different branch before
running `nix flake update` to see if it introduced errors before merging
with your main branch, you could do something like:

```bash
git checkout -b update-test

nix flake update

sudo nixos-rebuild test --flake .
```

If you're satisfied you can merge:

```bash
git checkout main
git add . # Stage the change
git commit -m "update"
git branch -D update-test
git merge update-test
sudo nixos-rebuild switch --flake .
```

With JJ a similar workflow could be:

```bash
jj new  # Create a new child commit/start working on a new change
nix flake update
sudo nixos-rebuild test --flake .
jj squash #  equivalent to `git commit -a --amend`
jj describe -m "update" # Similar to git commit -m
jj commit # Only needed if finalizing an explicit commit
sudo nixos-rebuild switch --flake .
```

- With `jj` you're creating a new commit rather than a new branch.

- Amending vs. Squashing: Git's `git commit --amend` updates the last commit.
  `jj squash` combines the current commit with its parent, effectively doing
  the same thing in terms of history.

- Merging: Git's merge command is explicit. In `jj`, the concept is similar,
  but since there's no branch, you're "merging" by moving your working commit
  to include these changes. The `jj squash` here acts like merging the changes
  into the main line of development.

- No need to delete branches: Since there are no branches in `jj`, there's
  no equivalent to `git branch -D` to clean up. Instead commits that are no
  longer needed can be "abandoned" with `jj abandon` if you want to clean up
  your commit graph.

- `jj describe` without a flag just opens `$EDITOR` where you can write your
  commit message save and exit.

- In `git`, we finish a set of changes to our code by committing, but in
  `jj` we start new work by creating a change, and _then_ make changes to
  our code. It's more useful to write an initial description of your intended
  changes, and then refine it as you work, than it is creating a commit message
  after the fact.

- I have heard that jj can struggle with big repositories such as Nixpkgs and
  have noticed some issues here and there when using with NixOS.
  I'm hoping that as the project matures,it gets better on this front.

- This is just the start of what is possible, here are some resources about
  it if you're interested:

### Resources

- [jj_github](https://github.com/jj-vcs/jj)

- [official_tutorial](https://jj-vcs.github.io/jj/latest/tutorial/)

- [jj_init](https://v5.chriskrycho.com/essays/jj-init/) # very good article

- [steves_jj_tutorial](https://steveklabnik.github.io/jujutsu-tutorial/)
