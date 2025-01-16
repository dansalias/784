use crate::ActivationFunction;

pub struct Relu;

impl ActivationFunction for Relu {
    fn fprop(&self, values: &[f64]) -> Vec<f64> {
        values.iter().map(|v| f64::max(0.0, *v)).collect()
    }

    fn bprop(&self, values: &[f64]) -> Vec<f64> {
        values
            .iter()
            .map(|v| if *v <= 0.0 { 0.0 } else { 1.0 })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fprop_negative() {
        assert_eq!(Relu.fprop(&[-2.0]), vec![0.0]);
    }

    #[test]
    fn fprop_zero() {
        assert_eq!(Relu.fprop(&[0.0]), vec![0.0]);
    }

    #[test]
    fn fprop_positive() {
        assert_eq!(Relu.fprop(&[2.0]), vec![2.0]);
    }

    #[test]
    fn bprop_negative() {
        assert_eq!(Relu.bprop(&[-2.0]), vec![0.0]);
    }

    #[test]
    fn bprop_zero() {
        assert_eq!(Relu.bprop(&[0.0]), vec![0.0]);
    }

    #[test]
    fn bprop_positive() {
        assert_eq!(Relu.bprop(&[2.0]), vec![1.0]);
    }
}
