#module "docdb-autoscaling" {
#  source  = "app.terraform.io/wallet-connect/docdb-autoscaling/aws"
#  version = "0.1.3"
#  context = module.this
#
#  cluster_identifier = aws_docdb_cluster.main.id
#  min_capacity       = 1
#  max_capacity       = 6
#
#  scale_out_policy = [{
#    metric_name = "CPUUtilization"
#    target      = 80
#    statistic   = "Average"
#    cooldown    = 900
#  }]
#
#  scale_in_policy = [{
#    metric_name = "CPUUtilization"
#    target      = 20
#    statistic   = "Average"
#    cooldown    = 900
#  }]
#}
