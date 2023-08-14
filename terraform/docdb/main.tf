locals {
  name_prefix = replace("${module.this.stage}-${module.this.name}-${var.db_name}", "_", "-")
}

resource "aws_docdb_cluster" "main" {
  cluster_identifier              = local.name_prefix
  master_username                 = "keyserver"
  master_password                 = local.master_password
  port                            = var.port
  db_subnet_group_name            = aws_docdb_subnet_group.private_subnets.name
  storage_encrypted               = true
  kms_key_id                      = aws_kms_key.docdb_encryption.arn
  enabled_cloudwatch_logs_exports = ["audit"]

  vpc_security_group_ids = [
    aws_security_group.service_security_group.id
  ]
  skip_final_snapshot = true
}

#tfsec:ignore:aws-documentdb-encryption-customer-key
resource "aws_docdb_cluster_instance" "primary" {
  count              = var.primary_instance_count
  identifier         = "${local.name_prefix}-primary-instance-${count.index}"
  cluster_identifier = aws_docdb_cluster.main.id
  instance_class     = var.primary_instance_class
  promotion_tier     = 0
}

#tfsec:ignore:aws-documentdb-encryption-customer-key
resource "aws_docdb_cluster_instance" "replica" {
  count              = var.replica_instance_count
  identifier         = "${local.name_prefix}-replica-instance-${count.index}"
  cluster_identifier = aws_docdb_cluster.main.id
  instance_class     = var.replica_instance_class
  promotion_tier     = 1
}
