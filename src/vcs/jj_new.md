---
title: JJ New
date: 2025-11-22
author: saylesss88
description: JJ New
---

# Version Control with JJ

<details>
<summary> ✔️ Click to Expand Table of Contents</summary>

<!-- toc -->

</details>

![JJ Logo](../images/jujutsu.png)

⚠️ **Important**: Never commit secrets (passwords, API keys, tokens, etc.) in
plain text to your Git repository. If you plan to publish your NixOS
configuration, always use a secrets management tool like sops-nix or agenix to
keep sensitive data safe. See the
[Sops-Nix Guide](https://saylesss88.github.io/installation/enc/sops-nix.html)
for details.

You may want to check
[Steve's Jujutsu Tutorial](https://steveklabnik.github.io/jujutsu-tutorial/), he
does a great job of explaining how to use jj with practical examples. It is
recommended in the official JJ docs as a more up to date intro.

After installing JJ you can run `jj help -k tutorial`, for the official guide in
a pager but like I said Steve's guide is more up to date.

You can find help for nearly every command with the command followed by `--help`
(e.g., `jj git init --help`, `jj git push --help`)

If you're completely unfamiliar with Git,
[Pro Git](https://git-scm.com/book/en/v2) is a searchable document where you can
look up any terms to compare with JJ and gain a base understanding of VCS.
(Version Control Systems).

## Introduction

Jujutsu (jj) is a modern, Git-compatible version control system designed to
simplify and improve the developer experience. It offers a new approach to
distributed version control, focusing on a more intuitive workflow, powerful
undo capabilities, and a branchless model that reduces common pitfalls of Git.

**Key Terms**

- [Working Copy](https://jj-vcs.github.io/jj/latest/working-copy/): The working
  copy is where the current

- `trunk()`: is a revset alias to `main@origin`. It tells JJ that whenever you
  use `trunk()` in a `jj` command (like `jj log -r trunk()`), it should
  interpret that as referring to the latest commit on the `main` branch of your
  `origin` remote (i.e., GitHub, GitLab, etc.). Often, the commits on the
  `trunk()` are immutable in JJ.

## Named Branches (Bookmarks) in JJ

In an existing Git Repository run:

```bash
jj git init --colocate
Done importing changes from the underlying Git repo.
Setting the revset alias `trunk()` to `main@origin`
Initialized repo in "."
```

This creates the following `revset-alias` in your `.jj/repo/config.toml`:

```bash
[revset-aliases]
"trunk()" = "main@origin"
```

You can inspect this with:

```bash
jj config edit --repo
```

This will open your repos `.jj/repo/config.toml`

```bash
jj bookmark create trunk -r main
```

Creating New Work (from `trunk`):

```bash
jj new trunk
```

**Updating your `trunk`**: Your `trunk` bookmark needs to stay updated with the
remote's `main` branch.

When you want to pull in the latest changes from `origin/main` into your local
`trunk`:

```bash
jj git fetch                      # Fetch latest changes from Git remote
jj rebase -d main@origin -s trunk # Rebase your local trunk onto the remote's main
                                # Or simply: jj branch track main@origin trunk
```

**Merging Changes into `trunk`**: When you've finished a feature branch (say,
`my-feature`) and want to merge it into the main line:

```bash
jj new trunk my-feature
```

This creates a merge commit with `trunk` and `my-feature` as parents.

**Viewing History**:

```bash
jj log -r trunk
# or with the revset alias
jj log -r trunk()
```

**Publishing/Pushing Changes (from `trunk`)**:

```bash
jj git push -r trunk --branch main@origin
```

This pushes your local `trunk` to the `main` branch on your `origin` remote.

- `git_head()`: Usually points to the currently checked-out Git branch.

If you decide to keep the `trunk` bookmark, update it regularly with:

```bash
jj rebase -d main@origin -s trunk
```

This ensures your changes are based off of the latest changes on `main`.

To delete the `trunk` bookmark:

```bash
jj bookmark delete trunk
```
