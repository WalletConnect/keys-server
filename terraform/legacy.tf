data "aws_vpc" "vpc" {
  filter {
    name   = "tag:Name"
    values = ["ops-${terraform.workspace}-vpc"]
  }
}

data "aws_subnets" "private_subnets" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.vpc.id]
  }

  filter {
    name   = "tag:Class"
    values = ["private"]
  }
}
