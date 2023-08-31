local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Identity - Invalid JWT during Unregistration', 'invalid_identity_unregister_jwt', 'Invalid JWTs')
}
