use std::sync::Once;

use tracing_subscriber::fmt::format;

/// params: debug bool
pub async fn logging(debug: bool) {
    static START: Once = Once::new();

    START.call_once(|| {
        let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
            .event_format(format().compact());

        match debug {
            true => {
                subscriber
                    .with_line_number(true)
                    .with_target(true)
                    .with_file(true)
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }
            false => {
                subscriber
                    .with_target(false)
                    .with_max_level(tracing::Level::INFO)
                    .init();
            }
        }
    });
}
