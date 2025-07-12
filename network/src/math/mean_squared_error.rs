use crate::backprop::LossFunction;

pub struct MeanSquaredError;

impl LossFunction for MeanSquaredError {
    fn function(&self, values: &[f64], target: &[f64]) -> Vec<f64> {
        values
            .iter()
            .zip(target.iter())
            .map(|(v, t)| 0.5 * f64::powf(t - v, 2.0))
            .collect()
    }

    fn derivative(&self, values: &[f64], target: &[f64]) -> Vec<f64> {
        values
            .iter()
            .zip(target.iter())
            .map(|(v, t)| v - t)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function() {
        assert_eq!(
            MeanSquaredError.function(&[1.0, 2.0, 3.0], &[1.0, 1.0, 0.0],),
            vec![0.0, 0.5, 4.5],
        );
    }

    #[test]
    fn derivative() {
        assert_eq!(
            MeanSquaredError.derivative(&[1.0, 2.0, 3.0], &[1.0, 1.0, 0.0],),
            vec![0.0, 1.0, 3.0],
        );
    }
}
