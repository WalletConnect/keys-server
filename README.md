# Keys-Server

## API Guide

For API see [WalletConnect Docs](https://docs.walletconnect.com/2.0/specs/servers/keys/keys-server-api)

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

Running the keys-server:
```sh
$ source .env # make sure the env variables are set
$ just run
```

Running the docker-compose set up (MongoDB + MongoExpress + Jaeger + Keys-Server):
```sh
$ source .env # make sure the env variables are set
$ just build-docker
$ just run-docker
```

Running tests:
```sh
$ just test
```
