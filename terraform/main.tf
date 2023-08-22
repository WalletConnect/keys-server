data "terraform_remote_state" "org" {
  backend = "remote"
  config = {
    organization = "wallet-connect"
    workspaces = {
      name = "aws-org"
    }
  }
}

data "terraform_remote_state" "dns" {
  backend = "remote"
  config = {
    organization = "wallet-connect"
    workspaces = {
      name = "dns-delegation"
    }
  }
}

data "terraform_remote_state" "monitoring" {
  backend = "remote"
  config = {
    organization = "wallet-connect"
    workspaces = {
      name = "monitoring"
    }
  }
}

resource "random_pet" "this" {
  length = 2
}

locals {
  keystore_name      = "keystore"
  ecr_repository_url = data.terraform_remote_state.org.outputs.accounts-ecr-urls.wl.keys[local.stage]

  stage = lookup({
    "keyserver-wl-staging" = "staging",
    "keyserver-wl-prod"    = "prod",
    "keyserver-staging"    = "staging",
    "keyserver-prod"       = "prod",
    "wl-staging"           = "staging",
    "wl-prod"              = "prod",
    "staging"              = "staging",
    "prod"                 = "prod",
  }, terraform.workspace, terraform.workspace)
}
