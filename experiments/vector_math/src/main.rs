use vector_math::parameters;
use vector_math::util;
use vector_math::_00_flat;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parameters = parameters::Parameters::new(
        vec![784, 300, 100, 10],
    );

    let input = util::vec_rand(784, (-1.0, 1.0));

    match args[1].as_str() {
        "flat" => _00_flat::fprop(&parameters, &input),
        _ => println!("{} not found", args[1]),
    }
}
