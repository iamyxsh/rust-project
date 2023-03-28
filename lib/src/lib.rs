pub mod models;
pub mod utils;

use models::event::Event;
use std::io::BufRead;

// Process the lines and returns the Vec of Events.
pub fn parse_reader<T: BufRead>(stream: T) -> Vec<Event> {
    let mut deq: Vec<Event> = Vec::new();
    for line in stream.lines() {
        match line {
            Ok(line) => {
                let event: Event = serde_json::from_str(line.as_str()).unwrap();

                deq.push(event);
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    deq
}
