use crate::parameters::Parameters;
use rand::{self, Rng};
use rand_distr::Normal;

pub fn kaiming(network_structure: &[usize]) -> Parameters {
    let mut rng = rand::thread_rng();

    network_structure[..network_structure.len() - 1]
        .iter()
        .zip(network_structure[1..].iter())
        .map(|(input_size, output_size)| {
            let distribution =
                Normal::new(0.0, (2.0 / *input_size as f64).sqrt()).unwrap();

            vec![
                (
                    (&mut rng)
                        .sample_iter(distribution)
                        .take(*input_size)
                        .collect(),
                    (&mut rng).sample(distribution),
                );
                *output_size
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parameters() {
        let parameters = kaiming(&[ 12, 6, 2 ]);

        assert_eq!(
            [
                parameters.len(),
                parameters[0].len(),
                parameters[0][0].0.len(),
            ],
            [
                2,
                6,
                12,
            ],
        );
    }
}
