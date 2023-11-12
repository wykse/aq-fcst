use clap::Parser;
use humantime::format_duration;
use std::process;
use std::time::{Duration, Instant};

fn main() {
    // Start timer
    let start_time = Instant::now();

    // Parse args
    let args = aq_fcst::Args::parse();

    // // Run program
    if let Err(e) = aq_fcst::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // Stop timer
    let elapsed_time = start_time.elapsed();

    // Print elapsed time in seconds
    println!(
        "Time taken: {}",
        format_duration(Duration::from_secs(elapsed_time.as_secs()))
    );
}
