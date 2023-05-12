# Terraform Configuration
terraform {
  required_version = "~> 1.0"

  backend "remote" {
    hostname     = "app.terraform.io"
    organization = "wallet-connect"
    workspaces {
      prefix = "keyserver-"
    }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
    grafana = {
      source  = "grafana/grafana"
      version = "~> 1.24"
    }
  }
}
