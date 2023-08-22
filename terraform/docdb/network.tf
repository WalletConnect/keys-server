resource "aws_docdb_subnet_group" "private_subnets" {
  name       = "${local.name_prefix}-private-subnet-group"
  subnet_ids = var.private_subnet_ids
}

resource "aws_security_group" "service_security_group" {
  name        = "${local.name_prefix}-service"
  description = "Allow ingress from the application"
  vpc_id      = var.vpc_id

  ingress {
    description = "Allow inbound traffic to the DocDB cluster"
    from_port   = 27017
    to_port     = 27017
    protocol    = "TCP"
    cidr_blocks = var.allowed_ingress_cidr_blocks
  }

  egress {
    description = "Allow outbound traffic from the DocDB cluster"
    from_port   = 0    # Allowing any incoming port
    to_port     = 0    # Allowing any outgoing port
    protocol    = "-1" # Allowing any outgoing protocol
    cidr_blocks = var.allowed_egress_cidr_blocks
  }
}
