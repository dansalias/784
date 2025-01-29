pub fn el_sum(vector_list: Vec<Vec<f64>>) -> Vec<f64> {
    vector_list
        .into_iter()
        .reduce(|sum, vector| {
            sum
                .iter()
                .zip(vector.iter())
                .map(|(s, v)| s + v)
                .collect()
        })
        .unwrap()
}

pub fn el_mean(vector_list: Vec<Vec<f64>>) -> Vec<f64> {
    let len = vector_list.len() as f64;

    el_sum(vector_list)
        .into_iter()
        .map(|v| v / len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_el_sum() {
        assert_eq!(
            el_sum(vec![
                vec![1.0, 2.0, 3.0],
                vec![2.0, 3.0, 4.0],
            ]),
            vec![3.0, 5.0, 7.0],
        );
    }

    #[test]
    fn test_el_mean() {
        assert_eq!(
            el_mean(vec![
                vec![1.0, 2.0, 3.0],
                vec![2.0, 3.0, 4.0],
            ]),
            vec![1.5, 2.5, 3.5],
        );
    }
}
