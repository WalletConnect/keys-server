module "cloudwatch" {
  source  = "app.terraform.io/wallet-connect/cloudwatch-constants/aws"
  version = "1.0.0"
}

locals {
  alarm_prefix       = "${title(module.this.name)} - ${title(module.this.stage)}"
  evaluation_periods = 1
  period             = 60 * 5
}

#tfsec:ignore:aws-sns-enable-topic-encryption
resource "aws_sns_topic" "webhook" {
  name         = "cloudwatch-webhook"
  display_name = "CloudWatch Webhook forwarding to BetterUptime"
}

resource "aws_sns_topic_subscription" "webhook" {
  endpoint  = var.webhook_url
  protocol  = "https"
  topic_arn = aws_sns_topic.webhook.arn
}
