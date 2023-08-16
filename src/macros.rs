#[macro_export]
macro_rules! increment_counter {
    ($state:ident$(.$property:ident)*, $metric:ident) => {{
        use {opentelemetry::Context, tracing::debug};

        if let Some(metrics) = &$state$(.$property)* {
            metrics.$metric.add(&Context::current(), 1, &[]);
            debug!("incremented `{}` counter", stringify!($metric));
        }
    }};
}

#[macro_export]
macro_rules! increment_counter_with {
    ($state:ident$(.$property:ident)*, $metric:ident, $value:expr) => {{
        use {opentelemetry::Context, tracing::debug};

        if let Some(metrics) = &$state$(.$property)* {
            metrics.$metric.add(&Context::current(), $value, &[]);
            debug!("incremented `{}` counter", stringify!($metric));
        }
    }};
}
