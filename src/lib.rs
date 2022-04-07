mod factors;
mod vec_mulsum;

use std::ops::AddAssign;
use std::iter::Sum;
use num::{Integer, ToPrimitive};
use crate::factors::{Factors, factors};
use crate::vec_mulsum::vec_mulsum;

pub struct SolutionsIter<T: Integer + Clone> {
    factors_iter: Factors<T>,
    stamps: Vec<T>,
    price: T,
}

pub fn solutions_for_price<'a, T>(price: T, stamps: &'a Vec<T>)
    -> SolutionsIter<T>
where
    T: Integer + ToPrimitive + Sum + Copy
{
    let max_factors = stamps.into_iter().map(|s| price / *s).collect::<Vec<_>>();

    SolutionsIter {
        factors_iter: factors(&max_factors),
        stamps: stamps.clone(),
        price: price,
    }
}

impl<T: Integer + AddAssign + Sum + Copy> Iterator for SolutionsIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_factors = self.factors_iter.next();
            match next_factors {
                None => break None,
                Some(factors) => {
                    if self.price == vec_mulsum(&factors, &self.stamps) {
                        break Some(factors.clone())
                    }
                }
            }
        }
    }
}
