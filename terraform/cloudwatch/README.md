# `cloudwatch` module

This module configures the cloudwatch alarms and webhook forwarding.

<!-- BEGIN_TF_DOCS -->

## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 5.7 |
## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 5.7 |
## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_cloudwatch"></a> [cloudwatch](#module\_cloudwatch) | app.terraform.io/wallet-connect/cloudwatch-constants/aws | 1.0.0 |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |

## Inputs
| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_context"></a> [context](#input\_context) | Single object for setting entire context at once.<br>See description of individual variables for details.<br>Leave string and numeric variables as `null` to use default value.<br>Individual variable settings (non-null) override settings in context object,<br>except for attributes and tags, which are merged. |  <pre lang="json">any</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_docdb_cluster_id"></a> [docdb\_cluster\_id](#input\_docdb\_cluster\_id) | The DocumentDB cluster ID |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_docdb_cpu_threshold"></a> [docdb\_cpu\_threshold](#input\_docdb\_cpu\_threshold) | The DocumentDB CPU utilization alarm threshold in percents |  <pre lang="json">number</pre> |  <pre lang="json">80</pre> |  no |
| <a name="input_docdb_low_memory_throttling_threshold"></a> [docdb\_low\_memory\_throttling\_threshold](#input\_docdb\_low\_memory\_throttling\_threshold) | The DocumentDB low memory throttling alarm threshold in number of operations per period |  <pre lang="json">number</pre> |  <pre lang="json">2</pre> |  no |
| <a name="input_docdb_memory_threshold"></a> [docdb\_memory\_threshold](#input\_docdb\_memory\_threshold) | The DocumentDB available memory alarm threshold in GiB |  <pre lang="json">number</pre> |  <pre lang="json">4</pre> |  no |
| <a name="input_ecs_cluster_name"></a> [ecs\_cluster\_name](#input\_ecs\_cluster\_name) | The name of the ECS cluster running the application |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_ecs_cpu_threshold"></a> [ecs\_cpu\_threshold](#input\_ecs\_cpu\_threshold) | The ECS CPU utilization alarm threshold in percents |  <pre lang="json">number</pre> |  <pre lang="json">80</pre> |  no |
| <a name="input_ecs_memory_threshold"></a> [ecs\_memory\_threshold](#input\_ecs\_memory\_threshold) | The ECS memory utilization alarm threshold in percents |  <pre lang="json">number</pre> |  <pre lang="json">80</pre> |  no |
| <a name="input_ecs_service_name"></a> [ecs\_service\_name](#input\_ecs\_service\_name) | The name of the ECS service running the application |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_webhook_url"></a> [webhook\_url](#input\_webhook\_url) | The URL of the webhook to be called on alarms |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
## Outputs

No outputs.


<!-- END_TF_DOCS -->
