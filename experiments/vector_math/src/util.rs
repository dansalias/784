use rand::Rng;

pub fn vec_rand(size: usize, bounds: (f64, f64)) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    (0..size)
        .map(|_| rng.gen_range(bounds.0..=bounds.1))
        .collect()
}

pub fn windows(slice: &[usize]) -> impl Iterator<Item = (&usize, &usize)> {
    slice[..slice.len() - 1]
        .iter()
        .zip(slice[1..].iter())
}
