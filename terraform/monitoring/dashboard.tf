data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "${var.environment} - keyserver"
    dashboard_uid   = "${var.environment}-keyserver"

    prometheus_uid = grafana_data_source.prometheus.uid
    cloudwatch_uid = grafana_data_source.cloudwatch.uid

    notifications    = jsonencode(local.notifications)
    environment      = var.environment
    ecs_service_name = var.ecs_service_name
    target_group     = var.target_group
    load_balancer    = var.load_balancer
    docdb_cluster_id = var.docdb_cluster_id
  }
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly
resource "grafana_dashboard" "main" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
