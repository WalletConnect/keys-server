# keyserver-rust

## API Guide

```sh
RUST_SERVER_URL="keys.walletconnect.com"

# Check health
curl -i -X GET "$RUST_SERVER_URL/health"

# Appends set of identity key for account. Account might have multiple identity keys.
curl -i -X POST -H "Content-Type: application/json" \
    -d '{"identityKey": "2d573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"}' \
    "$RUST_SERVER_URL/account/eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826/identityKey"

# Register proposal encryption key. One per account
curl -i -X POST -H "Content-Type: application/json" \
    -d '{"proposalEncryptionKey": "2d573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"}' \
    "$RUST_SERVER_URL/account/eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826/proposalEncryptionKey"

# Checks identity key existance. 
curl -i -X GET "$RUST_SERVER_URL/identityKey/2f573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"

# Resolves account. Returns account with all identity keys and proposal encryption key
curl -i -X GET "$RUST_SERVER_URL/account/eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"

# Removes account. 
curl -i -X DELETE "$RUST_SERVER_URL/account/eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"

# Removes identity key
curl -i -X DELETE "$RUST_SERVER_URL/account/eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826/identityKey/2d573da1d2b8dbe3dcdb6ce7de47ce44b18fb8ec5ddc9d3f412ab4a718fff93c"
```


## Running Locally

Setup:
- Install [`rust`](https://www.rust-lang.org/tools/install);
- Install [`docker`](https://docs.docker.com/get-docker/);
- Install [`just`](https://github.com/casey/just#packages);
- Copy the env file:
  ```sh
  $ cp .env.example .env
  ```
- Fill `.env` file with necessary values

Running the keyserver:
```sh
$ source .env # make sure the env variables are set
$ just run
```

Running the docker-compose set up (MongoDB + MongoExpress + Jaeger + Keyserver):
```sh
$ source .env # make sure the env variables are set
$ just build-docker
$ just run-docker
```

Running tests:
```sh
$ just test
```