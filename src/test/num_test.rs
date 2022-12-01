use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[test]
fn num_test() {
    let file_lines_count = 1024;
    let max_res_count = 100;
    let difference: isize = max_res_count - file_lines_count;
    let begin_read_index = num::abs(difference);
    println!("{:?}", begin_read_index)
}

#[test]
fn test2() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    info!(number_of_yaks, "yak shaving completed.");
}
