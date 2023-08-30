local grafana        = import '../../../grafonnet-lib/grafana.libsonnet';
local panels         = grafana.panels;
local targets        = grafana.targets;

local defaults  = import '../../defaults.libsonnet';

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Invite - Registrations',
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)

    .addTarget(targets.prometheus(
      legendFormat  = 'Registrations',
      datasource    = ds.prometheus,
      expr          = 'sum(invite_register{aws_ecs_task_family="%s"})' % vars.ecs_task_family,
      refId         = "sources",
      exemplar      = true,
    ))
}
