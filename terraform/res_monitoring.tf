#module "monitoring" {
#  source = "./monitoring"
#
#  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
#  environment             = terraform.workspace
#  ecs_service_name        = module.ecs.service_name
#  target_group            = module.ecs.target_group_arn
#  load_balancer           = module.ecs.load_balancer_arn_suffix
#  docdb_cluster_id        = module.keystore.cluster_id
#}
