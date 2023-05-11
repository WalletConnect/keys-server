local grafana         = import '../../grafonnet-lib/grafana.libsonnet';
local panels          = grafana.panels;
local targets         = grafana.targets;

local defaults        = import '../defaults.libsonnet';

local _configuration  = defaults.configuration.timeseries;

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Database Connections',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)

    .addTarget(targets.cloudwatch(
      alias       = 'Database Connections',
      datasource  = ds.cloudwatch,
      namespace   = 'AWS/DocDB',
      metricName  = 'DatabaseConnections',
      statistic   = 'Average',
      dimensions  = {
        DBClusterIdentifier: vars.docdb_cluster_id
      },
      matchExact  = true,
    ))
}
