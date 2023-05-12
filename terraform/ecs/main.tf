locals {
  image = "${var.ecr_repository_url}:${var.image_version}"
}

# Log Group for our App
#tfsec:ignore:aws-cloudwatch-log-group-customer-key
resource "aws_cloudwatch_log_group" "cluster_logs" {
  name              = "${var.app_name}_logs"
  retention_in_days = 14
  # TODO: Enable CMK encryption of CloudWatch Log Groups:
  #  kms_key_id = aws_kms_key.log_key.arn
}

# ECS Cluster
resource "aws_ecs_cluster" "app_cluster" {
  name = "${var.app_name}_cluster"

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_encryption_enabled = false
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.cluster_logs.name
      }
    }
  }

  # Exposes metrics such as the
  # number of running tasks
  # in CloudWatch
  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

# ECS Task definition
resource "aws_ecs_task_definition" "app_task" {
  family = var.app_name
  container_definitions = jsonencode([
    {
      name : var.app_name,
      environment : [
        { "name" : "DATABASE_URL", "value" : var.persistent_keystore_mongo_addr }
      ],
      image : local.image,
      essential : true,
      portMappings : [
        {
          containerPort : var.port,
          hostPort : var.port
        }
      ],
      memory : 512,
      cpu : 256,
      logConfiguration : {
        logDriver : "awslogs",
        options : {
          "awslogs-group" : aws_cloudwatch_log_group.cluster_logs.name,
          "awslogs-region" : var.region,
          "awslogs-stream-prefix" : "ecs"
        }
      },
      dependsOn : [
        {
          containerName : "aws-otel-collector",
          condition : "START"
        }
      ]
    },
    {
      name : "aws-otel-collector",
      image : "public.ecr.aws/aws-observability/aws-otel-collector:latest",
      environment : [
        { name : "AWS_PROMETHEUS_SCRAPING_ENDPOINT", value : "0.0.0.0:${var.port}" },
        { name : "AWS_PROMETHEUS_ENDPOINT", value : "${var.prometheus_endpoint}api/v1/remote_write" }
      ],
      essential : true,
      command : [
        "--config=/etc/ecs/ecs-amp-prometheus.yaml"
      ],
      logConfiguration : {
        logDriver : "awslogs",
        options : {
          "awslogs-create-group" : "True",
          "awslogs-group" : "/ecs/${var.app_name}-ecs-aws-otel-sidecar-collector",
          "awslogs-region" : var.region,
          "awslogs-stream-prefix" : "ecs"
        }
      }
    }
  ])

  requires_compatibilities = ["FARGATE"] # Stating that we are using ECS Fargate
  network_mode             = "awsvpc"    # Using awsvpc as our network mode as this is required for Fargate
  memory                   = 512         # Specifying the memory our container requires
  cpu                      = 256         # Specifying the CPU our container requires
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn
  task_role_arn            = aws_iam_role.ecs_task_execution_role.arn

  runtime_platform {
    operating_system_family = "LINUX"
  }
}

resource "aws_iam_role" "ecs_task_execution_role" {
  name               = "${var.app_name}_ecs_task_execution_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}

data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_iam_role_policy_attachment" "cloudwatch_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess"
}

resource "aws_iam_role_policy_attachment" "prometheus_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonPrometheusRemoteWriteAccess"
}

# ECS Service
resource "aws_ecs_service" "app_service" {
  name            = "${var.app_name}-service"
  cluster         = aws_ecs_cluster.app_cluster.id
  task_definition = join(":", slice(split(":", aws_ecs_task_definition.app_task.arn), 0, 6))
  launch_type     = "FARGATE"
  desired_count   = var.min_capacity

  # Wait for the service deployment to succeed
  wait_for_steady_state = true

  network_configuration {
    subnets          = var.private_subnet_ids
    assign_public_ip = false                                                                     # We do public ingress through the LB
    security_groups  = [aws_security_group.tls_ingess.id, aws_security_group.vpc_app_ingress.id] # Setting the security group
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.target_group.arn
    container_name   = aws_ecs_task_definition.app_task.family
    container_port   = var.port
  }

  # Allow external changes without Terraform plan difference
  lifecycle {
    ignore_changes = [desired_count]
  }
}
