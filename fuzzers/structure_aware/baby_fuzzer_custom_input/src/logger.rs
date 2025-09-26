use tracing_subscriber::prelude::*;

pub fn init() {
    let args: Vec<String> = std::env::args().collect();
    let use_file_logging = args.iter().any(|a| a == "--file");

    let mut layers: Vec<
        Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>,
    > = Vec::new();
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    layers.push(env_filter.boxed());
    let stdout_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);
    layers.push(stdout_layer.boxed());

    if use_file_logging {
        let file_writer = tracing_subscriber::fmt::writer::BoxMakeWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("tracing.log")
                .unwrap(),
        );
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false);

        layers.push(file_layer.boxed());
    }

    tracing_subscriber::registry().with(layers).init();

    // tracing_subscriber::fmt::init();
    // tracing_log::LogTracer::init().unwrap();
}
