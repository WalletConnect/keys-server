module "docdb-autoscaling" {
  source             = "github.com/theuves/docdb-autoscaling?ref=06de20e170853b515cc6ae986ceb5941f7b34f5e"
  cluster_identifier = aws_docdb_cluster.docdb_primary.id
  name               = "${var.environment}-${var.app_name}-docdb-autoscaling"
  min_capacity       = 0
  max_capacity       = 6

  scaling_policy = [
    {
      metric_name = "CPUUtilization"
      target      = 80
      statistic   = "Average"
      cooldown    = 300
    }
  ]
}
