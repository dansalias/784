use crate::{
    parameters::{ParametersFlat, ParametersStructured},
    util,
};

pub fn to_flat(
    structured: &ParametersStructured,
) -> ParametersFlat {
    structured
        .iter()
        .flat_map(|layer|
            layer
                .iter()
                .flat_map(|(weights, bias)|
                    weights
                        .iter()
                        .copied()
                        .chain(std::iter::once(*bias))
                )
        )
        .collect()
}

pub fn to_structured(
    flat: &[f64],
    structure: Vec<usize>,
) -> ParametersStructured {
    let mut cursor: usize = 0;
    let mut structured = vec![];

    for (&input_size, &output_size) in util::slice::pairs(&structure) {
        let mut layer = vec![];

        for _ in 0..output_size {
            let weights = flat[cursor..cursor + input_size].to_vec();

            cursor += input_size;

            let bias = flat[cursor];

            cursor += 1;

            layer.push((weights, bias));
        }

        structured.push(layer);
    }

    structured
}



#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> ParametersFlat {
        (1..=14)
            .map(|i| i as f64 / 10.0)
            .collect()
    }

    fn structured() -> ParametersStructured {
        vec![
            vec![
                (vec![0.1, 0.2, 0.3], 0.4),
                (vec![0.5, 0.6, 0.7], 0.8),
            ],
            vec![
                (vec![0.9, 1.0], 1.1),
                (vec![1.2, 1.3], 1.4),
            ],
        ]
    }

    #[test]
    fn test_to_flat() {
        assert_eq!(
            to_flat(&structured()),
            flat(),
        );
    }

    #[test]
    fn test_to_structured() {
        assert_eq!(
            to_structured(&flat(), vec![3, 2, 2]),
            structured(),
        );
    }
}
