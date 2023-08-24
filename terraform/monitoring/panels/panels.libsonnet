{
  ecs: {
    cpu: (import 'ecs/cpu.libsonnet').new,
    memory: (import 'ecs/memory.libsonnet').new,
  },

  app: {
    invite: {
      register: (import 'app/invite/register.libsonnet').new,
      resolved: (import 'app/invite/resolved.libsonnet').new,
      unregister: (import 'app/invite/unregister.libsonnet').new,
      invalid_register_jwt: (import 'app/invite/invalid_register_jwt.libsonnet').new,
      invalid_unregister_jwt: (import 'app/invite/invalid_unregister_jwt.libsonnet').new,
    },
    identity: {
      register: (import 'app/identity/register.libsonnet').new,
      resolved: (import 'app/identity/resolved.libsonnet').new,
      unregister: (import 'app/identity/unregister.libsonnet').new,
      invalid_register_cacao: (import 'app/identity/invalid_register_cacao.libsonnet').new,
      invalid_unregister_jwt: (import 'app/identity/invalid_unregister_jwt.libsonnet').new,
    },
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
