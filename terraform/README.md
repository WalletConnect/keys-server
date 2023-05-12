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
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_assert"></a> [assert](#requirement\_assert) | ~> 0.0.1 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 3.27 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | ~> 1.24 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_assert"></a> [assert](#provider\_assert) | ~> 0.0.1 |
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 3.27 |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_dns"></a> [dns](#module\_dns) | github.com/WalletConnect/terraform-modules/modules/dns | n/a |
| <a name="module_ecs"></a> [ecs](#module\_ecs) | ./ecs | n/a |
| <a name="module_keystore-docdb"></a> [keystore-docdb](#module\_keystore-docdb) | ./docdb | n/a |
| <a name="module_o11y"></a> [o11y](#module\_o11y) | ./monitoring | n/a |
| <a name="module_tags"></a> [tags](#module\_tags) | github.com/WalletConnect/terraform-modules/modules/tags | n/a |

## Resources

| Name | Type |
|------|------|
| [aws_prometheus_workspace.prometheus](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/prometheus_workspace) | resource |
| [assert_test.workspace](https://registry.terraform.io/providers/bwoznicki/assert/latest/docs/data-sources/test) | data source |
| [aws_ecr_repository.repository](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/ecr_repository) | data source |
| [aws_subnets.private_subnets](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/subnets) | data source |
| [aws_subnets.public_subnets](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/subnets) | data source |
| [aws_vpc.vpc](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/vpc) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_grafana_endpoint"></a> [grafana\_endpoint](#input\_grafana\_endpoint) | The endpoint of the Grafana instance | `string` | n/a | yes |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | The version of the image to deploy | `string` | n/a | yes |
| <a name="input_keystore_docdb_primary_instance_class"></a> [keystore\_docdb\_primary\_instance\_class](#input\_keystore\_docdb\_primary\_instance\_class) | The instance class of the primary docdb instances | `string` | n/a | yes |
| <a name="input_keystore_docdb_primary_instances"></a> [keystore\_docdb\_primary\_instances](#input\_keystore\_docdb\_primary\_instances) | The number of primary docdb instances to deploy | `number` | n/a | yes |
| <a name="input_keystore_docdb_replica_instance_class"></a> [keystore\_docdb\_replica\_instance\_class](#input\_keystore\_docdb\_replica\_instance\_class) | The instance class of the replica docdb instances | `string` | n/a | yes |
| <a name="input_keystore_docdb_replica_instances"></a> [keystore\_docdb\_replica\_instances](#input\_keystore\_docdb\_replica\_instances) | The number of replica docdb instances to deploy | `number` | n/a | yes |
| <a name="input_region"></a> [region](#input\_region) | AWS region to deploy to | `string` | `"eu-central-1"` | no |

## Outputs

No outputs.
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
