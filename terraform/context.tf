module "this" {
  source  = "app.terraform.io/wallet-connect/label/null"
  version = "0.3.2"

  namespace = "walletconnect"
  region    = var.region
  stage     = local.stage
  name      = var.name

  tags = {
    Application = "keyserver"
  }
}
