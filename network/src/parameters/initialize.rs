use crate::{
    parameters::{number_of_parameters, ParametersFlat, Structure},
    util,
};
use rand::{self, Rng};
use rand_distr::Normal;

pub fn kaiming(structure: Structure) -> ParametersFlat {
    let mut rng = rand::thread_rng();

    let mut parameters = Vec::with_capacity(number_of_parameters(structure.clone()));

    util::slice::pairs(&structure)
        .for_each(|(&input_size, &output_size)| {
            let distribution =
                Normal::new(0.0, (2.0 / input_size as f64).sqrt()).unwrap();

            parameters.extend(
                (&mut rng)
                    .sample_iter(distribution)
                    .take(input_size * output_size + output_size)
            );
        });

    parameters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parameters() {
        let parameters = kaiming(vec![3, 2, 2]);

        assert_eq!(
            parameters.len(),
            3 * 2 + 2 + 2 * 2 + 2,
        );
    }
}
