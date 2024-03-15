#---------------------------------------
# Cluster

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

variable "autoscaling_desired_count" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_min_capacity" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_max_capacity" {
  description = "Maximum number of instances in the autoscaling group"
  type        = number
  default     = 8
}

#---------------------------------------
# DNS

variable "route53_zones" {
  description = "The FQDNs to use for the app"
  type        = map(string)
}

variable "route53_zones_certificates" {
  description = "The ARNs of the ACM certificates to use for HTTPS"
  type        = map(string)
}

#---------------------------------------
# Network

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
#---------------------------------------
# Application

variable "port" {
  description = "The port the app listens on"
  type        = number
}

variable "keystore_addr" {
  description = "The address of the MongoDB instance to use for the persistent keystore"
  type        = string
}

variable "log_level" {
  description = "Defines logging level for the application"
  type        = string
}

variable "ofac_blocked_countries" {
  description = "The list of countries to block"
  type        = string
}

variable "project_id" {
  description = "Project ID for Blockchain API"
  type        = string
}

#---------------------------------------
# Monitoring

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus server to use for monitoring"
  type        = string
}

#---------------------------------------
# GeoIP

variable "geoip_db_bucket_name" {
  description = "The name of the S3 bucket where the GeoIP database is stored"
  type        = string
}

variable "geoip_db_key" {
  description = "The key of the GeoIP database in the S3 bucket"
  type        = string
}
