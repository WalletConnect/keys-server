# `monitoring` module

Configure the Grafana dashboards for the application

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | ~> 2.0 |
| <a name="requirement_jsonnet"></a> [jsonnet](#requirement\_jsonnet) | ~> 2.2.0 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_grafana"></a> [grafana](#provider\_grafana) | ~> 2.0 |
| <a name="provider_jsonnet"></a> [jsonnet](#provider\_jsonnet) | ~> 2.2.0 |

## Modules

No modules.

## Resources

| Name | Type |
|------|------|
| [grafana_dashboard.main](https://registry.terraform.io/providers/grafana/grafana/latest/docs/resources/dashboard) | resource |
| [grafana_data_source.cloudwatch](https://registry.terraform.io/providers/grafana/grafana/latest/docs/resources/data_source) | resource |
| [grafana_data_source.prometheus](https://registry.terraform.io/providers/grafana/grafana/latest/docs/resources/data_source) | resource |
| [jsonnet_file.dashboard](https://registry.terraform.io/providers/alxrem/jsonnet/latest/docs/data-sources/file) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_docdb_cluster_id"></a> [docdb\_cluster\_id](#input\_docdb\_cluster\_id) | n/a | `string` | n/a | yes |
| <a name="input_ecs_service_name"></a> [ecs\_service\_name](#input\_ecs\_service\_name) | n/a | `string` | n/a | yes |
| <a name="input_environment"></a> [environment](#input\_environment) | n/a | `string` | n/a | yes |
| <a name="input_load_balancer"></a> [load\_balancer](#input\_load\_balancer) | n/a | `string` | n/a | yes |
| <a name="input_prometheus_workspace_id"></a> [prometheus\_workspace\_id](#input\_prometheus\_workspace\_id) | The workspace ID for the Prometheus workspace. | `string` | n/a | yes |
| <a name="input_target_group"></a> [target\_group](#input\_target\_group) | n/a | `string` | n/a | yes |

## Outputs

| Name | Description |
|------|-------------|
| <a name="output_dashboard_definition"></a> [dashboard\_definition](#output\_dashboard\_definition) | The JSON definition of the dashboard. |
| <a name="output_prometheus_url"></a> [prometheus\_url](#output\_prometheus\_url) | The URL of the Prometheus server to use for this dashboard. |
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
