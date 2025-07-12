use crate::util;

type Structure = Vec<usize>;

pub struct Parameters {
    structure: Structure,
    raw: Vec<f64>,
}

impl Parameters {
    pub fn new(structure: Structure) -> Parameters {
        let parameter_size: usize =
            util::windows(&structure)
                .map(|(input_size, output_size)|
                    input_size * output_size + output_size
                )
                .sum();


        Parameters {
            structure,
            raw: util::vec_rand(parameter_size, (-1.0, 1.0)),
        }
    }

    pub fn get_structured(&self) -> Vec<Vec<(Vec<f64>, f64)>> {
        let mut cursor = 0;
        let mut structured = vec![];

        for (&input_size, &output_size) in util::windows(&self.structure) {
            let mut layer = vec![];

            for _ in 0..output_size {
                let weights = self.raw[cursor..cursor + input_size].to_vec();

                cursor += input_size;

                let bias = self.raw[cursor];

                cursor += 1;

                layer.push((weights, bias));
            }

            structured.push(layer);
        }

        structured
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parameters() {
        let parameters = Parameters::new(vec![3, 2, 2]);

        assert_eq!(
            parameters.raw.len(),
            3 * 2 + 2 + 2 * 2 + 2,
        );
    }
}
