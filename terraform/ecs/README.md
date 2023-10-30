# `ecs` module

This module creates an ECS cluster and an autoscaling group of EC2 instances to run the application.

<!-- BEGIN_TF_DOCS -->

## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 5.7 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.5.1 |
## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 5.7 |
| <a name="provider_random"></a> [random](#provider\_random) | 3.5.1 |
## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_ecs_cpu_mem"></a> [ecs\_cpu\_mem](#module\_ecs\_cpu\_mem) | app.terraform.io/wallet-connect/ecs_cpu_mem/aws | 1.0.0 |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |

## Inputs
| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_allowed_app_ingress_cidr_blocks"></a> [allowed\_app\_ingress\_cidr\_blocks](#input\_allowed\_app\_ingress\_cidr\_blocks) | A list of CIDR blocks to allow ingress access to the application. |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_allowed_lb_ingress_cidr_blocks"></a> [allowed\_lb\_ingress\_cidr\_blocks](#input\_allowed\_lb\_ingress\_cidr\_blocks) | A list of CIDR blocks to allow ingress access to the load-balancer. |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_context"></a> [context](#input\_context) | Single object for setting entire context at once.<br>See description of individual variables for details.<br>Leave string and numeric variables as `null` to use default value.<br>Individual variable settings (non-null) override settings in context object,<br>except for attributes and tags, which are merged. |  <pre lang="json">any</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_ecr_repository_url"></a> [ecr\_repository\_url](#input\_ecr\_repository\_url) | The URL of the ECR repository where the app image is stored |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_geoip_db_bucket_name"></a> [geoip\_db\_bucket\_name](#input\_geoip\_db\_bucket\_name) | The name of the S3 bucket where the GeoIP database is stored |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_geoip_db_key"></a> [geoip\_db\_key](#input\_geoip\_db\_key) | The key of the GeoIP database in the S3 bucket |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the app image to deploy |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_keystore_addr"></a> [keystore\_addr](#input\_keystore\_addr) | The address of the MongoDB instance to use for the persistent keystore |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_log_level"></a> [log\_level](#input\_log\_level) | Defines logging level for the application |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_max_capacity"></a> [max\_capacity](#input\_max\_capacity) | Maximum number of instances in the autoscaling group |  <pre lang="json">number</pre> |  <pre lang="json">8</pre> |  no |
| <a name="input_min_capacity"></a> [min\_capacity](#input\_min\_capacity) | Minimum number of instances in the autoscaling group |  <pre lang="json">number</pre> |  <pre lang="json">2</pre> |  no |
| <a name="input_ofac_blocked_countries"></a> [ofac\_blocked\_countries](#input\_ofac\_blocked\_countries) | The list of countries to block |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_port"></a> [port](#input\_port) | The port the app listens on |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_private_subnets"></a> [private\_subnets](#input\_private\_subnets) | The IDs of the private subnets to deploy to |  <pre lang="json">list(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_prometheus_endpoint"></a> [prometheus\_endpoint](#input\_prometheus\_endpoint) | The endpoint of the Prometheus server to use for monitoring |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_public_subnets"></a> [public\_subnets](#input\_public\_subnets) | The IDs of the public subnets to deploy to |  <pre lang="json">list(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_route53_zones"></a> [route53\_zones](#input\_route53\_zones) | The FQDNs to use for the app |  <pre lang="json">map(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_route53_zones_certificates"></a> [route53\_zones\_certificates](#input\_route53\_zones\_certificates) | The ARNs of the ACM certificates to use for HTTPS |  <pre lang="json">map(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_task_cpu"></a> [task\_cpu](#input\_task\_cpu) | The number of CPU units to reserve for the container. |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_task_memory"></a> [task\_memory](#input\_task\_memory) | The amount of memory (in MiB) to reserve for the container. |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_vpc_id"></a> [vpc\_id](#input\_vpc\_id) | The ID of the VPC to deploy to |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
## Outputs

| Name | Description |
|------|-------------|
| <a name="output_ecs_cluster_name"></a> [ecs\_cluster\_name](#output\_ecs\_cluster\_name) | The name of the ECS cluster |
| <a name="output_ecs_service_name"></a> [ecs\_service\_name](#output\_ecs\_service\_name) | The name of the ECS service |
| <a name="output_ecs_task_family"></a> [ecs\_task\_family](#output\_ecs\_task\_family) | The family of the task definition |
| <a name="output_load_balancer_arn"></a> [load\_balancer\_arn](#output\_load\_balancer\_arn) | The ARN of the load balancer |
| <a name="output_load_balancer_arn_suffix"></a> [load\_balancer\_arn\_suffix](#output\_load\_balancer\_arn\_suffix) | The ARN suffix of the load balancer |
| <a name="output_service_name"></a> [service\_name](#output\_service\_name) | The name of the service |
| <a name="output_service_security_group_id"></a> [service\_security\_group\_id](#output\_service\_security\_group\_id) | The ID of the security group for the service |
| <a name="output_target_group_arn"></a> [target\_group\_arn](#output\_target\_group\_arn) | The ARN of the target group |


<!-- END_TF_DOCS -->
