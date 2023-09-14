resource "random_pet" "this" {
  length = 2
}

locals {
  keystore_name      = "keystore"
  ecr_repository_url = data.terraform_remote_state.org.outputs.accounts.wl.keys[local.stage].ecr-url

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
