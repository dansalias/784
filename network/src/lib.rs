pub mod data;
pub mod math;
pub mod parameters;

pub trait ActivationFunction {
    fn fprop(&self, input: &[f64]) -> Vec<f64>;
    fn bprop(&self, input: &[f64]) -> Vec<f64>;
}

pub type Layer = (usize, Box<dyn ActivationFunction>);

pub struct Network {
    pub input_size: usize,
    pub layers: Vec<Layer>,
    pub parameters: parameters::Parameters,
}

impl Network {
    pub fn new(
        input_size: usize,
        layers: Vec<Layer>,
    ) -> Network {
        Network {
            input_size,
            layers,
            parameters: vec![],
        }
    }

    pub fn fprop(&self, input: &[f64]) -> Vec<f64> {
        self
            .parameters
            .iter()
            .zip(
                self
                    .layers
                    .iter()
            )
            .fold(input.to_vec(), |input, (parameters, (_, activation_function))|
                activation_function.fprop(
                    &parameters
                        .iter()
                        .map(|(weights, bias)|
                            weights
                                .iter()
                                .zip(&input)
                                .map(|(w, i)| w * i)
                                .sum::<f64>()
                            + bias
                        )
                        .collect::<Vec<f64>>()
                )
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math;

    #[test]
    fn propagates_forward() {
        let mut network = Network::new(
            3,
            vec![
                (2, Box::new(math::Relu)),
                (2, Box::new(math::Relu)),
            ],
        );

        network.parameters = vec![
            vec![
                (vec![ 0.2, 0.2, 0.3 ], 0.1),
                (vec![ 0.2, 0.3, 0.4 ], 0.1),
            ],
            vec![
                (vec![ 0.2, 0.3 ], 0.1),
                (vec![ 0.3, 0.4 ], -0.5),
            ],
        ];

        assert_eq!(
            network.fprop(&[ 0.3, 0.2, 0.1 ]),
            vec![ 0.224, 0.0 ],
        );
    }
}
