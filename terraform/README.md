# Terraform Infrastructure

Get yourself some AWS creds and then init your workspace:

`terraform -chdir=terraform init -var-file="vars/dev.tfvars"`

Use the dev workspace:

`terraform -chdir=terraform workspace select dev`

Now you can apply the changes:

`terraform -chdir=terraform apply  -var-file="vars/dev.tfvars"`

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | >= 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | >= 5.7 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | >= 2.1 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.5.1 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | >= 5.7 |
| <a name="provider_random"></a> [random](#provider\_random) | 3.5.1 |
| <a name="provider_terraform"></a> [terraform](#provider\_terraform) | n/a |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_dns_certificate"></a> [dns\_certificate](#module\_dns\_certificate) | app.terraform.io/wallet-connect/dns/aws | 0.1.3 |
| <a name="module_ecs"></a> [ecs](#module\_ecs) | ./ecs | n/a |
| <a name="module_keystore"></a> [keystore](#module\_keystore) | ./docdb | n/a |
| <a name="module_monitoring"></a> [monitoring](#module\_monitoring) | ./monitoring | n/a |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |
| <a name="module_vpc"></a> [vpc](#module\_vpc) | terraform-aws-modules/vpc/aws | 5.1 |
| <a name="module_vpc_endpoints"></a> [vpc\_endpoints](#module\_vpc\_endpoints) | terraform-aws-modules/vpc/aws//modules/vpc-endpoints | 5.1 |
| <a name="module_vpc_flow_s3_bucket"></a> [vpc\_flow\_s3\_bucket](#module\_vpc\_flow\_s3\_bucket) | terraform-aws-modules/s3-bucket/aws | ~> 3.14 |

## Resources

| Name | Type |
|------|------|
| [aws_prometheus_workspace.prometheus](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/prometheus_workspace) | resource |
| [random_pet.this](https://registry.terraform.io/providers/hashicorp/random/3.5.1/docs/resources/pet) | resource |
| [aws_availability_zones.available](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/availability_zones) | data source |
| [aws_iam_policy_document.vpc_flow_log_s3](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/iam_policy_document) | data source |
| [terraform_remote_state.dns](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.monitoring](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |
| [terraform_remote_state.org](https://registry.terraform.io/providers/hashicorp/terraform/latest/docs/data-sources/remote_state) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_grafana_auth"></a> [grafana\_auth](#input\_grafana\_auth) | The API Token for the Grafana instance | `string` | `""` | no |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the image to deploy | `string` | n/a | yes |
| <a name="input_keystore_primary_instance_class"></a> [keystore\_primary\_instance\_class](#input\_keystore\_primary\_instance\_class) | The instance class of the primary docdb instances | `string` | n/a | yes |
| <a name="input_keystore_primary_instance_count"></a> [keystore\_primary\_instance\_count](#input\_keystore\_primary\_instance\_count) | The number of primary docdb instances to deploy | `number` | n/a | yes |
| <a name="input_keystore_replica_instance_class"></a> [keystore\_replica\_instance\_class](#input\_keystore\_replica\_instance\_class) | The instance class of the replica docdb instances | `string` | n/a | yes |
| <a name="input_keystore_replica_instance_count"></a> [keystore\_replica\_instance\_count](#input\_keystore\_replica\_instance\_count) | The number of replica docdb instances to deploy | `number` | n/a | yes |
| <a name="input_name"></a> [name](#input\_name) | The name of the application | `string` | `"keyserver"` | no |
| <a name="input_notification_channels"></a> [notification\_channels](#input\_notification\_channels) | The notification channels to send alerts to | `list(any)` | `[]` | no |
| <a name="input_region"></a> [region](#input\_region) | AWS region to deploy to | `string` | n/a | yes |

## Outputs

No outputs.
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
