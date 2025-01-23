use crate::LossFunction;

pub struct CrossEntropy;

impl LossFunction for CrossEntropy {
    fn fprop(&self, actual: &[f64], expected: &[f64]) -> Vec<f64> {
        actual
            .iter()
            .zip(expected.iter())
            .map(|(p, y)| -y * p.log10())
            .collect()
    }

    fn bprop(&self, actual: &[f64], expected: &[f64]) -> Vec<f64> {
        actual
            .iter()
            .zip(expected.iter())
            .map(|(p, y)| p - y)
            .collect()
    }
}
