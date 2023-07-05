locals {
  app_name = "${var.environment}_${var.app_name}"
  image    = "${var.ecr_repository_url}:${var.image_version}"
}
