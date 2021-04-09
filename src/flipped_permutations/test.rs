use super::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_permutation_date_creator() {
    let top = [2, 0, 1];
    let bottom = [1, 2, 0];
    let flip_set = [1];

    assert_eq!(
        FlippedPermutation::new(&top, &bottom, &flip_set),
        Ok(FlippedPermutation {
            top: vec![2, 0, 1],
            bottom: vec![1, 2, 0],
            flip_set: HashSet::from_iter(flip_set.iter().cloned())
        })
    );

    let top = [3, 0, 2, 1];
    let bottom = [3, 2, 1];
    let flip_set = [];
    assert_eq!(
        FlippedPermutation::new(&top, &bottom, &flip_set),
        Err(PermutationBuilderError::UnmatchedInterval)
    );

    let top = [3, 0, 2, 1];
    let bottom = [3, 3, 2, 1];
    let flip_set = [];
    assert_eq!(
        FlippedPermutation::new(&top, &bottom, &flip_set),
        Err(PermutationBuilderError::Repeated(3))
    );

    let top = [4, 0, 2, 1];
    let bottom = [3, 3, 2, 1];
    let flip_set = [];
    assert_eq!(
        FlippedPermutation::new(&top, &bottom, &flip_set),
        Err(PermutationBuilderError::OutOfBounds(4))
    );
}

#[test]
fn test_is_irreducible() {
    let top = vec![2, 0, 1];
    let bottom = vec![1, 2, 0];
    let flip_set = vec![];
    let permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();
    assert!(permutation.is_irreducible());

    let top = vec![2, 0, 1];
    let bottom = vec![2, 0, 1];
    let flip_set = vec![];
    let permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();
    assert!(!permutation.is_irreducible());

    let top = vec![2, 0, 1, 3];
    let bottom = vec![1, 2, 0, 3];
    let flip_set = vec![];
    let permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();
    assert!(!permutation.is_irreducible());

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![];
    let permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();
    assert!(permutation.is_irreducible());

    let top = vec![1, 3, 0, 2];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![];
    let permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();
    assert!(!permutation.is_irreducible());
}

#[test]
fn test_rauzy_move() {
    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![];
    let old_permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 0, 1, 2];
    let flip_set = vec![];
    let top_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Top),
        top_winner_result
    );

    let top = vec![2, 0, 3, 1];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![];
    let bottom_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Bottom),
        bottom_winner_result
    );

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![3];
    let old_permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    let top = vec![2, 0, 1, 3];
    let bottom = vec![0, 3, 1, 2];
    let flip_set = vec![3, 0];
    let top_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Top),
        top_winner_result
    );

    let top = vec![2, 0, 3, 1];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![3];
    let bottom_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Bottom),
        bottom_winner_result
    );

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![0];
    let old_permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 0, 1, 2];
    let flip_set = vec![0];
    let top_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Top),
        top_winner_result
    );

    let top = vec![2, 3, 0, 1];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![3, 0];
    let bottom_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Bottom),
        bottom_winner_result
    );

    let top = vec![2, 0, 1, 3];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![0, 3];
    let old_permutation = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    let top = vec![2, 0, 1, 3];
    let bottom = vec![0, 3, 1, 2];
    let flip_set = vec![3];
    let top_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Top),
        top_winner_result
    );

    let top = vec![2, 3, 0, 1];
    let bottom = vec![3, 1, 2, 0];
    let flip_set = vec![0];
    let bottom_winner_result = FlippedPermutation::new(&top, &bottom, &flip_set).unwrap();

    assert_eq!(
        old_permutation.rauzy_move(WinningSide::Bottom),
        bottom_winner_result
    );
}
