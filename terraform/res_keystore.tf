module "keystore" {
  source  = "./docdb"
  context = module.this

  db_name                = local.keystore_name
  default_database       = local.keystore_name
  port                   = local.ports.docdb
  primary_instance_count = var.keystore_primary_instance_count
  primary_instance_class = var.keystore_primary_instance_class
  replica_instance_count = var.keystore_replica_instance_count
  replica_instance_class = var.keystore_replica_instance_class

  vpc_id                      = module.vpc.vpc_id
  private_subnet_ids          = module.vpc.intra_subnets
  allowed_ingress_cidr_blocks = [module.vpc.vpc_cidr_block]
  allowed_egress_cidr_blocks  = [module.vpc.vpc_cidr_block]
}
