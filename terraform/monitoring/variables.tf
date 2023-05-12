variable "environment" {
  type = string
}

variable "prometheus_workspace_id" {
  description = "The workspace ID for the Prometheus workspace."
  type        = string
}

variable "ecs_service_name" {
  type = string
}

variable "target_group" {
  type = string
}

variable "load_balancer" {
  type = string
}

variable "docdb_cluster_id" {
  type = string
}
