module "cloudwatch" {
  source  = "./cloudwatch"
  context = module.this.context

  webhook_url = var.betterstack_cloudwatch_webhook

  ecs_cluster_name = module.ecs.ecs_cluster_name
  ecs_service_name = module.ecs.ecs_service_name

  docdb_cluster_id       = module.keystore.cluster_id
  docdb_memory_threshold = module.this.stage == "prod" ? 4 : 2
}
