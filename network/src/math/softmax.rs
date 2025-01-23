use crate::ActivationFunction;

pub struct Softmax;

impl ActivationFunction for Softmax {
    fn fprop(&self, values: &[f64]) -> Vec<f64> {
        let exponentials: Vec<f64> = values.iter().map(|v| v.exp()).collect();
        let sum: f64 = exponentials.iter().sum();

        exponentials
            .iter()
            .map(|v| v / sum)
            .collect()
    }

    fn bprop(&self, values: &[f64]) -> Vec<f64> {
        values
            .iter()
            .map(|v| v * (1.0 - v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        assert_eq!(
            Softmax.fprop(&[-1.0, 0.0, 1.0, 3.0]),
            vec![
                0.01521942886415593,
                0.04137069692096015,
                0.11245721367093253,
                0.83095266054395130,
            ]
        )
    }
}
