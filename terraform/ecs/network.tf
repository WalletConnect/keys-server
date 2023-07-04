# Load Balancer
# tfsec:ignore:aws-elb-alb-not-public
resource "aws_alb" "application_load_balancer" {
  name               = replace("${var.app_name}-lb-${substr(uuid(), 0, 3)}", "_", "-")
  load_balancer_type = "application"
  subnets            = var.public_subnet_ids

  lifecycle {
    create_before_destroy = true
    ignore_changes        = [name]
  }
  drop_invalid_header_fields = true
}

resource "aws_lb_target_group" "target_group" {
  name        = replace("${var.app_name}-${substr(uuid(), 0, 3)}", "_", "-")
  port        = var.port
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = var.vpc_id

  # Deregister quickly to allow for faster deployments
  deregistration_delay = 30 # Seconds

  health_check {
    protocol            = "HTTP"
    path                = "/health"
    interval            = 10
    healthy_threshold   = 2
    unhealthy_threshold = 2
  }

  lifecycle {
    create_before_destroy = true
    ignore_changes        = [name]
  }
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn = aws_alb.application_load_balancer.arn
  port              = "443"
  protocol          = "HTTP"
  certificate_arn   = var.acm_certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.target_group.arn
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_lb_listener" "listener-http" {
  load_balancer_arn = aws_alb.application_load_balancer.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"

    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}

# Security Groups
resource "aws_security_group" "tls_ingess" {
  name        = "${var.app_name}-tls-ingress"
  description = "Allow tls ingress from everywhere"
  vpc_id      = var.vpc_id

  ingress { #tfsec:ignore:aws-ec2-add-description-to-security-group-rule
    from_port = 443
    to_port   = 443
    protocol  = "tcp"
    #tfsec:ignore:aws-ec2-no-public-ingress-sgr
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic in from all sources
  }

  egress {           #tfsec:ignore:aws-ec2-add-description-to-security-group-rule
    from_port = 0    # Allowing any incoming port
    to_port   = 0    # Allowing any outgoing port
    protocol  = "-1" # Allowing any outgoing protocol
    #tfsec:ignore:aws-ec2-no-public-egress-sgr
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic out to all IP addresses
  }
}

resource "aws_security_group" "vpc_app_ingress" {
  name        = "${var.app_name}-vpc-ingress-to-app"
  description = "Allow app port ingress from vpc"
  vpc_id      = var.vpc_id

  ingress { #tfsec:ignore:aws-ec2-add-description-to-security-group-rule
    from_port   = var.port
    to_port     = var.port
    protocol    = "tcp"
    cidr_blocks = var.allowed_ingress_cidr_blocks
  }

  egress {           #tfsec:ignore:aws-ec2-add-description-to-security-group-rule
    from_port = 0    # Allowing any incoming port
    to_port   = 0    # Allowing any outgoing port
    protocol  = "-1" # Allowing any outgoing protocol
    #tfsec:ignore:aws-ec2-no-public-egress-sgr
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic out to all IP addresses
  }
}
