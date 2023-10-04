data "aws_s3_bucket" "geoip" {
  bucket = data.terraform_remote_state.infra_aws.outputs.geoip_bucked_id
}

# ECS Cluster, Task, Service, and Load Balancer for our app
module "ecs" {
  source  = "./ecs"
  context = module.this

  # Cluster
  ecr_repository_url = local.ecr_repository_url
  image_version      = var.image_version
  task_cpu           = 512
  task_memory        = 1024
  min_capacity       = 2
  max_capacity       = 8

  # DNS
  route53_zones              = local.zones
  route53_zones_certificates = local.zones_certificates

  # Network
  vpc_id                          = module.vpc.vpc_id
  public_subnets                  = module.vpc.public_subnets
  private_subnets                 = module.vpc.private_subnets
  allowed_app_ingress_cidr_blocks = module.vpc.vpc_cidr_block
  allowed_lb_ingress_cidr_blocks  = module.vpc.vpc_cidr_block

  # Application
  port          = 8080
  keystore_addr = module.keystore.connection_url
  log_level     = var.log_level

  # Monitoring
  prometheus_endpoint = aws_prometheus_workspace.prometheus.prometheus_endpoint

  # GeoIP
  geoip_db_bucket_name = data.aws_s3_bucket.geoip.id
  geoip_db_key         = var.geoip_db_key
}
