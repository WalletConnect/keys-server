local grafana         = import '../../grafonnet-lib/grafana.libsonnet';
local panels          = grafana.panels;
local targets         = grafana.targets;
local alert           = grafana.alert;
local alertCondition  = grafana.alertCondition;

local defaults  = import '../defaults.libsonnet';

local _configuration = defaults.configuration.timeseries_resource
  .withUnit('percent')
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = 30,
  );

local cpu_alert(vars) = alert.new(
  name        = "%s App CPU/Memory alert" % vars.environment,
  message     = "%s App CPU/Memory" % vars.environment,
  period      = '25m',
  frequency   = '1m',
  conditions  = [
    alertCondition.new(
      evaluatorParams = [ 50 ],
      evaluatorType   = 'gt',
      operatorType    = 'or',
      queryRefId      = 'CPU_Avg',
      queryTimeStart  = '25m',
      reducerType     = 'max',
    ),
    alertCondition.new(
      evaluatorParams = [ 50 ],
      evaluatorType   = 'gt',
      operatorType    = 'or',
      queryRefId      = 'Mem_Avg',
      queryTimeStart  = '25m',
      reducerType     = 'max',
    ),
  ]
);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'App CPU/Memory',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .setAlert(cpu_alert(vars))

    .addTarget(targets.cloudwatch(
      refId       = 'CPU_Max',
      alias       = 'CPU (Max)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ECS',
      metricName  = 'CPUUtilization',
      statistic   = 'Maximum',
      dimensions  = {
        ServiceName: vars.ecs_service_name
      },
    ))
    .addTarget(targets.cloudwatch(
      refId       = 'CPU_Avg',
      alias       = 'CPU (Avg)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ECS',
      metricName  = 'CPUUtilization',
      statistic   = 'Average',
      dimensions  = {
        ServiceName: vars.ecs_service_name
      },
    ))

    .addTarget(targets.cloudwatch(
      refId       = 'Mem_Max',
      alias       = 'Memory (Max)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ECS',
      metricName  = 'MemoryUtilization',
      statistic   = 'Maximum',
      dimensions  = {
        ServiceName: vars.ecs_service_name
      },
    ))
    .addTarget(targets.cloudwatch(
      refId       = 'Mem_Avg',
      alias       = 'Memory (Avg)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/ECS',
      metricName  = 'MemoryUtilization',
      statistic   = 'Average',
      dimensions  = {
        ServiceName: vars.ecs_service_name
      },
    ))
}
