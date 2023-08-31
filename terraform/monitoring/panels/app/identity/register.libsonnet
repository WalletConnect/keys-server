local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Identity - Registrations', 'identity_register', 'Registrations')
}
