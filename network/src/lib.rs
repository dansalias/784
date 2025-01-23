pub mod data;
pub mod math;
pub mod parameters;
pub mod training;
pub mod util;

pub trait ActivationFunction {
    fn fprop(&self, input: &[f64]) -> Vec<f64>;
    fn bprop(&self, input: &[f64]) -> Vec<f64>;
}

pub trait LossFunction {
    fn fprop(&self, actual: &[f64], expected: &[f64]) -> Vec<f64>;
    fn bprop(&self, actual: &[f64], expected: &[f64]) -> Vec<f64>;
}

pub type Layer = (usize, Option<Box<dyn ActivationFunction>>);

pub struct Network {
    pub input_size: usize,
    pub layers: Vec<Layer>,
    pub parameters: parameters::Parameters,

    activations: Vec<(Vec<f64>, Vec<f64>)>,
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
            activations: vec![],
        }
    }

    pub fn fprop(&mut self, input: &[f64]) -> Vec<f64> {
        self.activations = vec![(
            input.to_vec(),
            input.to_vec(),
        )];

        self
            .parameters
            .iter()
            .zip(self.layers.iter())
            .fold(input.to_vec(), |input, (parameters, (_, activation_function))| {
                let weighted_sums =
                    parameters
                        .iter()
                        .map(|(weights, bias)|
                            weights
                                .iter()
                                .zip(input.iter())
                                .map(|(w, i)| w * i)
                                .sum::<f64>()
                            + bias
                        )
                        .collect::<Vec<f64>>();

                let activations = match activation_function {
                    Some(af) => af.fprop(&weighted_sums),
                    _ => weighted_sums.to_vec(),
                };

                self.activations.push((
                    weighted_sums,
                    activations.to_vec(),
                ));

                activations
            })
    }

    pub fn bprop(&mut self, error: &[f64], learning_rate: f64) {
        self
            .layers
            .iter()
            .enumerate()
            .zip(
                self
                    .activations[..self.activations.len() - 1]
                    .iter()
                    .zip(self.activations[1..].iter())
            )
            .rfold(
                error.to_vec(),
                |
                    gradient,
                    ((layer_index, (_, activation_function)), (
                        (_, input_activations),
                        (output_weighted_sums, _),
                    ))
                | {
                    let gradient: Vec<f64> =
                        gradient
                            .iter()
                            .zip((
                                match activation_function {
                                    Some(af) => af.bprop(&output_weighted_sums),
                                    _ => vec![1.0; output_weighted_sums.len()],
                                }
                            ).iter())
                            .map(|(da, dz)| da * dz)
                            .collect();

                    let next_gradient = (0..input_activations.len())
                        .map(|i|
                            gradient
                                .iter()
                                .zip(self
                                    .parameters[layer_index]
                                    .iter()
                                    .map(|(weights, _)| weights[i])
                                )
                                .map(|(g, w)| g * w)
                                .sum::<f64>()
                        )
                        .collect::<Vec<f64>>();

                    self.parameters[layer_index] =
                        self.parameters[layer_index]
                            .iter()
                            .zip(gradient)
                            .map(|((weights, bias), g)| (
                                weights
                                    .iter()
                                    .zip(input_activations)
                                    .map(|(w, a)| w + g * a * learning_rate)
                                    .collect(),
                                bias + g * learning_rate,
                            ))
                            .collect();

                    next_gradient
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test;

    #[test]
    fn propagates_forward() {
        assert_eq!(
            test::get_test_network().fprop(&[-1.0, 1.0, 1.0]),
            vec![1.5, 2.5],
        );
    }

    #[test]
    fn propagates_backward() {
        let mut network = test::get_test_network();

        network.fprop(&[-1.0, 1.0, 1.0]);

        network.bprop(&[1.0, 0.5], 1.0);

        assert_eq!(
            network.parameters,
            vec![
                vec![
                    (vec![-0.5, 2.0, 2.0], 1.5),
                    (vec![1.0, 0.0, 0.0], 0.5),
                ],
                vec![
                    (vec![2.5, 1.0], 1.5),
                    (vec![2.0, 1.0], 1.0),
                ],
            ],
        );
    }
}
