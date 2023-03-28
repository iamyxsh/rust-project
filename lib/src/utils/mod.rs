pub mod file;
pub mod parser;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/**
 * A custom struct so that the f32
 * can be used to implement Ord.
 * Took inspiration from a stack overflow answer.
 */

#[derive(PartialEq, Debug, Copy, Clone, Deserialize, Serialize, Default)]
pub struct MinNonNan(pub f32);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
