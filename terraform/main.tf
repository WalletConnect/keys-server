locals {
  app_name         = "keyserver"
  hosted_zone_name = "keys.walletconnect.com"
  fqdn             = terraform.workspace == "prod" ? local.hosted_zone_name : "${terraform.workspace}.${local.hosted_zone_name}"
}

module "tags" {
  # tflint-ignore: terraform_module_pinned_source
  source = "github.com/WalletConnect/terraform-modules/modules/tags"

  application = local.app_name
  env         = terraform.workspace
}

module "dns" {
  # tflint-ignore: terraform_module_pinned_source
  source = "github.com/WalletConnect/terraform-modules/modules/dns"

  hosted_zone_name = local.hosted_zone_name
  fqdn             = local.fqdn
}

data "aws_ecr_repository" "repository" {
  name = local.app_name
}


# ECS Cluster, Task, Service, and Load Balancer for our app
module "ecs" {
  source = "./ecs"

  ecr_repository_url  = data.aws_ecr_repository.repository.repository_url
  image_version       = var.image_version
  app_name            = "${terraform.workspace}_${local.app_name}"
  region              = var.region
  port                = 8080
  acm_certificate_arn = module.dns.certificate_arn
  fqdn                = local.fqdn
  route53_zone_id     = module.dns.zone_id
  prometheus_endpoint = aws_prometheus_workspace.prometheus.prometheus_endpoint

  vpc_id                      = data.aws_vpc.vpc.id
  public_subnet_ids           = data.aws_subnets.public_subnets.ids
  private_subnet_ids          = data.aws_subnets.private_subnets.ids
  allowed_ingress_cidr_blocks = [data.aws_vpc.vpc.cidr_block]

  persistent_keystore_mongo_addr = module.keystore_docdb.connection_url
}

moved {
  from = module.keystore-docdb
  to   = module.keystore_docdb
}

module "keystore_docdb" {
  source = "./docdb"

  app_name                    = local.app_name
  mongo_name                  = "keystore-docdb"
  environment                 = terraform.workspace
  default_database            = "keystore"
  primary_instances           = var.keystore_docdb_primary_instances
  primary_instance_class      = var.keystore_docdb_primary_instance_class
  replica_instances           = var.keystore_docdb_replica_instances
  replica_instance_class      = var.keystore_docdb_replica_instance_class
  vpc_id                      = data.aws_vpc.vpc.id
  private_subnet_ids          = data.aws_subnets.private_subnets.ids
  allowed_ingress_cidr_blocks = [data.aws_vpc.vpc.cidr_block]
  allowed_egress_cidr_blocks  = [data.aws_vpc.vpc.cidr_block]
}

module "o11y" {
  source = "./monitoring"

  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
  environment             = terraform.workspace
  ecs_service_name        = module.ecs.service_name
  target_group            = module.ecs.target_group_arn
  load_balancer           = module.ecs.load_balancer_arn_suffix
  docdb_cluster_id        = module.keystore_docdb.cluster_id
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}
