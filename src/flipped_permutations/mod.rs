//! Module implementing [`FlippedPermutation`] and implementing [`super::GeneralizedPermutation`]
//! for the type.

use super::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use thiserror::Error;

type Interval = u8;

/// The possible errors that can occus when constructing a `FlippedPermutation`.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PermutationBuilderError {
    /// When the top and bottom do not have the same number of intervals.
    #[error("The top and bottom have different number of intervals")]
    UnmatchedInterval,

    /// When an index is repeated in the top or bottom vector.
    #[error("Interval index {0} repeated")]
    Repeated(Interval),

    /// When an index greater than or equal to the number of intervals occurs.
    #[error("Interval index {0} out of bounds")]
    OutOfBounds(Interval),

    /// When more intervals than the type can handle are specified.
    #[error("More than 256 intervals")]
    TooLarge,
}

/// Struct encoding a permutation with flips on $n$ intervals. The vectors `top` and `bottom`
/// contain the numbers $0$ to $n-1$ in some order, and the `flip_set` contains the intervals that
/// get flipped by the permutation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FlippedPermutation {
    top: Vec<Interval>,
    bottom: Vec<Interval>,
    flip_set: HashSet<Interval>,
}

impl FlippedPermutation {
    /// Constructor for a `FlippedPermutation`. The arguments `top` and `bottom` need to be `&[u8]`
    /// containing numbers $0$ to $n-1$ where $n$ is the length of `top` and `bottom`, and `flip_set`
    /// needs to be a `&[u8]` containing the intervals that get flipped; the order of appearance does
    /// not matter, since it gets turned into a `HashSet`.
    pub fn new(
        top: &[Interval],
        bottom: &[Interval],
        flip_set: &[Interval],
    ) -> Result<Self, PermutationBuilderError> {
        let length = match Interval::try_from(top.len()) {
            Ok(l) => l,
            Err(_) => {
                return Err(PermutationBuilderError::TooLarge);
            }
        };

        if top.len() != bottom.len() {
            return Err(PermutationBuilderError::UnmatchedInterval);
        }

        let mut top_vec = Vec::new();
        let mut bottom_vec = Vec::new();

        let mut top_seen: HashSet<Interval> = HashSet::new();
        let mut bottom_seen: HashSet<Interval> = HashSet::new();
        let flip_set: HashSet<Interval> =
            flip_set.iter().copied().filter(|x| *x < length).collect();

        for &index in top {
            if top_seen.contains(&index) {
                return Err(PermutationBuilderError::Repeated(index));
            } else {
                top_seen.insert(index);
            }

            if index >= length {
                return Err(PermutationBuilderError::OutOfBounds(index));
            }

            top_vec.push(index);
        }

        for &index in bottom {
            if bottom_seen.contains(&index) {
                return Err(PermutationBuilderError::Repeated(index));
            } else {
                bottom_seen.insert(index);
            }

            if index >= length {
                return Err(PermutationBuilderError::OutOfBounds(index));
            }

            bottom_vec.push(index);
        }

        Ok(Self {
            top: top_vec,
            bottom: bottom_vec,
            flip_set,
        })
    }
}

impl GeneralizedPermutation for FlippedPermutation {
    fn is_irreducible(&self) -> bool {
        let mut top_but_not_bottom: HashSet<u8> = HashSet::new();
        let mut bottom_but_not_top: HashSet<u8> = HashSet::new();

        for i in 0..(self.top.len() - 1) {
            let top_index = self.top[i];
            let bottom_index = self.bottom[i];

            top_but_not_bottom.insert(top_index);
            bottom_but_not_top.insert(bottom_index);

            if bottom_but_not_top.contains(&top_index) {
                top_but_not_bottom.remove(&top_index);
                bottom_but_not_top.remove(&top_index);
            }

            if top_but_not_bottom.contains(&bottom_index) {
                top_but_not_bottom.remove(&bottom_index);
                bottom_but_not_top.remove(&bottom_index);
            }

            if top_but_not_bottom.is_empty() && bottom_but_not_top.is_empty() {
                return false;
            }
        }
        true
    }

    fn rauzy_move(&self, winner: WinningSide) -> Self {
        let (current_top, current_bottom) = match winner {
            WinningSide::Top => (&self.top, &self.bottom),
            WinningSide::Bottom => (&self.bottom, &self.top),
        };
        let mut index = 0;

        let n = current_top.len();
        let new_top = current_top.clone();
        let mut new_bottom = Vec::new();
        let mut new_flip_set = self.flip_set.clone();

        let winning_interval = current_top[n - 1];
        let losing_interval = current_bottom[n - 1];

        for interval in current_bottom {
            if *interval == winning_interval {
                break;
            }
            new_bottom.push(*interval);
            index += 1;
        }

        if !self.flip_set.contains(&winning_interval) {
            new_bottom.push(winning_interval);
            new_bottom.push(losing_interval);
        } else {
            new_bottom.push(losing_interval);
            new_bottom.push(winning_interval);

            if self.flip_set.contains(&losing_interval) {
                new_flip_set.remove(&losing_interval);
            } else {
                new_flip_set.insert(losing_interval);
            }
        }

        for interval in &current_bottom[index + 1..n - 1] {
            new_bottom.push(*interval);
        }

        match winner {
            WinningSide::Top => Self {
                top: new_top,
                bottom: new_bottom,
                flip_set: new_flip_set,
            },
            WinningSide::Bottom => Self {
                top: new_bottom,
                bottom: new_top,
                flip_set: new_flip_set,
            },
        }
    }
}

#[cfg(test)]
mod test;
