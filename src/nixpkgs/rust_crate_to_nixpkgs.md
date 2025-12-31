# Packaging a Rust crate for Nixpkgs

> NOTE: This example assumes that you've already pushed a crate to `crates.io`,
> or are packaging an existing rust crate for Nixpkgs.

Since I'm building an mdbook preprocessor, I can check out the existing
preprocessors within Nixpkgs to see their structure. They are located in
`nixpkgs/pkgs/by-name/md/`, which is where I'll place my package as well.

First you'll want to fork and clone Nixpkgs, I was able to do this with a
shallow clone:
[Local Nixpkgs](https://saylesss88.github.io/Working_with_Nixpkgs_Locally_10.html)

1. Create a branch for your PR:

```bash
git switch -c mdbook-rss-feed
```

2. Prefetch the crate source:

Use `fetchCrate` / `crate2nix` style workflow, or just prefetch the `crates.io`
tarball:

```bash
nix-prefetch-url \
  --unpack \
  https://crates.io/api/v1/crates/mdbook-rss-feed/1.3.0/download
```

That prints a `sha256-...` hash after downloading the crate source.

Use that as the `src` hash:

```nix
src = builtins.fetchTarball {
  url = "https://crates.io/api/v1/crates/mdbook-rss-feed/1.3.0/download";
  sha256 = "output of nix-prefetch-url above";
};
```

3. Get `cargoHash` via a failing build

Once `src` is wired up, do the usual `cargoHash` dance:

```nix
{
  lib,
  rustPlatform,
  versionCheckHook,
}:
rustPlatform.buildRustPackage rec {
  pname = "mdbook-rss-feed";
  version = "1.3.0";

  src = builtins.fetchTarball {
    url = "https://crates.io/api/v1/crates/mdbook-rss-feed/1.3.0/download";
    sha256 = "output of nix-prefetch-url above";
  };

  cargoHash = lib.fakeHash;

  nativeInstallCheckInuts = [
    versionCheckHook
  ];
  doInstallCheck = true;

  meta = {
    description = "mdBook preprocessor that generates RSS, Atom, and JSON feeds";
    mainProgram = "mdbook-rss-feed";
    homePage = "https://crates.io/crates/mdbook-rss-feed";
    license = lib.licenses.asl20;
    maintainers = [lib.maintainers.saylesss88]
  };
}
```

Then in the `nixpkgs` root, run:

```bash
nix-build -A mdbook-rss-feed
```

Nix will fail with a message like:

```text
hash mismatch
specified: sha256-....
got: sha256-1...
```

Copy the `got` value into `cargoHash`, rebuild, and you're done.

4. Test from `nixpkgs` root (i.e., the `nixpkgs` directory):

```bash
./result/bin/mdbook-rss-feed --version
```

---

## Adding yourself as maintainer

Edit `nixpkgs/maintainers/maintainer-list.nix`:

```nix
your-handle = {
  email = "you@example.com";
  name = "Your Name";
  github = "your-gh-handle";
  # Optional
  # githubId = 12345678;
};
```

- You can get `githubId` from `https://api.github.com/users/your-user`

**Use the handle in your package**:

```nix
meta = {
  # ...
  maintainers = [lib.maintainers.your-user];
};
```

---

## Quick upstream sync check

From the `mdbook-rss-feed` branch:

```bash
# Ensure you have upstream remote (NixOS/nixpkgs)
git remote -v   # should show 'upstream' -> https://github.com/NixOS/nixpkgs.git

# if missing, add it:
# git remote add upstream https://github.com/NixOS/nixpkgs.git

# Fetch the latest upstream
git fetch upstream master

# See if anything new happened since your branch base
git log --oneline upstream/master..mdbook-rss-feed  # our changes
git log --oneline mdbook-rss-feed..upstream/master  # upstream changes
```

If `mdbook-rss-feed..upstream/master` shows commits, rebase to stay current:

```bash
git rebase upstream/master
# resolve any conflicts if they appear (unlikely for new packages)
git rebase --continue
```

**Then commit and push**

```bash
git commit -m "mdbook-rss-feed: init at 1.3.0"
git push -u origin mdbook-rss-feed
```

Then:

1. Go to GitHub -> your fork

2. Click "Compare & pull request" on the `mdbook-rss-feed` branch

3. Fill out the PR template (why useful, tested on x86_64-linux, etc.)

4. Submit!

The package will go through CI checks, and once green + approved by a
maintainer, it'll land in nixpkgs.

<details>
<summary> ✔️ Example PR template filled out </summary>
