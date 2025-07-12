use divan::Bencher;
use vector_math::{_00_flat, parameters, util};

fn main() {
    divan::main();
}

#[derive(Debug)]
enum Implementation {
    Flat,
}

#[divan::bench(args = [
    Implementation::Flat,
])]
fn fprop(b: Bencher, implementation: &Implementation) {
    let parameters = parameters::Parameters::new(
        vec![784, 300, 100, 10],
    );

    let input = util::vec_rand(784, (-1.0, 1.0));

    let fprop = match implementation {
        Implementation::Flat => _00_flat::fprop,
    };

    b.bench(|| {
        fprop(&parameters, &input)
    });
}
