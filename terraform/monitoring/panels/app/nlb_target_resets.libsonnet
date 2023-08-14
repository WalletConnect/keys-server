local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local panels    = grafana.panels;
local targets   = grafana.targets;

local defaults  = import '../defaults.libsonnet';

local _configuration = defaults.configuration.timeseries_tr80
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = 250,
  );


{
  new(ds, vars)::
    panels.timeseries(
      title       = 'NLB Target Resets',
      description = "When the NLB has connection failures to the targets then these jump. We for instance had this when we had a too low file descriptor limit.",
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .addTarget(targets.cloudwatch(
      alias       = 'LB-0',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/NetworkELB',
      metricName  = 'TCP_Target_Reset_Count',
      statistic   = 'Sum',
      dimensions  = {
        LoadBalancer: vars.load_balancer
      },
      matchExact  = true,
    ))
}
