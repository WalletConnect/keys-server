#-------------------------------------------------------------------------------
# Configuration

variable "grafana_auth" {
  description = "The API Token for the Grafana instance"
  type        = string
  default     = ""
}

#-------------------------------------------------------------------------------
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

variable "log_level" {
  description = "Defines logging level for the application"
  type        = string
}

variable "ofac_blocked_countries" {
  description = "The list of countries to block"
  type        = string
  default     = ""
}

#-------------------------------------------------------------------------------
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

#-------------------------------------------------------------------------------
# Monitoring

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
  default     = []
}

variable "betterstack_prometheus_webhook" {
  description = "The BetterStack webhook to send Prometheus alerts to"
  type        = string
  sensitive   = true
}

variable "betterstack_cloudwatch_webhook" {
  description = "The BetterStack webhook to send CloudWatch alerts to"
  type        = string
  sensitive   = true
}

#---------------------------------------
# GeoIP

variable "geoip_db_key" {
  description = "The name to the GeoIP database"
  type        = string
  default     = "GeoLite2-City.mmdb"
}
