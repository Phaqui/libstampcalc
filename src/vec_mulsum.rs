pub fn vec_mulsum(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let zipped = a.into_iter().zip(b.into_iter());
    zipped.map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(0, vec_mulsum(&vec![], &vec![]));
    }

    #[test]
    fn test_single() {
        assert_eq!(50, vec_mulsum(&vec![5], &vec![10]));
    }

    #[test]
    fn test_multi() {
        assert_eq!(68, vec_mulsum(&vec![3, 5, 7], &vec![2, 4, 6]));
    }

    #[test]
    fn test_one_is_longer() {
        let a1 = vec![3, 5, 7, 100, 200, 300];
        let b1 = vec![2, 4, 6];
        let a2 = vec![3, 5, 7];
        let b2 = vec![2, 4, 6, 100, 200, 300];
        let ans1 = vec_mulsum(&a1, &b1);
        let ans2 = vec_mulsum(&a2, &b2);
        assert_eq!(ans1, ans2);
    }
}
