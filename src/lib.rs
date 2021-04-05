//! Library for working with interval exchange transformations.
//!
//! ## Usage
//! TO ADD

#![warn(missing_docs, unused_variables, rust_2018_idioms)]

use std::collections::HashSet;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PermutationBuilderError {
    #[error("The top and bottom have different number of intervals")]
    UnmatchedInterval,

    #[error("Interval index {0} repeated")]
    Repeated(u8),

    #[error("Interval index {0} out of bounds")]
    OutOfBounds(u8),

    #[error("More than 256 intervals")]
    TooLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlipData {
    Flipped,
    NotFlipped,
}

#[derive(Debug, PartialEq, Eq)]
struct PermutationData {
    top: Vec<u8>,
    bottom: Vec<(u8, FlipData)>,
}

impl PermutationData {
    fn new(top: &[u8], bottom: &[u8], flip_set: &[u8]) -> Result<Self, PermutationBuilderError> {
        let length = match u8::try_from(top.len()) {
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

        let mut top_seen: HashSet<u8> = HashSet::new();
        let mut bottom_seen: HashSet<u8> = HashSet::new();
        let flip_set: HashSet<u8> = flip_set.into_iter().map(|x| *x).collect();

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

            if flip_set.contains(&index) {
                bottom_vec.push((index, FlipData::Flipped));
            } else {
                bottom_vec.push((index, FlipData::NotFlipped));
            }
        }

        Ok(Self {
            top: top_vec,
            bottom: bottom_vec,
        })
    }
}

#[cfg(test)]
mod test {
    use super::{FlipData, PermutationBuilderError, PermutationData};

    #[test]
    fn test_permutation_date_creator() {
        let top = [2, 0, 1];
        let bottom = [1, 2, 0];
        let flip_set = [1];

        assert_eq!(
            PermutationData::new(&top, &bottom, &flip_set),
            Ok(PermutationData {
                top: vec![2, 0, 1],
                bottom: vec![
                    (1, FlipData::Flipped),
                    (2, FlipData::NotFlipped),
                    (0, FlipData::NotFlipped)
                ]
            })
        );

        let top = [3, 0, 2, 1];
        let bottom = [3, 2, 1];
        let flip_set = [];
        assert_eq!(
            PermutationData::new(&top, &bottom, &flip_set),
            Err(PermutationBuilderError::UnmatchedInterval)
        );

        let top = [3, 0, 2, 1];
        let bottom = [3, 3, 2, 1];
        let flip_set = [];
        assert_eq!(
            PermutationData::new(&top, &bottom, &flip_set),
            Err(PermutationBuilderError::Repeated(3))
        );

        let top = [4, 0, 2, 1];
        let bottom = [3, 3, 2, 1];
        let flip_set = [];
        assert_eq!(
            PermutationData::new(&top, &bottom, &flip_set),
            Err(PermutationBuilderError::OutOfBounds(4))
        );
    }
}
