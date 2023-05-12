{
  app: {
    app_cpu_memory:                 (import 'app/app_cpu_memory.libsonnet'                    ).new,
    healthy_hosts:                  (import 'app/healthy_hosts.libsonnet'                     ).new,
    active_nlb_flows:               (import 'app/active_nlb_flows.libsonnet'                     ).new,
    nlb_target_resets:              (import 'app/nlb_target_resets.libsonnet'                     ).new,
  },

  docdb: {
    buffer_cache_hit_ratio:         (import 'docdb/buffer_cache_hit_ratio.libsonnet'          ).new,
    cpu:                            (import 'docdb/cpu.libsonnet'                             ).new,
    volume:                         (import 'docdb/volume.libsonnet'                          ).new,
    available_memory:               (import 'docdb/available_memory.libsonnet'                ).new,
    connections:                    (import 'docdb/connections.libsonnet'                     ).new,
    low_mem_op_throttled:           (import 'docdb/low_mem_op_throttled.libsonnet'            ).new,
  },
}
