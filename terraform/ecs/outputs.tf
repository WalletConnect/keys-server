output "service_security_group_id" {
  value = aws_security_group.vpc_app_ingress.id
}

output "target_group_arn" {
  value = aws_lb_target_group.target_group.arn
}

output "load_balancer_arn" {
  description = "The ARN of the load balancer"
  value       = aws_alb.network_load_balancer.arn
}

output "load_balancer_arn_suffix" {
  description = "The ARN suffix of the load balancer"
  value       = aws_alb.network_load_balancer.arn_suffix
}

output "service_name" {
  description = "The name of the service"
  value       = aws_ecs_service.app_service.name
}
