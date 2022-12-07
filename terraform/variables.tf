variable "region" {
  type    = string
  default = "eu-central-1"
}

variable "grafana_endpoint" {
  type = string
}



variable "keystore_docdb_primary_instance_class" {
  type = string
}

variable "keystore_docdb_primary_instances" {
  type = number
}
