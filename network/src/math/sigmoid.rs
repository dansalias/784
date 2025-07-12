use super::*;

pub struct Sigmoid;

impl ActivationFunction for Sigmoid {
    fn function(&self, values: Vec<f64>) -> Vec<f64> {
        values.iter().map(unit_sigmoid).collect()
    }

    fn derivative(&self, values: Vec<f64>) -> Vec<f64> {
        values
            .iter()
            .map(|v| unit_sigmoid(&v) * (1.0 - unit_sigmoid(&v)))
            .collect()
    }
}

fn unit_sigmoid(&value: &f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative() {
        assert_eq!(Sigmoid.function(vec![-1.0]), vec![0.2689414213699951]);
    }

    #[test]
    fn zero() {
        assert_eq!(Sigmoid.function(vec![0.0]), vec![0.5]);
    }

    #[test]
    fn positive() {
        assert_eq!(Sigmoid.function(vec![1.0]), vec![0.7310585786300049]);
    }

    #[test]
    fn d_negative() {
        assert_eq!(Sigmoid.derivative(vec![-1.0]), vec![0.19661193324148185]);
    }

    #[test]
    fn d_zero() {
        assert_eq!(Sigmoid.derivative(vec![0.0]), vec![0.25]);
    }

    #[test]
    fn d_positive() {
        assert_eq!(Sigmoid.derivative(vec![1.0]), vec![0.19661193324148185]);
    }
}
