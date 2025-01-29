use crate::{
    math,
    parameters,
    Network,
};

pub fn get_test_network() -> Network {
    let mut network = Network::new(
        3,
        vec![
            (2, Some(Box::new(math::Relu))),
            (2, None),
        ],
    );

    network.parameters =
        parameters::Parameters::new(vec![3, 2, 2])
            .initialize(parameters::Initialization::FromStructured(
                vec![
                    vec![
                        (vec![0.5, 1.0, 1.0], 0.5),
                        (vec![1.0, 0.0, 0.0], 0.5),
                    ],
                    vec![
                        (vec![0.5, 1.0], 0.5),
                        (vec![1.0, 1.0], 0.5),
                    ],
                ]
            ));

    network
}
