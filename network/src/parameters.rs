pub mod binary;
mod kaiming;

pub type Parameters = Vec<Vec<(Vec<f64>, f64)>>;

pub mod generate {
    pub use super::kaiming::*;
}
