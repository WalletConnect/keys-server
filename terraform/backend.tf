# Terraform Configuration
terraform {
  required_version = "~> 1.0"
  required_providers {
    assert = {
      source  = "bwoznicki/assert"
      version = "~> 0.0.1"
    }
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
    grafana = {
      source  = "grafana/grafana"
      version = "~> 1.24"
    }
  }

  backend "s3" {
    region               = "eu-central-1"
    bucket               = "opz"
    workspace_key_prefix = "infra/env"
    key                  = "apps/keyserver.tfstate"

    force_path_style = true
  }
}
