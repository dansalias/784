use crate::parameters::Parameters;

pub fn fprop(parameters: &Parameters, input: &[f64]) {
    parameters
        .get_structured()
        .iter()
        .fold(input.to_vec(), |input, layer|
            layer
                .iter()
                .map(|(weights, bias)|
                    weights
                        .iter()
                        .zip(input.iter())
                        .map(|(w, i)| w * i)
                        .sum::<f64>()
                    + bias
                )
                .collect::<Vec<f64>>()
        );
}
