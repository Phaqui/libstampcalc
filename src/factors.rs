use std::ops::AddAssign;
use num::{Integer, ToPrimitive};

pub struct Factors<T: Integer + Clone> {
    factors: Vec<T>,
    current: Vec<T>,
    iterations_left: usize,
}

impl<T: Integer + AddAssign + Clone> Iterator for Factors<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // 0,0 -> 0,1 -> 0,2 -> 0,3 -> 1,0 -> 1,1 -> ...
        if self.iterations_left == 0 { return None; }
        self.iterations_left -= 1;

        let ret = Some(self.current.clone());

        for i in (0..self.current.len()).rev() {
            self.current[i] += T::one();

            if self.current[i] > self.factors[i] {
                self.current[i] = T::zero();
            } else {
                break;
            }
        }

        return ret;
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.iterations_left;
        (l, Some(l))
    }
}

impl<T: Integer + AddAssign + Clone> core::iter::FusedIterator for Factors<T> {}

pub fn factors<T>(input: &Vec<T>) -> Factors<T>
where
    T: Integer + ToPrimitive + Clone,
{
    let len = input.len();
    let n_iterations_todo = if len == 0 {
        0
    } else {
        input.iter().map(|v| (*v).to_usize().unwrap() + 1).product()
    };

    Factors {
        current: vec![T::zero(); len],
        iterations_left: n_iterations_todo,
        factors: input.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zerolen() {
        let expected: Vec<Vec<u32>> = Vec::new();
        let actual: Vec<Vec<u32>> = factors(&vec![]).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_only_one_zero() {
        let iterator = factors(&vec![0]);
        let known = vec![vec![0]];
        let calculated_len = iterator.size_hint().0;
        let actual = iterator.collect::<Vec<Vec<_>>>();
        let actual_len = actual.len();
        assert_eq!(calculated_len, actual_len);
        assert_eq!(known.len(), actual_len);
        assert_eq!(actual, known);
    }

    #[test]
    fn test_single_dimension() {
        let expected = vec![vec![0], vec![1], vec![2], vec![3]];
        let actual: Vec<Vec<_>> = factors(&vec![3]).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiple_dimensions() {
        let iterator = factors(&vec![2, 3]);
        let known = vec![
            vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3],
            vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3],
            vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3],
        ];
        let expected_len = iterator.size_hint().0;
        let actual = iterator.collect::<Vec<Vec<_>>>();
        let actual_len = actual.len();

        assert_eq!(expected_len, actual_len);
        assert_eq!(expected_len, known.len());
        assert_eq!(actual, known);
    }
}
