local grafana         = import '../grafonnet-lib/grafana.libsonnet';

{
  configuration:: {
    timeseries::
      grafana.panels.timeseries().createConfiguration(
        scaleDistribution = {
          type : 'linear'
        },
        stacking = {
          group:  'A',
          mode:   'none'
        },
        legend  = grafana.common.legend(),
        tooltip = grafana.common.tooltip(),
      ),

    timeseries_resource::
      grafana.panels.timeseries().createConfiguration(
        axisSoftMin = 0,
        axisSoftMax = 100,
        thresholdsStyle = {
          mode: 'area',
        },
        scaleDistribution = {
          type: 'linear'
        },
        stacking = {
          group: 'A',
          mode:  'none'
        },
        legend  = grafana.common.legend(),
        tooltip = grafana.common.tooltip(),
      )
      .addOverride(grafana.override.new(
        name = 'CPU_Max',
        properties = [{
          id: 'color',
          value: {
            mode: 'fixed',
            fixedColor: 'dark-blue'
          }
        }],
      ))
      .addOverride(grafana.override.new(
        name = 'CPU_Avg',
        properties = [{
          id: 'color',
          value: {
            mode: 'fixed',
            fixedColor: 'blue'
          }
        }],
      ))
      .addOverride(grafana.override.new(
        name = 'Mem_Max',
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
      .addThreshold({
        color : 'red',
        value : 50
      }),

    timeseries_tr80:: self.timeseries
      .addThreshold({
        color : 'red',
        value : 80
      }),
  },
}
