local grafana     = import 'grafonnet-lib/grafana.libsonnet';
local panels      = import 'panels/panels.libsonnet';

local dashboard   = grafana.dashboard;

local ds    = {
  prometheus: {
    type: 'prometheus',
    uid:  std.extVar('prometheus_uid'),
  },
  cloudwatch: {
    type: 'cloudwatch',
    uid:  std.extVar('cloudwatch_uid'),
  }
};
local vars  = {
  notifications:    std.parseJson(std.extVar('notifications')),
  environment:      std.extVar('environment'),
  ecs_service_name: std.extVar('ecs_service_name'),
  load_balancer:    std.extVar('load_balancer'),
  docdb_cluster_id: std.extVar('docdb_cluster_id'),
};

////////////////////////////////////////////////////////////////////////////////

local height  = 8;
local pos     = grafana.layout.pos(height);

////////////////////////////////////////////////////////////////////////////////

dashboard.new(
  title         = std.extVar('dashboard_title'),
  uid           = std.extVar('dashboard_uid'),
  editable      = true,
  graphTooltip  = dashboard.graphTooltips.sharedCrosshair,
)
.addAnnotation(
  grafana.annotation.new(
    target = {
      limit:    100,
      matchAny: false,
      tags:     [],
      type:     'dashboard',
    },
  )
)
.addPanels(
  grafana.layout.generate_grid([
    panels.app.app_cpu_memory(ds, vars)         { gridPos: pos._2     },
    panels.app.healthy_hosts(ds, vars)          { gridPos: pos._2     },
    panels.app.active_nlb_flows(ds, vars)       { gridPos: pos._2     },
    panels.app.nlb_target_resets(ds, vars)      { gridPos: pos._2     },

    ////////////////////////////////////////////////////////////////////////////
    grafana.panels.text(
      content     = '# DocumentDB',
      transparent = true
    )                                                 { gridPos: pos.title  },

    panels.docdb.cpu(ds, vars)                        { gridPos: pos._3     },
    panels.docdb.available_memory(ds, vars)           { gridPos: pos._3     },
    panels.docdb.connections(ds, vars)                { gridPos: pos._3     },

    panels.docdb.low_mem_op_throttled(ds, vars)       { gridPos: pos._3     },
    panels.docdb.volume(ds, vars)                     { gridPos: pos._3     },
    panels.docdb.buffer_cache_hit_ratio(ds, vars)     { gridPos: pos._3     },
  ])
)
