variable "min_capacity" {
  type    = number
  default = 2
}

variable "max_capacity" {
  type    = number
  default = 8
}

variable "ecr_repository_url" {
  type = string
}

variable "image_version" {
  type = string
}

variable "app_name" {
  type = string
}

variable "region" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "private_subnet_ids" {
  type = list(string)
}

variable "public_subnet_ids" {
  type = list(string)
}

variable "allowed_ingress_cidr_blocks" {
  type = list(string)
}

variable "port" {
  type = number
}

variable "acm_certificate_arn" {
  type = string
}

variable "fqdn" {
  type = string
}

variable "route53_zone_id" {
  type = string
}

variable "prometheus_endpoint" {
  type = string
}

variable "persistent_keystore_mongo_addr" {
  type = string
}
