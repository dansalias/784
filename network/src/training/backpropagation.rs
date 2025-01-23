use crate::Network;

pub type Gradient = Vec<Vec<(Vec<f64>, f64)>>;

pub fn get_gradient(network: &Network, loss: &[f64]) -> Gradient {
    let mut gradient: Gradient = vec![];

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
                            .zip(network
                                .parameters[layer_index]
                                .iter()
                                .map(|(weights, _)| weights[i])
                            )
                            .map(|(g, w)| g * w)
                            .sum::<f64>()
                    )
                    .collect::<Vec<f64>>();

                gradient.insert(
                    0,
                    layer_loss
                        .iter()
                        .map(|g| (
                             input_activations
                                 .iter()
                                 .map(|a| g * a)
                                 .collect(),
                             *g,
                        ))
                        .collect(),
                );

                propagation
            }
        );

    gradient
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test;

    #[test]
    fn gets_gradient() {
        let mut network = test::get_test_network();

        network.fprop(&[-1.0, 1.0, 1.0]);

        assert_eq!(
            get_gradient(&network, &[1.0, 0.5]),
            vec![
                vec![
                    (vec![-1.0, 1.0, 1.0], 1.0),
                    (vec![0.0, 0.0, 0.0], 0.0),
                ],
                vec![
                    (vec![2.0, 0.0], 1.0),
                    (vec![1.0, 0.0], 0.5),
                ],
            ],
        )
    }
}
