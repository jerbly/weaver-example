pub mod attributes;

use opentelemetry::trace::Tracer;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::ExporterBuildError;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;

fn init_tracer_provider() -> Result<SdkTracerProvider, ExporterBuildError> {
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

fn example_span() {
    // Get a tracer
    let tracer = global::tracer("weaver-example");

    // Create and start a span with the required attributes
    let _span = tracer
        .span_builder("example_message")
        .with_attributes(vec![
            KeyValue::new(attributes::EXAMPLE_MESSAGE, "Hello, World!"),
            KeyValue::new(attributes::HOST_NAME, "opentelemetry-test"),
            KeyValue::new(attributes::HOST_ARCH, std::env::consts::ARCH),
        ])
        .start(&tracer);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Initialize the tracer provider with stdout exporter
    let tracer_provider = init_tracer_provider()?;

    // Set the global tracer provider
    let _ = global::set_tracer_provider(tracer_provider.clone());

    example_span();

    tracer_provider.force_flush()?;

    Ok(())
}
