use network::*;

fn main() {
    let mut network = Network::new(
        784,
        vec![
            (300, Some(Box::new(math::Relu))),
            (100, Some(Box::new(math::Relu))),
            (10, Some(Box::new(math::Softmax))),
        ],
    );

    network.parameters = parameters::Parameters::new(vec![784, 300, 100, 10])
        .initialize(parameters::Initialization::Kaiming);

    training::train(
        &mut network,
        data::Data {
            test: data::get_data(data::Dataset::Test, None),
            train: data::get_data(data::Dataset::Train, None),
        },
        training::TrainingOptions {
            batch_size: 32,
            epoch_count: 8,
            learning_rate: 0.01,
            loss_function: Box::new(math::CrossEntropy),
        },
    );

    parameters::binary::write_file(
        "./parameters.bin",
        &network.parameters.raw,
        &[784, 300, 100, 10],
    ).expect("unable to write parameters");
}
