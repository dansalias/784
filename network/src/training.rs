pub mod report;
pub mod backpropagation;

use crate::{data::{Data, DataPoint}, LossFunction, Network};
use backpropagation::Gradient;
use rand::{seq::SliceRandom, thread_rng};

pub struct TrainingOptions {
    pub batch_size: usize,
    pub epoch_count: usize,
    pub learning_rate: f64,
    pub loss_function: Box<dyn LossFunction>,
}

pub fn train(
    network: &mut Network,
    data: Data,
    options: TrainingOptions,
) {
    let reporter = report::Reporter::new();

    let mut accuracy = None;

    for epoch in 1..=options.epoch_count {
        let mut training_data: Vec<&DataPoint> = data.train.iter().collect();

        training_data.shuffle(&mut thread_rng());

        let batches = training_data.chunks(options.batch_size);

        let iteration_count = training_data.len() / options.batch_size;

        for (iteration, mini_batch) in batches.enumerate() {
            let mut gradients: Vec<Gradient> = vec![];

            for data_point in mini_batch {
                let output = network.fprop(&data_point.input);

                gradients.push(
                    backpropagation::get_gradient(
                        &network,
                        &options.loss_function.bprop(
                            &output,
                            &data_point.expected,
                        ),
                    ),
                );
            }

            for gradient in gradients {
                for layer_index in 0..network.parameters.len() {
                    for neuron_index in 0..network.parameters[layer_index].len() {
                        network.parameters[layer_index][neuron_index] = (
                            network.parameters[layer_index][neuron_index].0
                                .iter()
                                .enumerate()
                                .map(|(i, w)|
                                    w -
                                        gradient[layer_index][neuron_index].0[i]
                                        * options.learning_rate
                                        / mini_batch.len() as f64
                                )
                                .collect(),
                            network.parameters[layer_index][neuron_index].1 -
                                gradient[layer_index][neuron_index].1
                                * options.learning_rate
                                / mini_batch.len() as f64,
                        );
                    }
                }
            }

            if iteration % 10 == 0 {
                accuracy = Some(
                    data.test
                        .iter()
                        .map(|data_point| {
                            let output = network.fprop(&data_point.input);
                            let estimate = output
                                .iter()
                                .enumerate()
                                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                                .map(|(i, _)| i)
                                .unwrap();
                            let actual = &data_point
                                .expected
                                .iter()
                                .position(|el| el == &1.0)
                                .unwrap();

                            if estimate == *actual {
                                1.0
                            } else {
                                0.0
                            }
                        })
                        .sum::<f64>()
                        / data.test.len() as f64
                );
            }

            reporter.report(report::ReportData {
                accuracy,
                epoch,
                epoch_count: options.epoch_count,
                iteration,
                iteration_count,
            });
        }
    }
}
