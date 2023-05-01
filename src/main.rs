mod utils;

use utils::error::MaskerError;

use tracing::{info, debug};
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    // The runtime logging can be enabled here by initializing `tracing` with `tracing-subscriber`
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_line_number(true)
        .with_thread_names(true)
        .event_format(format().compact())
        .init();

    info!("info!");
    debug!("debug");
    Ok(())
}
