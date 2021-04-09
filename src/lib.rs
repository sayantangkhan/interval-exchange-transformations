//! Library for working with interval exchange transformations.
//!
//! ## Outline
//! The trait [`GeneralizedPermutation`] is a trait modelling the actions one can perform on
//! permutations, which correspond to IETs, [`flipped_permutations::FlippedPermutation`],
//! which correspond to IETs with flips, and the corresponding versions for linear involutions
//! with and without flips. These actions include checking irreducibility, computing stratum,
//! performing Rauzy moves and other computations on the Rauzy graph, computing the J-invariant,
//! etc.
//!
//! Currently, only [`flipped_permutations::FlippedPermutation`] is implemented.

#![warn(missing_docs, unused_variables, rust_2018_idioms)]

pub mod flipped_permutations;

/// Trait modelling the actions one can perform on a generalized permutation.
/// Currently, only irreducibility checking and Rauzy moves are implemented.
pub trait GeneralizedPermutation {
    /// Checks whether permutation is irreducible.
    fn is_irreducible(&self) -> bool;
    /// Applies Rauzy move corresponding to [`WinningSide::Top`] or [`WinningSide::Bottom`] winning
    /// and returns the resulting [`GeneralizedPermutation`].
    fn rauzy_move(&self, winner: WinningSide) -> Self;
}

/// Enum encoding the side that wins in a Rauzy move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinningSide {
    /// Variant for the top interval
    Top,
    /// Variant for the bottom interval
    Bottom,
}
