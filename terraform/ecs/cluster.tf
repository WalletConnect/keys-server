locals {
  otel_cpu    = 128
  otel_memory = 128
}

# ECS Cluster
resource "aws_ecs_cluster" "app_cluster" {
  name = "${local.app_name}_cluster"

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_encryption_enabled = false
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.cluster_logs.name
      }
    }
  }

  # Exposes metrics such as the number of running tasks in CloudWatch
  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

# ECS Service
resource "aws_ecs_service" "app_service" {
  name            = "${local.app_name}-service"
  cluster         = aws_ecs_cluster.app_cluster.id
  task_definition = aws_ecs_task_definition.app_task.arn
  launch_type     = "FARGATE"
  desired_count   = var.min_capacity
  propagate_tags  = "TASK_DEFINITION"

  # Wait for the service deployment to succeed
  wait_for_steady_state = true

  network_configuration {
    subnets = var.private_subnets
    assign_public_ip = false
    security_groups  = [aws_security_group.app_ingress.id] # Setting the security group
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

# ECS Task definition
resource "aws_ecs_task_definition" "app_task" {
  family                   = local.app_name
  memory                   = var.task_memory
  cpu                      = var.task_cpu
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc" # Using awsvpc as our network mode as this is required for Fargate
  execution_role_arn       = aws_iam_role.ecs_task_execution_role.arn
  task_role_arn            = aws_iam_role.ecs_task_execution_role.arn

  container_definitions = jsonencode([
    {
      name      = local.app_name,
      image     = local.image,
      essential = true,
      cpu       = var.task_cpu - local.otel_cpu,
      memory    = var.task_memory - local.otel_memory,

      portMappings = [
        {
          containerPort = var.port,
          hostPort      = var.port
        }
      ],

      environment = [
        { "name" = "DATABASE_URL", "value" = var.persistent_keystore_mongo_addr }
      ],

      logConfiguration : {
        logDriver = "awslogs",
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.cluster_logs.name,
          "awslogs-region"        = var.region,
          "awslogs-stream-prefix" = "ecs"
        }
      },

      dependsOn = [
        { containerName : "aws-otel-collector", condition : "START" }
      ]
    },

    {
      name      = "aws-otel-collector",
      image     = "public.ecr.aws/aws-observability/aws-otel-collector:latest",
      essential = true,
      cpu       = local.otel_cpu,
      memory    = local.otel_memory,

      command = [
        "--config=/etc/ecs/ecs-amp-prometheus.yaml"
      ],

      environment = [
        { name : "AWS_PROMETHEUS_SCRAPING_ENDPOINT", value : "0.0.0.0:${var.port}" },
        { name : "AWS_PROMETHEUS_ENDPOINT", value : "${var.prometheus_endpoint}api/v1/remote_write" },
        { name = "AWS_REGION", value = "eu-central-1" },
      ],

      logConfiguration = {
        logDriver = "awslogs",
        options = {
          "awslogs-create-group"  = "True",
          "awslogs-group"         = "/ecs/${local.app_name}-ecs-aws-otel-sidecar-collector",
          "awslogs-region"        = var.region,
          "awslogs-stream-prefix" = "ecs"
        }
      }
    }
  ])

  runtime_platform {
    operating_system_family = "LINUX"
  }
}
