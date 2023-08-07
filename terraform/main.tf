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

#tfsec:ignore:aws-ec2-require-vpc-flow-logs-for-all-vpcs
#tfsec:ignore:aws-ec2-no-public-ip-subnet
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "3.0.0"
  name    = "${terraform.workspace}-${local.app_name}"
  cidr    = "10.0.0.0/16"

  azs             = var.azs
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.4.0/24", "10.0.5.0/24", "10.0.6.0/24"]

  private_subnet_tags = {
    Visibility = "private"
  }
  public_subnet_tags = {
    Visibility = "public"
  }

  enable_dns_support     = true
  enable_dns_hostnames   = true
  enable_nat_gateway     = true
  single_nat_gateway     = true
  one_nat_gateway_per_az = false
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

  vpc_id                      = module.vpc.vpc_id
  public_subnet_ids           = module.vpc.public_subnets
  private_subnet_ids          = module.vpc.private_subnets
  allowed_ingress_cidr_blocks = [module.vpc.vpc_cidr_block]

  persistent_keystore_mongo_addr = module.keystore_docdb.connection_url
}

module "keystore_docdb" {
  source = "./docdb"

  app_name                           = local.app_name
  mongo_name                         = "keystore-docdb"
  environment                        = terraform.workspace
  default_database                   = "keystore"
  primary_instances                  = var.keystore_docdb_primary_instances
  primary_instance_class             = var.keystore_docdb_primary_instance_class
  replica_instances                  = var.keystore_docdb_replica_instances
  replica_instance_class             = var.keystore_docdb_replica_instance_class
  vpc_id                             = module.vpc.vpc_id
  private_subnet_ids                 = module.vpc.private_subnets
  allowed_ingress_cidr_blocks        = [module.vpc.vpc_cidr_block]
  allowed_egress_cidr_blocks         = [module.vpc.vpc_cidr_block]
  legacy_vpc_id                      = data.aws_vpc.vpc.id
  legacy_private_subnet_ids          = data.aws_subnets.private_subnets.ids
  legacy_allowed_ingress_cidr_blocks = [data.aws_vpc.vpc.cidr_block]
  legacy_allowed_egress_cidr_blocks  = [data.aws_vpc.vpc.cidr_block]
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
