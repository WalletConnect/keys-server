local grafana         = import '../../grafonnet-lib/grafana.libsonnet';
local panels          = grafana.panels;
local targets         = grafana.targets;
local alert           = grafana.alert;
local alertCondition  = grafana.alertCondition;

local defaults        = import '../defaults.libsonnet';

local ops_threshold   = 2;

local _configuration  = defaults.configuration.timeseries
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = 10,
  )
  .withThresholdStyle('area')
  .addOverride(grafana.override.new(
    name = 'Ops_Max',
    properties = [{
      id: 'color',
      value: {
        mode: 'fixed',
        fixedColor: 'red'
      }
    }],
  ))
  .addThreshold({
    color : 'red',
    value : ops_threshold
  });


local ops_alert(vars) = alert.new(
  name        = "%s Keyserver DocumentDB LowMem Num Operations Throttled Alert" % vars.environment,
  message     = "%s Keyserver DocumentDB LowMem Num Operations Throttled" % vars.environment,
  period      = '5m',
  frequency   = '1m',
  notifications = vars.notifications,
  conditions  = [
    alertCondition.new(
      evaluatorParams = [ ops_threshold ],
      evaluatorType   = 'gt',
      operatorType    = 'and',
      queryRefId      = 'Ops_Max',
      queryTimeStart  = '5m',
      queryTimeEnd    = 'now',
      reducerType     = 'max',
    ),
  ]
);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'LowMem Num Operations Throttled',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)

    .setAlert(ops_alert(vars))

    .addTarget(targets.cloudwatch(
      alias       = 'LowMem Num Operations Throttled (Avg)',
      datasource  = ds.cloudwatch,
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
      namespace   = 'AWS/DocDB',
      metricName  = 'LowMemNumOperationsThrottled',
      statistic   = 'Maximum',
      refId       = 'Ops_Max',
    ))
}
