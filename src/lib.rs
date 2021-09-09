mod factors;
mod vec_mulsum;
use crate::factors::{Factors, factors};
use vec_mulsum::vec_mulsum;

pub struct SolutionsIter {
    factors_iter: Factors,
    stamps: Vec<u32>,
    price: u32,
}

pub fn solutions_for_price<'a>(price: u32, stamps: &'a Vec<u32>)
    -> SolutionsIter
{
    let max_factors = stamps.into_iter().map(|s| price / s).collect::<Vec<_>>();
    let factors_iter = factors(&max_factors);

    SolutionsIter {
        factors_iter: factors_iter,
        stamps: stamps.clone(),
        price: price,
    }
}

impl Iterator for SolutionsIter {
    type Item = Vec<u32>;

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
