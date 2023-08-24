{
  app: {
    cpu: (import 'app/cpu.libsonnet').new,
    memory: (import 'app/memory.libsonnet').new,
  },

  lb: {
    active_connections: (import 'lb/active_connections.libsonnet').new,
    error_4xx: (import 'lb/error_4xx.libsonnet').new,
    error_5xx: (import 'lb/error_5xx.libsonnet').new,
    healthy_hosts: (import 'lb/healthy_hosts.libsonnet').new,
    requests: (import 'lb/requests.libsonnet').new,
  },

  docdb: {
    buffer_cache_hit_ratio: (import 'docdb/buffer_cache_hit_ratio.libsonnet').new,
    cpu: (import 'docdb/cpu.libsonnet').new,
    volume: (import 'docdb/volume.libsonnet').new,
    available_memory: (import 'docdb/available_memory.libsonnet').new,
    connections: (import 'docdb/connections.libsonnet').new,
    low_mem_op_throttled: (import 'docdb/low_mem_op_throttled.libsonnet').new,
  },
}
