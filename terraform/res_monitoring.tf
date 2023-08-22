module "monitoring" {
  source  = "./monitoring"
  context = module.this

  notification_channels = var.notification_channels
  prometheus_endpoint   = aws_prometheus_workspace.prometheus.prometheus_endpoint
  ecs_service_name      = module.ecs.service_name
  ecs_target_group_arn  = module.ecs.target_group_arn
  load_balancer_arn     = module.ecs.load_balancer_arn_suffix
  keystore_cluster_id   = module.keystore.cluster_id
  monitoring_role_arn   = data.terraform_remote_state.monitoring.outputs.grafana_workspaces.main.iam_role_arn
}
