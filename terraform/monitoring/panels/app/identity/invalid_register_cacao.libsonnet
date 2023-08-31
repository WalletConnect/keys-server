local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Identity - Invalid CACAO during Registration', 'invalid_identity_register_cacao', 'Invalid CACAOs')
}
