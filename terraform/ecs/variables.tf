// Cluster settings

variable "app_name" {
  description = "The name of the app"
  type        = string
}

variable "region" {
  description = "The AWS region to deploy to"
  type        = string
}

variable "environment" {
  description = "The environment to deploy to"
  type        = string
}

variable "ecr_repository_url" {
  description = "The URL of the ECR repository where the app image is stored"
  type        = string
}

variable "image_version" {
  description = "The version of the app image to deploy"
  type        = string
}

variable "task_cpu" {
  description = "The number of CPU units to reserve for the container."
  type        = number
}

variable "task_memory" {
  description = "The amount of memory (in MiB) to reserve for the container."
  type        = number
}

variable "min_capacity" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "max_capacity" {
  description = "Maximum number of instances in the autoscaling group"
  type        = number
  default     = 8
}

variable "port" {
  description = "The port the app listens on"
  type        = number
}

// DNS settings

variable "acm_certificate_arn" {
  description = "The ARN of the ACM certificate to use for HTTPS"
  type        = string
}

variable "route53-fqdn" {
  description = "The FQDN to use for the app"
  type        = string
}

variable "route53_zone_id" {
  description = "The ID of the Route53 zone to use for the app"
  type        = string
}

// Network settings

variable "vpc_id" {
  description = "The ID of the VPC to deploy to"
  type        = string
}

variable "public_subnets" {
  description = "The IDs of the public subnets to deploy to"
  type        = list(string)
}

variable "private_subnets" {
  description = "The IDs of the private subnets to deploy to"
  type        = list(string)
}

variable "allowed_app_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the application."
  type        = string
}

variable "allowed_lb_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the load-balancer."
  type        = string
}

// Application settings

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus server to use for monitoring"
  type        = string
}

variable "persistent_keystore_mongo_addr" {
  description = "The address of the MongoDB instance to use for the persistent keystore"
  type        = string
}
