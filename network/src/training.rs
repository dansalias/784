pub mod report;
pub mod backpropagation;

use crate::{data::{Data, DataPoint}, LossFunction, Network, parameters, util};
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
    let mut results: Vec<(f64, f64, f64)> = vec![];

    let reporter = report::Reporter::new();

    let mut accuracy = None;

    for epoch in 1..=options.epoch_count {
        let mut training_data: Vec<&DataPoint> = data.train.iter().collect();

        training_data.shuffle(&mut thread_rng());

        let batches = training_data.chunks(options.batch_size);

        let iteration_count = training_data.len() / options.batch_size;

        let mut gradients: Vec<Gradient> = vec![];

        for (iteration, mini_batch) in batches.enumerate() {
            gradients.clear();

            for data_point in mini_batch {
                let output = network.fprop(&data_point.input);

                gradients.push(
                    backpropagation::get_gradient(
                        &network,

                        &output
                            .iter()
                            .zip(data_point.expected.iter())
                            .map(|(a, y)| a - y)
                            .collect::<Vec<f64>>()

                    ),
                );
            }

            let gradient = util::vector::el_mean(gradients.clone());

            // refactor: "raw" -> "flat"
            network.parameters.raw = network
                .parameters
                .raw
                .iter()
                .zip(gradient.iter())
                .map(|(p, g)| p - g * options.learning_rate)
                .collect();

            // feat: options.update_accuracy_frequency
            // tidy
            if iteration % 1000 == 0 {
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

        let test_accuracy =
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
                / data.test.len() as f64;

        let test_loss =
            data.test
                .iter()
                .map(|data_point| {
                    let output = network.fprop(&data_point.input);
                    options.loss_function.fprop(
                        &output,
                        &data_point.expected,
                    )
                    .iter()
                    .sum::<f64>()
                })
                .sum::<f64>()
                / data.test.len() as f64;

        let train_loss =
            data.train
                .iter()
                .map(|data_point| {
                    let output = network.fprop(&data_point.input);
                    options.loss_function.fprop(
                        &output,
                        &data_point.expected,
                    )
                    .iter()
                    .sum::<f64>()
                })
                .sum::<f64>()
                / data.train.len() as f64;

        let train_accuracy =
            data.train
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
                / data.train.len() as f64;

        results.push((
            test_loss,
            train_loss,
            train_loss - test_loss,
            // test_accuracy,
            // train_accuracy,
            // train_accuracy - test_accuracy,
        ));

        parameters::binary::write_file(
            format!("./parameters/parameters.e{}.bin", epoch).as_str(),
            &network.parameters.raw,
            &[784, 300, 100, 10],
        ).expect("unable to write parameters");
    }

    for result in results {
        println!("{}, {}, {}", result.0, result.1, result.2);
    }
}
