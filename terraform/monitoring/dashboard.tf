data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "keyserver - ${module.this.stage}"
    dashboard_uid   = "keyserver-${module.this.stage}"

    prometheus_uid = grafana_data_source.prometheus.uid
    cloudwatch_uid = grafana_data_source.cloudwatch.uid

    notifications    = jsonencode(var.notification_channels)
    environment      = module.this.stage
    ecs_service_name = var.ecs_service_name
    target_group     = var.ecs_target_group_arn
    load_balancer    = var.load_balancer_arn
    docdb_cluster_id = var.keystore_cluster_id
  }
}

resource "grafana_dashboard" "main" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
