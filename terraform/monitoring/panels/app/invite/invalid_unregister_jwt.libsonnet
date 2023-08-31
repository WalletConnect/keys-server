local app_metric = import '../app_metric.libsonnet';

{
  new(ds, vars):: app_metric.new(ds, vars, 'Invite - Invalid JWT during Unregistration', 'invalid_invite_unregister_jwt', 'Invalid JWTs')
}
