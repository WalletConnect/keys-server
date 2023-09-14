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
