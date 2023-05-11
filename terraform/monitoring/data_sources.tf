locals {
  prometheus_url = "https://aps-workspaces.eu-central-1.amazonaws.com/workspaces/${var.prometheus_workspace_id}/"
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${var.environment}-keyserver-amp"
  url  = local.prometheus_url

  json_data_encoded = jsonencode({
    httpMethod    = "GET"
    sigV4Auth     = true
    sigV4AuthType = "workspace-iam-role"
    sigV4Region   = "eu-central-1"
  })
}

resource "grafana_data_source" "cloudwatch" {
  type = "cloudwatch"
  name = "${var.environment}-keyserver-cloudwatch"

  json_data_encoded = jsonencode({
    defaultRegion = "eu-central-1"
  })
}
