variable "environment" {
  description = "The environment to deploy to"
  type        = string
}

variable "app_name" {
  description = "The name of the application"
  type        = string
}

variable "mongo_name" {
  description = "The name of the mongo database"
  type        = string
}

variable "primary_instances" {
  description = "The number of primary instances to create"
  type        = number
}

variable "primary_instance_class" {
  description = "The instance class of the primary instances"
  type        = string
}

variable "replica_instances" {
  description = "The number of replica instances to create"
  type        = number
}

variable "replica_instance_class" {
  description = "The instance class of the replica instances"
  type        = string
}

variable "allowed_ingress_cidr_blocks" {
  description = "The CIDR blocks to allow ingress from"
  type        = list(string)
}

variable "allowed_egress_cidr_blocks" {
  description = "The CIDR blocks to allow egress to"
  type        = list(string)
}

variable "vpc_id" {
  description = "The ID of the VPC to deploy to"
  type        = string
}

variable "private_subnet_ids" {
  description = "The IDs of the private subnets to deploy to"
  type        = list(string)
}

variable "default_database" {
  description = "The name of the default database to create"
  type        = string
}
