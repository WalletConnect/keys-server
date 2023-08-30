local grafana        = import '../../../grafonnet-lib/grafana.libsonnet';
local panels         = grafana.panels;
local targets        = grafana.targets;

local defaults  = import '../../defaults.libsonnet';

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Invite - Invalid JWT during Unregistration',
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)

    .addTarget(targets.prometheus(
      legendFormat  = 'Invalid JWTs',
      datasource    = ds.prometheus,
      expr          = 'sum(invalid_invite_unregister_jwt{aws_ecs_task_family="%s"})' % vars.ecs_task_family,
      refId         = "sources",
      exemplar      = true,
    ))
}
