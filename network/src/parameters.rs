use crate::util;

pub mod binary;
pub mod convert;
pub mod initialize;

pub type ParametersFlat = Vec<f64>;
pub type ParametersStructured = Vec<Vec<(Vec<f64>, f64)>>;
pub type Structure = Vec<usize>;

pub enum Initialization {
    FromStructured(ParametersStructured),
    Kaiming,
}

pub fn number_of_parameters(structure: Structure) -> usize {
    util::slice::pairs(&structure)
        .map(|(i, o)| i * o + o)
        .sum()
}

pub struct Parameters {
    pub raw: ParametersFlat,

    structure: Vec<usize>,
}

impl Parameters {
    pub fn new(structure: Vec<usize>) -> Parameters {
        Parameters {
            raw: Vec::with_capacity(number_of_parameters(structure.clone())),
            structure,
        }
    }

    pub fn initialize(mut self, from: Initialization) -> Parameters {
        self.raw = match from {
            Initialization::FromStructured(structured) =>
                convert::to_flat(&structured),
            Initialization::Kaiming => initialize::kaiming(self.structure.clone()),
        };

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_parameters() {
        let parameters = Parameters::new(vec![3, 2, 2]);

        assert_eq!(parameters.raw.capacity(), 14);
    }
}
