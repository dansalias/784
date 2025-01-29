pub fn pairs(slice: &[usize]) -> impl Iterator<Item = (&usize, &usize)> {
    slice[..slice.len() - 1]
        .iter()
        .zip(slice[1..].iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        assert_eq!(
            pairs(&[1, 2, 3, 4]).collect::<Vec<(&usize, &usize)>>(),
            vec![(&1, &2), (&2, &3), (&3, &4)],
        );
    }
}
