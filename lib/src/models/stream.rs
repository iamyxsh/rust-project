use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;

use super::event::Event;

/**
 * This is the strcut that implements the Stream.
 * I took inspiration from a stack overflow answer.
 */

pub struct StreamDeq {
    pub deq: VecDeque<Event>,
    pub pos: usize,
}

impl Stream for StreamDeq {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.pos < self.deq.len() {
            let item = self.deq[self.pos].clone();
            self.pos += 1;
            Poll::Ready(Some(item))
        } else {
            Poll::Ready(None)
        }
    }
}

impl StreamDeq {
    pub fn new(deq: Vec<Event>) -> Self {
        StreamDeq {
            deq: VecDeque::from(deq),
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{super::parse_reader, super::utils::file::read_file};
    use super::*;
    use futures::StreamExt;

    const TEST_FILE: &str = "../data/test_file.txt";

    fn return_stream() -> StreamDeq {
        let reader = read_file(TEST_FILE);
        StreamDeq::new(parse_reader(reader))
    }

    #[test]
    fn test_new() {
        let vec = Vec::new();
        let stream = StreamDeq::new(vec);

        assert_eq!(stream.pos, 0);
        assert_eq!(stream.deq.len(), 0);
    }

    #[tokio::test]
    async fn test_stream_reading() {
        let mut stream = return_stream();

        while let Some(event) = stream.next().await {
            if stream.pos == 1 {
                assert_eq!(event.timestamp, 1662022800005);
            } else {
                assert_eq!(event.timestamp, 1662022800043);
            }
        }
    }
}
