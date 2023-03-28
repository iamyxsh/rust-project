use std::time;

use chrono::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use tensor_lib::{models::rolling::Rolling, parse_reader, utils::file::read_file};

const FILE_NAME: &str = "../data/dataset-a.txt";
const TICKERS: [&str; 2] = ["TURBOUSDT", "FISHUSDT"];
const DURATION_IN_MILLISECONDS: i64 = 5 * 60 * 1000;
const ANSWER_FILE: &str = "../data/yash.txt";

/**
 * Here, I simply tried to bench mark the add_event().
 * Note: I did not use the Stream API as that will require me
 *      to set up an async function. I had problems setting that up so went ahead
 *      with sync.  
 */

fn rolling_holc_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rolling_holc");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("rolling_holc", |b| {
        b.iter(|| {
            let reader = read_file(FILE_NAME);

            let stream = parse_reader(reader);

            let mut rolling = Rolling::new(
                Duration::milliseconds(DURATION_IN_MILLISECONDS),
                &TICKERS,
                ANSWER_FILE,
            );
            for event in stream.into_iter() {
                rolling.add_event(event);
            }
            //fs::write(ANSWER_FILE, "").expect("unable to write file");
        });
    });
    group.finish();
}

criterion_group! {name = benches;
config=Criterion::default().measurement_time(time::Duration::from_secs(500));
targets = rolling_holc_benchmark}
criterion_main!(benches);
