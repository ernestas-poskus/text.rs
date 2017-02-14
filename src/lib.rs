extern crate regex;

mod manipulation;
mod unique;
mod intersect;
mod occurrence;
mod sort;
mod repetitions;

pub use manipulation::Manipulation;
pub use unique::Unique;
pub use intersect::Intersect;
pub use occurrence::Occurrence;
pub use sort::Sort;
pub use repetitions::Repetitions;
