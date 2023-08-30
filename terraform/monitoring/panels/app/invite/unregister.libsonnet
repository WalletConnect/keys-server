local grafana        = import '../../../grafonnet-lib/grafana.libsonnet';
local panels         = grafana.panels;
local targets        = grafana.targets;

local defaults  = import '../../defaults.libsonnet';

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Invite - Unregistrations',
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)

    .addTarget(targets.prometheus(
      legendFormat  = 'Unregistrations',
      datasource    = ds.prometheus,
      expr          = 'sum(invite_unregister{aws_ecs_task_family="%s"})' % vars.ecs_task_family,
      refId         = "sources",
      exemplar      = true,
    ))
}
