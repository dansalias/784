use crate::{
    math,
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

    network.parameters = vec![
        vec![
            (vec![0.5, 1.0, 1.0], 0.5),
            (vec![1.0, 0.0, 0.0], 0.5),
        ],
        vec![
            (vec![0.5, 1.0], 0.5),
            (vec![1.0, 1.0], 0.5),
        ],
    ];

    network
}
