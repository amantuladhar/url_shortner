use color_eyre::Report;

// Setup logs
pub fn log() -> Result<(), Report> {
    // Setup colorful backtrace for better error handling
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    // Tracing for async logs
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt().init();

    Ok(())
}
