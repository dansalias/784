use crate::structure_from_layers;
use crate::{parameters, Network};

pub type Gradient = Vec<f64>;

pub fn get_gradient(network: &Network, loss: &[f64]) -> Gradient {
    let mut gradient: Gradient = vec![];

    let parameters = parameters::convert::to_structured(
        &network.parameters.raw,
        structure_from_layers(network.input_size, &network.layers),
    );

    network
        .layers
        .iter()
        .enumerate()
        .zip(
            network
                .activations[..network.activations.len() - 1]
                .iter()
                .zip(network.activations[1..].iter())
        )
        .rfold(
            loss.to_vec(),
            |
                layer_loss,
                ((layer_index, (_, activation_function)), (
                    (_, input_activations),
                    (output_weighted_sums, _),
                ))
            | {
                let layer_loss: Vec<f64> = match activation_function {
                    Some(af) => layer_loss
                        .iter()
                        .zip(af.bprop(&output_weighted_sums).iter())
                        .map(|(da, dz)| da * dz)
                        .collect(),
                    _ => layer_loss,
                };

                let propagation = (0..input_activations.len())
                    .map(|i|
                        layer_loss
                            .iter()
                            .zip(
                                parameters[layer_index]
                                    .iter()
                                    .map(|(weights, _)| weights[i])
                            )
                            .map(|(g, w)| g * w)
                            .sum::<f64>()
                    )
                    .collect::<Vec<f64>>();

                gradient.extend(
                    layer_loss
                        .iter()
                        .flat_map(|l|
                            input_activations
                                .iter()
                                .map(move |a| l * a)
                                .chain(
                                    std::iter::once(*l)
                                )
                        )
                        .rev()
                );

                propagation
            }
        );

    gradient.reverse();
    gradient
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test;
    use crate::parameters;

    #[test]
    fn gets_gradient() {
        let mut network = test::get_test_network();

        network.fprop(&[-1.0, 1.0, 1.0]);

        assert_eq!(
            get_gradient(&network, &[1.0, 0.5]),
            parameters::convert::to_flat(&vec![
                vec![
                    (vec![-1.0, 1.0, 1.0], 1.0),
                    (vec![0.0, 0.0, 0.0], 0.0),
                ],
                vec![
                    (vec![2.0, 0.0], 1.0),
                    (vec![1.0, 0.0], 0.5),
                ],
            ]),
        );
    }
}
