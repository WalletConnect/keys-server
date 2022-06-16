# keyserver-rust

```sh
RUST_SERVER_URL="keys.walletconnect.com" // Not available yet
RUST_SERVER_URL="159.65.123.131:8080"

curl -i -X DELETE "$RUST_SERVER_URL/keys"

curl -i -X GET "$RUST_SERVER_URL/keys"

curl -i -X POST -H "Content-Type: application/json" \
    -d '{"account": "eip:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826", "publicKey": "2d573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"}' \
    "$RUST_SERVER_URL/register"

curl -i -X GET "$RUST_SERVER_URL/resolve?account=eip:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"

curl -i -X DELETE "$RUST_SERVER_URL/remove/eip:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
```

## Getting Started

### Nix

Install nix:

```bash
sh <(curl -L https://nixos.org/nix/install)

```

By using `nix` you can setup your development environment for the following architectures:

- aarch64-darwin
- aarch64-linux
- x86_64-darwin
- x86_64-linux

You need to enable [Nix Flakes](https://nixos.wiki/wiki/Flakes) for `nix`. To do this,
you can either add the following command line options each time to run the `nix` command:

- `--extra-experimental-features nix-command`
- `--extra-experimental-features flakes`

or add the following config to `~/.config/nix/nix.conf`:

```conf
experimental-features = nix-command flakes
```

Once the above is done, run the command `nix develop` to enter your developer environment.
Nix programs, ending with `.nix`. The official formatter for these programs is:

- alejandra: https://github.com/kamadorueda/alejandra

##### Install Direnv

Direnv allows for the automatic setup of the `nix` virtual environment each time you enter
the repository. **You only want `direnv` if you want to avoid running `nix develop` each time you want to deploy or build this package.**

1. [Install](https://direnv.net/docs/installation.html)
2. [Setup your shell to use direnv](https://direnv.net/docs/hook.html)

##### For Nix Flake compatibility with direnv

Do this for the first time:

```
git clone https://github.com/nix-community/nix-direnv $HOME/nix-direnv

# then, put this in ~/.direnvrc
#   to source nix-direnv into .direnv
echo "source $HOME/nix-direnv/direnvrc" >> ~/.direnvrc
```

To add nix caching for faster environment loading, you need to add the
following to `~/.config/nix/nix.conf`

```
keep-derivations = true
keep-outputs = true
```

#### Deploy-rs

You can quickly deploy this code to the server by running:

`nix develop && deploy`
