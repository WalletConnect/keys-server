local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels    = grafana.panels;
local targets   = grafana.targets;

local threshold = 100;

local _configuration = defaults.configuration.timeseries
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = threshold * 1.2,
  )
  .withThresholdStyle(grafana.fieldConfig.thresholdStyle.dashed)
  .addThreshold({
    color : defaults.values.colors.critical,
    value : threshold,
  });

{
  new(ds, vars)::
    panels.timeseries(
      title       = '4XX',
      datasource  = ds.cloudwatch,
    )
    .configure(
      defaults.configuration.timeseries
        .withSoftLimit(
          axisSoftMin = 0,
          axisSoftMax = threshold * 1.2,
        )
        .withThresholdStyle(grafana.fieldConfig.thresholdStyle.dashed)
        .addThreshold({
          color : defaults.values.colors.critical,
          value : threshold,
        })
    )
    .addPanelThreshold(
      op    = 'gt',
      value = threshold,
    )

    .addTarget(targets.cloudwatch(
      alias       = 'ELB',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ApplicationELB',
      metricName  = 'HTTPCode_ELB_4XX_Count',
      dimensions  = {
        LoadBalancer: vars.load_balancer
      },
      matchExact  = true,
      statistic   = 'Sum',
      refId       = 'ELB',
    ))
    .addTarget(targets.cloudwatch(
      alias       = 'Target',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ApplicationELB',
      metricName  = 'HTTPCode_Target_4XX_Count',
      dimensions  = {
        LoadBalancer: vars.load_balancer
      },
      matchExact  = true,
      statistic   = 'Sum',
      refId       = 'Target',
    ))
}
