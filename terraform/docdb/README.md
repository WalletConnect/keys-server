# `docdb` module

Creates a DocumentDB cluster with auto-scaled read replicas.

<!-- BEGIN_TF_DOCS -->

## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 5.7 |
| <a name="requirement_random"></a> [random](#requirement\_random) | ~> 3.5 |
## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 5.7 |
| <a name="provider_random"></a> [random](#provider\_random) | ~> 3.5 |
## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.3.2 |

## Inputs
| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_allowed_egress_cidr_blocks"></a> [allowed\_egress\_cidr\_blocks](#input\_allowed\_egress\_cidr\_blocks) | The CIDR blocks to allow egress to |  <pre lang="json">list(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_allowed_ingress_cidr_blocks"></a> [allowed\_ingress\_cidr\_blocks](#input\_allowed\_ingress\_cidr\_blocks) | The CIDR blocks to allow ingress from |  <pre lang="json">list(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_context"></a> [context](#input\_context) | Single object for setting entire context at once.<br>See description of individual variables for details.<br>Leave string and numeric variables as `null` to use default value.<br>Individual variable settings (non-null) override settings in context object,<br>except for attributes and tags, which are merged. |  <pre lang="json">any</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_db_name"></a> [db\_name](#input\_db\_name) | The name of the mongo database |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_default_database"></a> [default\_database](#input\_default\_database) | The name of the default database to create |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_port"></a> [port](#input\_port) | The port the mongo database will listen on |  <pre lang="json">number</pre> |  <pre lang="json">27017</pre> |  no |
| <a name="input_primary_instance_class"></a> [primary\_instance\_class](#input\_primary\_instance\_class) | The instance class of the primary instances |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_primary_instance_count"></a> [primary\_instance\_count](#input\_primary\_instance\_count) | The number of primary instances to create |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_private_subnet_ids"></a> [private\_subnet\_ids](#input\_private\_subnet\_ids) | The IDs of the private subnets to deploy to |  <pre lang="json">list(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_replica_instance_class"></a> [replica\_instance\_class](#input\_replica\_instance\_class) | The instance class of the replica instances |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_replica_instance_count"></a> [replica\_instance\_count](#input\_replica\_instance\_count) | The number of replica instances to create |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
| <a name="input_vpc_id"></a> [vpc\_id](#input\_vpc\_id) | The ID of the VPC to deploy to |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
## Outputs

| Name | Description |
|------|-------------|
| <a name="output_cluster_id"></a> [cluster\_id](#output\_cluster\_id) | The cluster identifier |
| <a name="output_connection_url"></a> [connection\_url](#output\_connection\_url) | The connection url |
| <a name="output_endpoint"></a> [endpoint](#output\_endpoint) | The connection endpoint |
| <a name="output_password"></a> [password](#output\_password) | The master password |
| <a name="output_port"></a> [port](#output\_port) | The connection port |
| <a name="output_username"></a> [username](#output\_username) | The master username |


<!-- END_TF_DOCS -->
