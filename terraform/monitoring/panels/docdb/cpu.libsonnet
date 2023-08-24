local grafana         = import '../../grafonnet-lib/grafana.libsonnet';
local defaults        = import '../../grafonnet-lib/defaults.libsonnet';

local panels          = grafana.panels;
local targets         = grafana.targets;
local alert           = grafana.alert;
local alertCondition  = grafana.alertCondition;

local cpu_alert(vars) = alert.new(
  namespace     = vars.namespace,
  name          = "%s DocumentDB CPU alert" % vars.environment,
  message       = "%s DocumentDB CPU alert" % vars.environment,
  period        = '5m',
  frequency     = '1m',
  notifications = vars.notifications,
  conditions    = [
    alertCondition.new(
      evaluatorParams = [ 50 ],
      evaluatorType   = 'gt',
      operatorType    = 'or',
      queryRefId      = 'CPU_Max',
      queryTimeStart  = '5m',
      queryTimeEnd    = 'now',
      reducerType     = 'avg',
    ),
  ]
);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'CPU Utilization',
      datasource  = ds.cloudwatch,
    )
    .configure(defaults.configuration.timeseries_resource)
    .setAlert(cpu_alert(vars))

    .addTarget(targets.cloudwatch(
      refId       = 'CPU_Max',
      alias       = 'CPU (Max)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/DocDB',
      metricName  = 'CPUUtilization',
      statistic   = 'Maximum',
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
    ))
    .addTarget(targets.cloudwatch(
      refId       = 'CPU_Avg',
      alias       = 'CPU (Avg)',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/DocDB',
      metricName  = 'CPUUtilization',
      statistic   = 'Average',
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
    ))
}
