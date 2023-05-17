use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[cfg(not(windows))]
const TICK_SETTINGS: (&str, u64) = ("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ", 80);

#[cfg(windows)]
const TICK_SETTINGS: (&str, u64) = (r"+-x| ", 200);

/// Return a pre-configured progress bar
pub fn get_progress_bar(length: u64, msg: &str) -> ProgressBar {
    let progressbar_style =  ProgressStyle::default_spinner()
            .tick_chars(TICK_SETTINGS.0)
            .progress_chars("=> ")
            .template(" {spinner} {msg} {percent}% [{bar:40}] {pos}/{len} ETA {elapsed}")
            .expect("no template error");

    let progress_bar =ProgressBar::new(length);
    progress_bar.set_style(progressbar_style);
    progress_bar.enable_steady_tick(Duration::from_millis(TICK_SETTINGS.1));
    progress_bar.set_message(msg.to_owned());

    progress_bar
}