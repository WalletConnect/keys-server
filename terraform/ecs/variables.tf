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

variable "ecr_repository_url" {
  description = "The URL of the ECR repository where the app image is stored"
  type        = string
}

variable "image_version" {
  description = "The version of the app image to deploy"
  type        = string
}

variable "app_name" {
  description = "The name of the app"
  type        = string
}

variable "region" {
  description = "The AWS region to deploy to"
  type        = string
}

variable "vpc_id" {
  description = "The ID of the VPC to deploy to"
  type        = string
}

variable "private_subnet_ids" {
  description = "The IDs of the private subnets to deploy to"
  type        = list(string)
}

variable "public_subnet_ids" {
  description = "The IDs of the public subnets to deploy to"
  type        = list(string)
}

variable "allowed_ingress_cidr_blocks" {
  description = "The CIDR blocks to allow ingress from"
  type        = list(string)
}

variable "port" {
  description = "The port the app listens on"
  type        = number
}

variable "acm_certificate_arn" {
  description = "The ARN of the ACM certificate to use for HTTPS"
  type        = string
}

variable "fqdn" {
  description = "The FQDN to use for the app"
  type        = string
}

variable "route53_zone_id" {
  description = "The ID of the Route53 zone to use for the app"
  type        = string
}

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus server to use for monitoring"
  type        = string
}

variable "persistent_keystore_mongo_addr" {
  description = "The address of the MongoDB instance to use for the persistent keystore"
  type        = string
}
