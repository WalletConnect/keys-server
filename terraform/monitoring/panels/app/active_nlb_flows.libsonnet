local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local panels    = grafana.panels;
local targets   = grafana.targets;

local defaults  = import '../defaults.libsonnet';

local _configuration = defaults.configuration.timeseries
  .withSoftLimit(
    axisSoftMin = 0
  );

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Active NLB Flows',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .addTarget(targets.cloudwatch(
      alias       = 'LB-0',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/NetworkELB',
      metricName  = 'ActiveFlowCount_TLS',
      statistic   = 'Maximum',
      dimensions  = {
        LoadBalancer: vars.load_balancer
      },
      matchExact  = true,
    ))
}
