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

pub fn structure_from_layers(input_size: usize, layers: &[Layer]) -> Vec<usize> {
    std::iter::once(input_size)
        .chain(
            layers
                .iter()
                .map(|(size, _)| *size)
        )
        .collect()
}

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
            parameters: parameters::Parameters::new(
                structure_from_layers(input_size, &layers),
            ),
            layers,
            activations: vec![],
        }
    }

    pub fn fprop(&mut self, input: &[f64]) -> Vec<f64> {
        self.activations = vec![(
            input.to_vec(),
            input.to_vec(),
        )];

        parameters::convert::to_structured(
            &self.parameters.raw,
            structure_from_layers(self.input_size, &self.layers),
        )
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
}

#[cfg(test)]
mod tests {
    use crate::util::test;

    #[test]
    fn propagates_forward() {
        assert_eq!(
            test::get_test_network().fprop(&[-1.0, 1.0, 1.0]),
            vec![1.5, 2.5],
        );
    }
}
