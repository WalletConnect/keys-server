{
  app: {
    cpu:                            (import 'app/cpu.libsonnet'                     ).new,
    memory:                         (import 'app/memory.libsonnet'                  ).new,
  },

  db: {
    buffer_cache_hit_ratio:         (import 'db/buffer_cache_hit_ratio.libsonnet'   ).new,
    cpu:                            (import 'db/cpu.libsonnet'                      ).new,
    volume:                         (import 'db/volume.libsonnet'                   ).new,
    available_memory:               (import 'db/available_memory.libsonnet'         ).new,
    connections:                    (import 'db/connections.libsonnet'              ).new,
    low_mem_op_throttled:           (import 'db/low_mem_op_throttled.libsonnet'     ).new,
  },

  lb: {
    active_connections:             (import 'lb/active_connections.libsonnet'       ).new,
    healthy_hosts:                  (import 'lb/healthy_hosts.libsonnet'            ).new,
    error_4xx:                      (import 'lb/error_4xx.libsonnet'                ).new,
    error_5xx:                      (import 'lb/error_5xx.libsonnet'                ).new,
    requests:                       (import 'lb/requests.libsonnet'                 ).new,
  }
}
