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

fmt-imports:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-fmt >/dev/null; then
    echo '==> Running rustfmt'
    cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=One
  else
    echo '==> rustfmt not found in PATH, skipping'
  fi

# Build docker image
build-docker:
  @echo '=> Build keys-server docker image'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml build keyserver

# Start keys-server & storage services on docker
run-docker:
    @echo '==> Start services on docker'
    docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml up -d

# Stop keys-server & storage services on docker
stop-docker:
  @echo '==> Stop services on docker'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml down

# Clean up docker keys-server & storage services
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

# Restart keys-server on docker
restart-keyserver-docker:
  @echo '==> Restart keys-server service on docker'
  docker-compose -f ./ops/docker-compose.keyserver.yml -f ./ops/docker-compose.storage.yml up -d --build --force-recreate --no-deps keyserver

# Lint the project for any quality issues
lint: clippy fmt commit-check

unit: lint test test-all test-doc tf-lint

devloop: unit fmt-imports

# Run project linter
clippy:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-clippy >/dev/null; then
    echo '==> Running clippy'
    cargo clippy --all-features --tests -- -D clippy::all -W clippy::style
  else
    echo '==> clippy not found in PATH, skipping'
  fi

# Run code formatting check
fmt:
  #!/bin/bash
  set -euo pipefail

  if command -v cargo-fmt >/dev/null; then
    echo '==> Running rustfmt'
    cargo fmt
  else
    echo '==> rustfmt not found in PATH, skipping'
  fi

# Run commit checker
commit-check:
  #!/bin/bash
  set -euo pipefail

  if command -v cog >/dev/null; then
    echo '==> Running cog check'
    cog check --from-latest-tag
  else
    echo '==> cog not found in PATH, skipping'
  fi

tf-lint: tf-validate tf-check-fmt tfsec tflint

# Check Terraform formating
tf-check-fmt:
  #!/bin/bash
  set -euo pipefail

  if command -v terraform >/dev/null; then
    echo '==> Checking terraform fmt'
    terraform -chdir=terraform fmt -check -recursive
  else
    echo '==> Terraform not found in PATH, skipping'
  fi

tf-validate:
  #!/bin/bash
  set -euo pipefail

  if command -v terraform >/dev/null; then
    echo '==> Running terraform validate'
    terraform -chdir=terraform validate
  else
    echo '==> Terraform not found in PATH, skipping'
  fi

# Check Terraform for potential security issues
tfsec:
  #!/bin/bash
  set -euo pipefail

  if command -v tfsec >/dev/null; then
    echo '==> Running tfsec'
    cd terraform
    tfsec
  else
    echo '==> tfsec not found in PATH, skipping'
  fi

# Run Terraform linter
tflint:
  #!/bin/bash
  set -euo pipefail

  if command -v tflint >/dev/null; then
    echo '==> Running tflint'
    cd terraform
    tflint
    tflint ./ecs
    tflint ./monitoring
    tflint ./docdb

  else
    echo '==> tflint not found in PATH, skipping'
  fi
