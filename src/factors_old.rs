pub struct Factors {
    current: Vec<u32>,
    factors: Vec<u32>,
}

impl Iterator for Factors {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.len() == 0 {
            return None;
        }

        self.current[0] += 1;

        if self.current[0] <= self.factors[0] {
            return Some(self.current.clone());
        }

        for i in 0..self.factors.len() - 1 {
            if self.current[i] > self.factors[i] {
                self.current[i] = 0;
                self.current[i + 1] += 1;
            }

            if self.current[i + 1] <= self.factors[i + 1] {
                return Some(self.current.clone());
            }
        }

        None
    }
}

pub fn factors(inp: &Vec<u32>) -> Factors {
    Factors {
        current: vec![0; inp.len()],
        factors: inp.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zerolen() {
        let expected: Vec<Vec<u32>> = Vec::new();
        let actual = factors(&vec![]).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single() {
        let expected = vec![vec![1], vec![2], vec![3]];
        let actual = factors(&vec![3]).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_contents() {
        let expected = vec![
            vec![1, 0], vec![2, 0],
            vec![0, 1], vec![1, 1], vec![2, 1],
            vec![0, 2], vec![1, 2], vec![2, 2],
            vec![0, 3], vec![1, 3], vec![2, 3],
        ];
        let actual = factors(&vec![2, 3]).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
