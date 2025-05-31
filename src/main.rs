pub mod attributes;

use anyhow::Result;
use opentelemetry::trace::Tracer;
use opentelemetry::{KeyValue, Value, global};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::env;

#[derive(Debug)]
pub struct ParamValue<'a>(&'a str);

impl From<ParamValue<'_>> for Value {
    fn from(msg_val: ParamValue<'_>) -> Self {
        let arg = msg_val.0;
        // Try to parse as integer first
        if let Ok(int_val) = arg.parse::<i64>() {
            return Value::I64(int_val);
        }

        // Try to parse as float
        if let Ok(float_val) = arg.parse::<f64>() {
            return Value::F64(float_val);
        }

        // Default to string
        Value::String(arg.to_string().into())
    }
}

fn init_tracer_provider() -> Result<SdkTracerProvider> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;
    Ok(SdkTracerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("weaver-example")
                .build(),
        )
        .with_batch_exporter(exporter)
        .build())
}

fn example_span(message: ParamValue<'_>) {
    let host_name = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let tracer = global::tracer("weaver-example");
    let _span = tracer
        .span_builder("example_message")
        .with_attributes(vec![
            KeyValue::new(attributes::EXAMPLE_MESSAGE, message),
            KeyValue::new(attributes::HOST_NAME, host_name),
            KeyValue::new(attributes::HOST_ARCH, std::env::consts::ARCH),
        ])
        .start(&tracer);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        ParamValue(&args[1])
    } else {
        ParamValue("Hello, World!")
    };

    let tracer_provider = init_tracer_provider()?;
    let _ = global::set_tracer_provider(tracer_provider.clone());

    example_span(message);

    tracer_provider.force_flush()?;

    Ok(())
}
