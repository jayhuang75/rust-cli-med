use tracing_subscriber::fmt::format;

/// params: debug bool
pub async fn logging(debug: bool) {
    // let mut builder = Builder::new();
    // if debug {
    //     // env::set_var("RUST_LOG", "debug");
    //     builder
    //         .filter(None, LevelFilter::Debug)
    //         .write_style(WriteStyle::Always)
    //         .init();
    // } else {
    //     // env::set_var("RUST_LOG", "info");
    //     builder
    //         .format(|buf, record| {
    //             let style = buf.style();
    //             let timestamp = buf.timestamp();
    //             writeln!(
    //                 buf,
    //                 "[{} {}]: {}",
    //                 timestamp,
    //                 LevelFilter::Info.to_string().bold().green(),
    //                 style.value(record.args())
    //             )
    //         })
    //         .filter(None, LevelFilter::Info)
    //         .write_style(WriteStyle::Always)
    //         .init();
    // }

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
}
