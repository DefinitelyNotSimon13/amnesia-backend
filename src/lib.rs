pub mod database;
pub mod reminder;
pub mod server;
use color_eyre::Result;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

pub mod reminderhandler {
    tonic::include_proto!("reminderhandler");
}

pub fn install_color_eyre() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}

pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .init();
}
