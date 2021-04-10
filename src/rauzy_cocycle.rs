//! Module for the definition of the cocycle matrix data structure.
use rug::{Assign, Integer};

/// Struct representing a cocycle matrix. This does not necessarily have to be square,
/// but the default constructor will create the square identity matrix.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CocycleMatrix {
    size: usize,
    columns: Vec<Vec<Integer>>,
}

impl CocycleMatrix {
    /// Constructor for the identity cocycle of size n.
    pub fn new(size: usize) -> Self {
        let mut columns = Vec::new();
        for i in 0..size {
            let mut column = Vec::new();
            for j in 0..size {
                let mut entry = Integer::new();
                if i == j {
                    entry.assign(1);
                }
                column.push(entry);
            }
            columns.push(column);
        }
        Self { size, columns }
    }

    /// Replaces the ith column with the sum of the ith and jth columns.
    pub fn sum_two_columns(&mut self, i: usize, j: usize) -> Option<()> {
        if i >= self.size || j >= self.size {
            return None;
        }

        let num_rows = self.columns[0].len();
        for k in 0..num_rows {
            let to_add = self.columns[j][k].clone();
            self.columns[i][k] += to_add;
        }

        Some(())
    }
}
