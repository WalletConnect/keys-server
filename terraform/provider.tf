provider "aws" {
  region = var.region

  default_tags {
    tags = module.this.tags
  }
}

provider "grafana" {
  url  = "https://${var.grafana_endpoint}"
  auth = var.grafana_auth
}
