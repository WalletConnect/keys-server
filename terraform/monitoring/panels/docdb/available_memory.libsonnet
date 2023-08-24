local grafana         = import '../../grafonnet-lib/grafana.libsonnet';
local defaults        = import '../../grafonnet-lib/defaults.libsonnet';

local panels          = grafana.panels;
local targets         = grafana.targets;
local alert           = grafana.alert;
local alertCondition  = grafana.alertCondition;

local mem_threshold = 4000000000;   // 4GiB
local max_memory    = 16000000000;  // 16GiB (AWS DocDB max on db.r6g.large)

local _configuration = defaults.configuration.timeseries
  .withThresholdStyle('area')
  .setThresholds(
    baseColor = 'red',
    steps = [
      { value: mem_threshold, color: 'green' },
    ]
  )
  .withUnit('decbytes')
  .addOverride(grafana.override.new(
    name = 'Mem_Min',
    properties = [{
      id: 'color',
      value: {
        mode: 'fixed',
        fixedColor: 'dark-purple'
      }
    }],
  ))
  .addOverride(grafana.override.new(
    name = 'Mem_Avg',
    properties = [{
      id: 'color',
      value: {
        mode: 'fixed',
        fixedColor: 'purple'
      }
    }],
  ))
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = max_memory,
  );


local mem_alert(vars) = alert.new(
  namespace     = vars.namespace,
  name          = "%s DocumentDB Freeable Memory Alert" % vars.environment,
  message       = "%s DocumentDB Freeable Memory" % vars.environment,
  period        = '5m',
  frequency     = '1m',
  notifications = vars.notifications,
  conditions    = [
    alertCondition.new(
      evaluatorParams = [ mem_threshold ],
      evaluatorType   = 'lt',
      operatorType    = 'and',
      queryRefId      = 'Mem_Avg',
      queryTimeStart  = '5m',
      queryTimeEnd    = 'now',
      reducerType     = 'min',
    ),
  ]
);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Available Memory',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .addPanelThreshold(
      op = 'lt',
      value = mem_threshold,
    )

    .setAlert(mem_alert(vars))

    .addTarget(targets.cloudwatch(
      refId       = 'Mem_Min',
      alias       = 'Freeable Memory (Min)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/DocDB',
      metricName  = 'FreeableMemory',
      statistic   = 'Minimum',
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
    ))
    .addTarget(targets.cloudwatch(
      refId       = 'Mem_Avg',
      alias       = 'Freeable Memory (Avg)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/DocDB',
      metricName  = 'FreeableMemory',
      statistic   = 'Average',
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
    ))
}
