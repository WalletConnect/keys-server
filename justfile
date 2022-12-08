# Default to listing recipes
_default:
  @just --list --list-prefix '  > '

# Fast check project for errors
check:
  @echo '==> Checking project for compile errors'
  cargo check

# Build service for development
build:
  @echo '==> Building project'
  cargo build

# Run the service
run: build
  @echo '==> Running project (ctrl+c to exit)'
  cargo run

# Run project test suite, skipping storage tests
test:
  @echo '==> Testing project (default)'
  cargo test

# Run project test suite, including storage tests (requires storage docker services to be running)
test-all:
  @echo '==> Testing project (all features)'
  cargo test --all-features

# Run test from project documentation
test-doc:
  @echo '==> Testing project docs'
  cargo test --doc

# Clean build artifacts
clean:
  @echo '==> Cleaning project target/*'
  cargo clean

# Reformat code
fmt:
  @echo '==> Reformatting code'
  cargo +nightly fmt

# Build docker image
build-docker:
  @echo '=> Build keyserver docker image'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml build keyserver

# Start keyserver & storage services on docker
run-docker:
    @echo '==> Start services on docker'
    docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml up -d

# Stop keyserver & storage services on docker
stop-docker:
  @echo '==> Stop services on docker'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml down

# Clean up docker keyserver & storage services
clean-docker:
  @echo '==> Clean services on docker'
  docker-compose  -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml stop
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml rm -f

# Start storage services on docker
run-storage-docker:
  @echo '==> Start storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml up -d

# Stop storage services on docker
stop-storage-docker:
  @echo '==> Stop storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml down

# Clean up docker storage services
clean-storage-docker:
  @echo '==> Clean storage services on docker'
  docker-compose -f ./ops/docker-compose.storage.yml stop
  docker-compose -f ./ops/docker-compose.storage.yml rm -f

# Restart keyserver on docker
restart-keyserver-docker:
  @echo '==> Restart keyserver service on docker'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml up -d --build --force-recreate --no-deps keyserver 
