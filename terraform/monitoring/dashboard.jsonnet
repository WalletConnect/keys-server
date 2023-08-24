local grafana     = import 'grafonnet-lib/grafana.libsonnet';
local panels      = import 'panels/panels.libsonnet';

local dashboard   = grafana.dashboard;
local row         = grafana.row;

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
  namespace:        'Keys',
  environment:      std.extVar('environment'),
  notifications:    std.parseJson(std.extVar('notifications')),

  ecs_service_name: std.extVar('ecs_service_name'),
  ecs_task_family:  std.extVar('ecs_task_family'),
  load_balancer:    std.extVar('load_balancer'),
  target_group:     std.extVar('target_group'),
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
.addPanels(grafana.layout.generate_grid([
  row.new('Application'),
    panels.app.identity.resolved(ds, vars)                { gridPos: pos._3 },
    panels.app.identity.register(ds, vars)                { gridPos: pos._3 },
    panels.app.identity.unregister(ds, vars)              { gridPos: pos._3 },

    panels.app.identity.invalid_register_cacao(ds, vars)  { gridPos: pos._2 },
    panels.app.identity.invalid_unregister_jwt(ds, vars)  { gridPos: pos._2 },

    panels.app.invite.resolved(ds, vars)                  { gridPos: pos._3 },
    panels.app.invite.register(ds, vars)                  { gridPos: pos._3 },
    panels.app.invite.unregister(ds, vars)                { gridPos: pos._3 },

    panels.app.invite.invalid_register_jwt(ds, vars)      { gridPos: pos._2 },
    panels.app.invite.invalid_unregister_jwt(ds, vars)    { gridPos: pos._2 },

  row.new('ECS'),
    panels.ecs.cpu(ds, vars)                              { gridPos: pos._2 },
    panels.ecs.memory(ds, vars)                           { gridPos: pos._2 },

  row.new('Load Balancer'),
    panels.lb.active_connections(ds, vars)                { gridPos: pos._2 },
    panels.lb.healthy_hosts(ds, vars)                     { gridPos: pos._2 },

    panels.lb.requests(ds, vars)                          { gridPos: pos._3 },
    panels.lb.error_4xx(ds, vars)                         { gridPos: pos._3 },
    panels.lb.error_5xx(ds, vars)                         { gridPos: pos._3 },

  row.new('DocumentDB'),
    panels.docdb.cpu(ds, vars)                            { gridPos: pos._3 },
    panels.docdb.available_memory(ds, vars)               { gridPos: pos._3 },
    panels.docdb.connections(ds, vars)                    { gridPos: pos._3 },

    panels.docdb.low_mem_op_throttled(ds, vars)           { gridPos: pos._3 },
    panels.docdb.volume(ds, vars)                         { gridPos: pos._3 },
    panels.docdb.buffer_cache_hit_ratio(ds, vars)         { gridPos: pos._3 },
]))
