local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Identity - Resolutions', 'identity_resolved', 'Resolutions')
}
