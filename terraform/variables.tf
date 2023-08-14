########################################
# Configuration

variable "grafana_auth" {
  description = "The API Token for the Grafana instance"
  type        = string
  default     = ""
}

########################################
# Application

variable "name" {
  description = "The name of the application"
  type        = string
  default     = "keyserver"
}

variable "region" {
  description = "AWS region to deploy to"
  type        = string
}

variable "image_version" {
  description = "The version of the image to deploy"
  type        = string
}


########################################
# Keystore

variable "keystore_primary_instance_count" {
  description = "The number of primary docdb instances to deploy"
  type        = number
}

variable "keystore_primary_instance_class" {
  description = "The instance class of the primary docdb instances"
  type        = string
}

variable "keystore_replica_instance_count" {
  description = "The number of replica docdb instances to deploy"
  type        = number
}

variable "keystore_replica_instance_class" {
  description = "The instance class of the replica docdb instances"
  type        = string
}

########################################
# Monitoring

variable "grafana_endpoint" {
  description = "The endpoint of the Grafana instance"
  type        = string
}
