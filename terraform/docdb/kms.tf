locals {
  master_password = aws_secretsmanager_secret_version.master_password.secret_string
}

resource "random_password" "master_password" {
  length  = 16
  special = false
}

#tfsec:ignore:aws-ssm-secret-use-customer-key
resource "aws_secretsmanager_secret" "master_password" {
  name = "${local.name_prefix}-master-password"
}

resource "aws_secretsmanager_secret_version" "master_password" {
  secret_id     = aws_secretsmanager_secret.master_password.id
  secret_string = random_password.master_password.result
}

resource "aws_kms_key" "docdb_encryption" {
  enable_key_rotation = true
}
