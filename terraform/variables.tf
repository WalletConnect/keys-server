variable "region" {
  description = "AWS region to deploy to"
  type        = string
  default     = "eu-central-1"
}

variable "grafana_endpoint" {
  description = "The endpoint of the Grafana instance"
  type        = string
}

variable "image_version" {
  description = "The version of the image to deploy"
  type        = string
}

variable "keystore_docdb_primary_instances" {
  description = "The number of primary docdb instances to deploy"
  type        = number
}

variable "keystore_docdb_primary_instance_class" {
  description = "The instance class of the primary docdb instances"
  type        = string
}

variable "keystore_docdb_replica_instances" {
  description = "The number of replica docdb instances to deploy"
  type        = number
}

variable "keystore_docdb_replica_instance_class" {
  description = "The instance class of the replica docdb instances"
  type        = string
}
