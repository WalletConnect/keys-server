variable "webhook_url" {
  description = "The URL of the webhook to be called on alarms"
  type        = string
}

#-------------------------------------------------------------------------------
# ECS

variable "ecs_cluster_name" {
  description = "The name of the ECS cluster running the application"
  type        = string
}

variable "ecs_service_name" {
  description = "The name of the ECS service running the application"
  type        = string
}

variable "ecs_cpu_threshold" {
  description = "The ECS CPU utilization alarm threshold in percents"
  type        = number
  default     = 80
}

variable "ecs_memory_threshold" {
  description = "The ECS memory utilization alarm threshold in percents"
  type        = number
  default     = 80
}

#-------------------------------------------------------------------------------
# DocumentDB

variable "docdb_cluster_id" {
  description = "The DocumentDB cluster ID"
  type        = string
}

variable "docdb_cpu_threshold" {
  description = "The DocumentDB CPU utilization alarm threshold in percents"
  type        = number
  default     = 80
}

variable "docdb_memory_threshold" {
  description = "The DocumentDB available memory alarm threshold in GiB"
  type        = number
  default     = 3
}

variable "docdb_low_memory_throttling_threshold" {
  description = "The DocumentDB low memory throttling alarm threshold in number of operations per period"
  type        = number
  default     = 2
}
