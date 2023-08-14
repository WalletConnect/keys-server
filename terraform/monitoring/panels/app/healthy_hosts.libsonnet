local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local panels    = grafana.panels;
local targets   = grafana.targets;

local defaults  = import '../defaults.libsonnet';

local _configuration = defaults.configuration.timeseries
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = 5,
  );

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Healthy Hosts',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)

    .addTarget(targets.cloudwatch(
      alias           = 'Hosts Count',
      metricQueryType = grafana.target.cloudwatch.metricQueryTypes.query,
      datasource      = ds.cloudwatch,
      namespace       = 'AWS/NetworkELB',
      metricName      = 'HealthyHostCount',

      sql           = {
        from: {
          property: {
            name: "AWS/NetworkELB",
            type: "string"
          },
          type: "property"
        },
        select: {
          name: "MAX",
          parameters: [
            {
              name: "HealthyHostCount",
              type: "functionParameter"
            }
          ],
          type: "function"
        },
        where: {
          expressions: [
            {
              operator: {
                name: "=",
                value: vars.load_balancer
              },
              property: {
                name: "LoadBalancer",
                type: "string"
              },
              type: "operator"
            }
          ],
          type: "and"
        }
      },
      sqlExpression = "SELECT MAX(HealthyHostCount) FROM \"AWS/NetworkELB\" WHERE LoadBalancer = '%s'" % [vars.load_balancer],
    ))
}
