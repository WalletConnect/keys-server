local grafana     = import 'grafonnet-lib/grafana.libsonnet';
local panels      = import 'panels/panels.libsonnet';

local dashboard   = grafana.dashboard;
local row         = grafana.row;
local layout      = grafana.layout;

local ds    = {
  prometheus: {
    type: 'prometheus',
    uid:  'prometheus_uid',
  },
  cloudwatch: {
    type: 'cloudwatch',
    uid:  'cloudwatch_uid',
  }
};
local vars  = {
  namespace:        'Keyserver',
  notifications:    std.parseJson('["notifications"]'),
  environment:      'environment',
  ecs_service_name: 'ecs_service_name',
  load_balancer:    'load_balancer',
  target_group:     'target_group',
  docdb_cluster_id: 'docdb_cluster_id',
};

////////////////////////////////////////////////////////////////////////////////

local height  = 8;
local pos     = grafana.layout.pos(height);

////////////////////////////////////////////////////////////////////////////////

dashboard.new(
  title         = 'dashboard_title',
  uid           = 'dashboard_uid',
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
.addPanels(layout.generate_grid([
  row.new('Application'),
    panels.app.cpu(ds, vars)                    { gridPos: pos._2 },
    panels.app.memory(ds, vars)                 { gridPos: pos._2 },

  row.new('Load Balancer'),
    panels.lb.active_connections(ds, vars)      { gridPos: pos._2 },
    panels.lb.healthy_hosts(ds, vars)           { gridPos: pos._2 },

    panels.lb.requests(ds, vars)                { gridPos: pos._3 },
    panels.lb.error_4xx(ds, vars)               { gridPos: pos._3 },
    panels.lb.error_5xx(ds, vars)               { gridPos: pos._3 },

  row.new('Database'),
    panels.db.cpu(ds, vars)                     { gridPos: pos._3 },
    panels.db.available_memory(ds, vars)        { gridPos: pos._3 },
    panels.db.connections(ds, vars)             { gridPos: pos._3 },

    panels.db.low_mem_op_throttled(ds, vars)    { gridPos: pos._3 },
    panels.db.volume(ds, vars)                  { gridPos: pos._3 },
    panels.db.buffer_cache_hit_ratio(ds, vars)  { gridPos: pos._3 },
]))
