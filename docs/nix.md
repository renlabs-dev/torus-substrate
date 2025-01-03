# Build and development with Nix

The recommended way to install dependencies on the Torus Node project is using
[Nix] and [direnv].

You can install Nix with the [Determinate Nix Installer]:

```sh
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

## With direnv

You can then install and use [direnv] to automatically load the environment:

- Install direnv:

  ```sh
  nix profile install nixpkgs#direnv
  ```

- Add to your `~/.zshrc` and reload your shell if you're using zsh:

  ```sh
  eval "$(direnv hook zsh)"
  ```

- For bash, add to your `~/.bashrc`:

  ```sh
  eval "$(direnv hook bash)"
  ```

- Run `direnv allow` inside the project directory to enable it.

### Without direnv

You can also manually load the environment:

```sh
nix develop
```

[Nix]: https://nixos.org/
[direnv]: https://direnv.net/
[Determinate Nix Installer]: https://github.com/DeterminateSystems/nix-installer
