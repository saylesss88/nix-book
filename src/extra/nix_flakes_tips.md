# Sub-Chapter 1

<!--toc:start-->

- [Sub-Chapter 1](#sub-chapter-1)
- [Shallow Clone Nixpkgs](#shallow-clone-nixpkgs)
  - [Import your Non-Flake Wallpaper Repo](#import-your-non-flake-wallpaper-repo)
  - [Understanding @-patterns](#understanding-patterns)
  - [Understanding `specialArgs`](#understanding-specialargs)
  - [Set up Flake Check and Formatter Outputs](#set-up-flake-check-and-formatter-outputs) -
  [Add a devShell Output](#add-a-devshell-output)
  <!--toc:end-->

# Shallow Clone Nixpkgs

<!-- ![gruv8](../images/gruv8.png) -->

1. Shallow clone nixpkgs, the full Git history isn't always necessary and this
   can speed up build times.

- The only issue I've had is `nix-index-database` not working well with the
  shallow clone... Also `jujutsu` may not play well with this. This may benefit
  low powered machines the most.

```nix
# flake.nix
inputs = {
    nixpkgs.url = "git+https://github.com/NixOS/nixpkgs?shallow=1&ref=nixos-unstable";
};
```

- Some times when you might need a full clone are debugging, working with
  repository history, and for pull requests.

## Import your Non-Flake Wallpaper Repo

2. Importing your non-flake wallpapers repo, you can use this one there are a
   ton of wallpapers I've collected over time:

```nix
# flake.nix
inputs = {
    wallpapers = {
      url = "github:saylesss88/wallpapers";
      flake = false;
    };
}
```

- After adding the input I can access individual wallpapers by adding the
  `inputs` argument and something like
  `path = "${inputs.wallpapers}/Aesthetic Scenery.jpg";`

## Understanding @-patterns

3. Understanding `@-patterns`, being able to reference your outputs argument set
   as a whole. An `@-pattern` is a way for a function to access variadic
   attributes (i.e. varying number of arguments).

```nix
# flake.nix
inputs = {
    home-manager.url = "github:nix-community/home-manager/master";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
    stylix.url = "github:danth/stylix";
};
outputs = {
    self,
    nixpkgs,
    home-manager,
} @ inputs:
```

With the above example to add the modules to your nixosConfigurations you would
add something like this:

```nix
# flake.nix
nixosConfigurations.${host} = nixpkgs.lib.nixosSystem {
  system = "x86_64-linux";
  specialArgs = {
    inherit inputs username host;
};
modules = [
  ./hosts/${host}/config.nix
  inputs.stylix.nixosModules.stylix
  home-manager.nixosModules.home-manager
  # .. snip ..
];
```

- Notice that since home-manager was explicitly listed in the outputs arguments:
  `outputs = { self, nixpkgs, home-manager, }; ` the `inputs` prefix is
  unnecessary. If home-manager was removed from the outputs arguments:
  `outputs = { self, ... }` then you would need
  `modules = [ inputs.home-manager.nixosModules.home-manager];` This can be
  confusing because many docs assume your not using an @-pattern so if you have
  one in your flake you need to prefix with `inputs`. I use this to reference my
  personal wallpapers repo mentioned earlier.

## Understanding `specialArgs`

4. Understanding `specialArgs` (nixos) and `extraSpecialArgs` (home-manager).
   Building on the @-patterns, using `specialArgs` and `extraSpecialArgs` is a
   way to pass arguments from your flake to your NixOS and home-manager modules.

For example, here is a snippet of some variables I set:

```nix
# flake.nix
outputs = {
  self,
  nixpkgs,
  home-manager,
  ...
} @ inputs: let
  system = "x86_64-linux";
  host = "magic";
  username = "jr";
  userVars = {
    timezone = "America/New_York";
    locale = "en_US.UTF-8";
    gitUsername = "saylesss88";
    dotfilesDir = "~/.dotfiles";
    wm = "hyprland";
    browser = "firefox";
    term = "ghostty";
    editor = "hx";
    keyboardLayout = "us";
  };
  in
```

Now I can pass them as special args like this:

```nix
# flake.nix
nixosConfigurations = {
      ${host} = nixpkgs.lib.nixosSystem {
        inherit system;
        specialArgs = {
          inherit
            inputs
            username
            host
            userVars
            ;
        };
        modules = [
        ./hosts/${host}/configuration.nix
        home-manager.nixosModules.home-manager
        inputs.stylix.nixosModules.stylix
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.${username} = import ./hosts/${host}/home.nix;
          home-manager.backupFileExtension = "backup";
          home-manager.extraSpecialArgs = {
            inherit
              inputs
              username
              host
              userVars
              ;
          };
        }
      ];
```

- To access values in `userVars` for example:

```nix
# git.nix
{ userVars, ... }: {
  programs = {
    git = {
      enable = true;
      userName = userVars.gitUsername;
    };
  };
}
```

## Set up Flake Check and Formatter Outputs

5. Set up `checks` and `formatter` outputs with `treefmt-nix`. Add `treefmt-nix`
   to your inputs and outputs arguments. Inside the `let` expression from tip 4
   I would add:

```nix
# flake.nix
let
# ... snip ...
pkgs = import nixpkgs {
  inherit system;
  config.allowUnfree = true;
};
treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
in
{
  checks.x86_64-linux.style = treefmtEval.config.build.check self;

  formatter.x86_64-linux = treefmtEval.config.build.wrapper;

  # ... snip ...
}
```

And in the `treefmt.nix`:

```nix
# treefmt.nix
{
projectRootFile = "flake.nix";
programs = {
  deadnix.enable = true;
  statix.enable = true;
  keep-sorted.enable = true;
  nixfmt = {
    enable = true;
    strict = true;
  };
};
settings.excludes = [
  "*.age"
  "*.jpg"
  "*.nu"
  "*.png"
  ".jj/*"
  "flake.lock"
  "justfile"
];
settings.formatter = {
  deadnix = {
    priority = 1;
  };

  statix = {
    priority = 2;
  };

  nixfmt = {
    priority = 3;
  };
};
}
```

- Use `treefmt-nix` to manage code formatters and linters as flake outputs. This
  ensures consistent styling and catches issues with tools like `deadnix`,
  `statix`, and `nixfmt`.

- Use `nix fmt` in the flake directory to format your whole configuration.

- Now you can run `nix flake check` to run your checks. Running `nix flake show`
  will list your outputs.

- Tools like `nix-fast-build` rely on flake checks and can be used after setting
  this up.

### Add a devShell Output

6. Make a devShell output:

```nix
 in
    {
      checks.x86_64-linux.style = treefmtEval.config.build.check self;

      formatter.x86_64-linux = treefmtEval.config.build.wrapper;

      devShells.${system}.default = import ./lib/dev-shell.nix { inherit inputs; };
```

and in the `dev-shell.nix` you could put something like this:

```nix
# dev-shell.nix
{
  inputs,
  system ? "x86_64-linux",
}:
let
  # Instantiate nixpkgs with the given system and allow unfree packages
  pkgs = import inputs.nixpkgs {
    localSystem = system;
    config.allowUnfree = true;
    overlays = [
      # Add overlays if needed, e.g., inputs.neovim-nightly-overlay.overlays.default
    ];
  };
in
pkgs.mkShell {
  name = "nixos-dev";
  packages = with pkgs; [
    # Nix tools
    nixfmt-rfc-style # Formatter
    deadnix # Dead code detection
    nixd # Nix language server
    nil # Alternative Nix language server
    nh # Nix helper
    nix-diff # Compare Nix derivations
    nix-tree # Visualize Nix dependencies

    # Code editing
    helix

    # General utilities
    git
    ripgrep
    jq
    tree
  ];

  shellHook = ''
    echo "Welcome to the NixOS development shell!"
    echo "System: ${system}"
    echo "🛠️ Tools available: nixfmt, deadnix, nixd, nil, nh, nix-diff, nix-tree, helix, git, ripgrep, jq, tree"
  '';
}
```

- You can enter this devshell with `nix develop` or automatically with `direnv`.
