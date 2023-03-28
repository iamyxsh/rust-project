use std::collections::{HashMap, VecDeque};

use chrono::{Duration, NaiveDateTime};

use crate::utils::{file::write_file, MinNonNan};

use super::{event::Event, ohlc::OHLC};

/**
 * This is the Rolling Struct.
 * this contains the deques associated with each ticker
 * as well as the metadata - window and asnwer file.
 */

#[derive(Debug)]
pub struct Rolling {
    pub duration: Duration,
    pub deque_map: HashMap<String, VecDeque<OHLC>>,
    pub answer_file: String,
}

impl Rolling {
    // Init
    pub fn new(d: Duration, tickers: &[&str], answer_file: &str) -> Self {
        let mut map = HashMap::new();
        for ticker in tickers.iter() {
            map.insert(ticker.to_string(), VecDeque::new());
        }
        Rolling {
            deque_map: map,
            duration: d,
            answer_file: answer_file.to_string(),
        }
    }

    pub fn add_event(&mut self, e: Event) {
        let deque = self.deque_map.get_mut(&e.symbol).unwrap();

        Rolling::recursive_pop(deque, &e, self.duration.clone());

        let price = Rolling::calculate_price(e.asking_price, e.bidding_price);

        let front = deque.front();
        match front {
            Some(front) => {
                let low = Rolling::get_low(&*deque).0;
                let high = Rolling::get_high(&*deque).0;

                let ohcl = OHLC::new(
                    front.open,
                    price,
                    high,
                    low,
                    e.timestamp,
                    e.symbol.clone(),
                    price,
                );

                write_file(&self.answer_file, &ohcl);

                deque.push_back(ohcl);
            }
            None => {
                let ohcl = OHLC::new(price, price, price, price, e.timestamp, e.symbol, price);

                write_file(&self.answer_file, &ohcl);

                deque.push_back(ohcl);
            }
        }
    }

    pub fn get_low(deque: &VecDeque<OHLC>) -> MinNonNan {
        match deque.iter().max_by_key(|a| a.price) {
            Some(val) => val.price,
            None => MinNonNan(0.0),
        }
    }

    pub fn get_high(deque: &VecDeque<OHLC>) -> MinNonNan {
        match deque.iter().min_by_key(|a| a.price) {
            Some(val) => val.price,
            None => MinNonNan(0.0),
        }
    }

    /**
     * This function checks if the lenth of the window is lower or
     * equal with respect to the given duration. If not, then pops one out.
     */
    pub fn recursive_pop(deque: &mut VecDeque<OHLC>, e: &Event, d: Duration) {
        if deque.len() > 0 {
            let front = deque.front().unwrap();
            let start_datetime = NaiveDateTime::from_timestamp_millis(front.timestamp);
            let end_datetime = NaiveDateTime::from_timestamp_millis(e.timestamp);
            let duration = end_datetime
                .unwrap()
                .signed_duration_since(start_datetime.unwrap());
            if duration >= d {
                deque.pop_front();
                Rolling::recursive_pop(deque, e, duration);
            }
        }
    }

    pub fn calculate_price(asking: f32, bidding: f32) -> f32 {
        (asking + bidding) / 2_f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "../data/test_file.txt";
    const SYMBOL: &str = "SYMBOL";

    fn push_to_vec(vec: &mut VecDeque<OHLC>) {
        let low = OHLC::new(
            0_f32,
            0_f32,
            0_f32,
            0_f32,
            1662022800005_i64,
            SYMBOL.to_string(),
            10_f32,
        );
        let mid = OHLC::new(
            0_f32,
            0_f32,
            0_f32,
            0_f32,
            1662022800010_i64,
            SYMBOL.to_string(),
            10_f32,
        );
        let high = OHLC::new(
            0_f32,
            0_f32,
            0_f32,
            0_f32,
            1662022800015_i64,
            SYMBOL.to_string(),
            5_f32,
        );
        vec.push_back(low);
        vec.push_back(mid);
        vec.push_back(high);
    }

    #[test]
    fn test_new() {
        let rolling = Rolling::new(Duration::minutes(1), &[SYMBOL], TEST_FILE);

        let mut map = HashMap::new();
        map.insert(SYMBOL.to_string(), VecDeque::new() as VecDeque<OHLC>);

        assert_eq!(rolling.deque_map.len(), map.len());
        assert_eq!(
            rolling.deque_map.get(SYMBOL).unwrap().len(),
            map.get(SYMBOL).unwrap().len()
        );
        assert_eq!(rolling.duration, Duration::minutes(1));
        assert_eq!(rolling.answer_file, TEST_FILE);
    }

    #[test]
    fn test_get_low() {
        let mut vec = VecDeque::new();
        push_to_vec(&mut vec);

        let low = Rolling::get_low(&vec);

        assert_eq!(low.0, 5_f32);
    }

    #[test]
    fn test_get_high() {
        let mut vec = VecDeque::new();
        push_to_vec(&mut vec);

        let high = Rolling::get_high(&vec);

        assert_eq!(high.0, 10_f32);
    }

    #[test]
    fn test_recursive_pop() {
        let mut vec = VecDeque::new();

        push_to_vec(&mut vec);
        assert_eq!(vec.len(), 3);

        Rolling::recursive_pop(
            &mut vec,
            &Event {
                asking_price: 0_f32,
                bidding_price: 0_f32,
                timestamp: 1662022800020_i64,
                symbol: SYMBOL.to_string(),
            },
            Duration::milliseconds(10),
        );

        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_calculate_price() {
        let price = Rolling::calculate_price(0.10_f32, 0.10_f32);

        assert_eq!(price, 0.10_f32);
    }
}
