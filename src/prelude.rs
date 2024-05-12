use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

/// Common DSA Re-exports
pub use std::collections::{HashMap, HashSet, VecDeque};
