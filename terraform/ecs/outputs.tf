output "service_name" {
  description = "The name of the service"
  value       = aws_ecs_service.app_service.name
}

output "service_security_group_id" {
  description = "The ID of the security group for the service"
  value       = aws_security_group.app_ingress.id
}

output "target_group_arn" {
  description = "The ARN of the target group"
  value       = aws_lb_target_group.target_group.arn
}

output "load_balancer_arn" {
  description = "The ARN of the load balancer"
  value       = aws_lb.load_balancer.arn
}

output "load_balancer_arn_suffix" {
  description = "The ARN suffix of the load balancer"
  value       = aws_lb.load_balancer.arn_suffix
}

output "ecs_task_family" {
  description = "The family of the task definition"
  value       = aws_ecs_task_definition.app_task.family
}
