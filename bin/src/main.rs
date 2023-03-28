use chrono::Duration;
use futures::StreamExt;
use tensor_lib::{
    models::{rolling::Rolling, stream::StreamDeq},
    parse_reader,
    utils::file::read_file,
};

/**
 * CONSTANTS for the app.
 * Can be changed by the user.
 */
const FILE_NAME: &str = "../data/dataset-b.txt";
const TICKERS: [&str; 2] = ["TURBOUSDT", "FISHUSDT"];
const DURATION_IN_MILLISECONDS: i64 = 5 * 60 * 1000;
const ANSWER_FILE: &str = "../data/ohlc-5m-b.txt";

#[tokio::main]
async fn main() {
    // Reading the file
    let reader = read_file(FILE_NAME);

    // Creating the stream
    let mut stream = StreamDeq::new(parse_reader(reader));

    let mut rolling = Rolling::new(
        Duration::milliseconds(DURATION_IN_MILLISECONDS),
        &TICKERS,
        ANSWER_FILE,
    );

    // Using the stream to feed the data
    while let Some(event) = stream.next().await {
        rolling.add_event(event);
    }

    // Cleaning the file, helps with dev.
    //fs::write(ANSWER_FILE, "").expect("unable to write file");
}
