resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${module.this.id}"
}

resource "aws_prometheus_alert_manager_definition" "prometheus_alerts" {
  workspace_id = aws_prometheus_workspace.prometheus.id
  definition   = <<-EOF
    alertmanager_config: |
      route:
        receiver: 'BetterUptime'
      receivers:
        - name: 'BetterUptime'
          sns_configs:
            - topic_arn: ${aws_sns_topic.prometheus_webhook.arn}
    EOF
}

resource "aws_sns_topic" "prometheus_webhook" {
  name         = "prometheus-webhook"
  display_name = "Prometheus Webhook forwarding to BetterUptime"
}

resource "aws_sns_topic_subscription" "prometheus_webhook" {
  endpoint  = var.betterstack_prometheus_webhook
  protocol  = "https"
  topic_arn = aws_sns_topic.prometheus_webhook.arn
}
