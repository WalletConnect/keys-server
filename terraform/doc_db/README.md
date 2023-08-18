# `docdb` module

Creates a DocumentDB cluster with auto-scaled read replicas.

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 3.27 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.4.3 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 3.27 |
| <a name="provider_random"></a> [random](#provider\_random) | 3.4.3 |

## Modules

No modules.

## Resources

| Name | Type |
|------|------|
| [aws_docdb_cluster.docdb](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/docdb_cluster) | resource |
| [aws_docdb_cluster_instance.docdb_instances](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/docdb_cluster_instance) | resource |
| [aws_docdb_cluster_instance.docdb_replica_instances](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/docdb_cluster_instance) | resource |
| [aws_docdb_subnet_group.private_subnets](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/docdb_subnet_group) | resource |
| [aws_secretsmanager_secret.master_password](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/secretsmanager_secret) | resource |
| [aws_secretsmanager_secret_version.master_password](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/secretsmanager_secret_version) | resource |
| [aws_security_group.service_security_group](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group) | resource |
| [random_password.master_password](https://registry.terraform.io/providers/hashicorp/random/3.4.3/docs/resources/password) | resource |
| [aws_kms_key.docdb_encryption](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/kms_key) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_allowed_egress_cidr_blocks"></a> [allowed\_egress\_cidr\_blocks](#input\_allowed\_egress\_cidr\_blocks) | The CIDR blocks to allow egress to | `list(string)` | n/a | yes |
| <a name="input_allowed_ingress_cidr_blocks"></a> [allowed\_ingress\_cidr\_blocks](#input\_allowed\_ingress\_cidr\_blocks) | The CIDR blocks to allow ingress from | `list(string)` | n/a | yes |
| <a name="input_app_name"></a> [app\_name](#input\_app\_name) | The name of the application | `string` | n/a | yes |
| <a name="input_default_database"></a> [default\_database](#input\_default\_database) | The name of the default database to create | `string` | n/a | yes |
| <a name="input_environment"></a> [environment](#input\_environment) | The environment to deploy to | `string` | n/a | yes |
| <a name="input_mongo_name"></a> [mongo\_name](#input\_mongo\_name) | The name of the mongo database | `string` | n/a | yes |
| <a name="input_primary_instance_class"></a> [primary\_instance\_class](#input\_primary\_instance\_class) | The instance class of the primary instances | `string` | n/a | yes |
| <a name="input_primary_instances"></a> [primary\_instances](#input\_primary\_instances) | The number of primary instances to create | `number` | n/a | yes |
| <a name="input_private_subnet_ids"></a> [private\_subnet\_ids](#input\_private\_subnet\_ids) | The IDs of the private subnets to deploy to | `list(string)` | n/a | yes |
| <a name="input_replica_instance_class"></a> [replica\_instance\_class](#input\_replica\_instance\_class) | The instance class of the replica instances | `string` | n/a | yes |
| <a name="input_replica_instances"></a> [replica\_instances](#input\_replica\_instances) | The number of replica instances to create | `number` | n/a | yes |
| <a name="input_vpc_id"></a> [vpc\_id](#input\_vpc\_id) | The ID of the VPC to deploy to | `string` | n/a | yes |

## Outputs

| Name | Description |
|------|-------------|
| <a name="output_cluster_id"></a> [cluster\_id](#output\_cluster\_id) | The cluster identifier |
| <a name="output_connection_url"></a> [connection\_url](#output\_connection\_url) | The connection url |
| <a name="output_endpoint"></a> [endpoint](#output\_endpoint) | The connection endpoint |
| <a name="output_password"></a> [password](#output\_password) | The master password |
| <a name="output_port"></a> [port](#output\_port) | The connection port |
| <a name="output_username"></a> [username](#output\_username) | The master username |
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
