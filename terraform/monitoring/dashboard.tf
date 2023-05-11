data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "${var.environment} - _keyserver"
    dashboard_uid   = "${var.environment}-_keyserver"

    prometheus_uid = grafana_data_source.prometheus.uid
    cloudwatch_uid = grafana_data_source.cloudwatch.uid

    environment      = var.environment
    ecs_service_name = var.ecs_service_name
    loadbalancer_arn = var.loadbalancer_arn
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
