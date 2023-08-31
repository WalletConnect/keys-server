local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Invite - Invalid JWT during Registration', 'invalid_invite_register_jwt', 'Invalid JWTs')
}
