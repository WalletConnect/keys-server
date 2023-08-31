local grafana        = import '../../grafonnet-lib/grafana.libsonnet';
local panels         = grafana.panels;
local targets        = grafana.targets;

local defaults  = import '../defaults.libsonnet';

{
  new(ds, vars, title, metric_name, metric_label)::
    panels.timeseries(
      title       = title,
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)

    .addTarget(targets.prometheus(
      legendFormat  = metric_label,
      datasource    = ds.prometheus,
      expr          = 'sum(avg_over_time(increase(%s{aws_ecs_task_family="%s"}[$__rate_interval])[5m:30s]))' % [metric_name, vars.ecs_task_family],
      refId         = metric_name,
      exemplar      = true,
    ))
}
