// This is necessary because functions like `f64::{fract, floor, ceil}`
// are only implemented in `std`.
#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;
use polkadot_sdk::{
    sp_runtime::{traits::Saturating, FixedPointNumber, FixedU128},
    sp_std::vec,
    sp_std::vec::Vec,
};

pub fn normalize(x: Vec<FixedU128>) -> Vec<FixedU128> {
    let sum: FixedU128 = x
        .iter()
        .fold(FixedU128::default(), |acc, &e| acc.saturating_add(e));
    normalize_with_sum(x, sum)
}

#[inline]
pub fn normalize_with_sum(mut x: Vec<FixedU128>, sum: FixedU128) -> Vec<FixedU128> {
    if sum == FixedU128::from_inner(0) {
        return x;
    }

    for ele in &mut x {
        *ele = ele.const_checked_div(sum).unwrap_or_default();
    }

    x
}

#[allow(dead_code)]
pub fn inplace_row_normalize_64(x: &mut [Vec<FixedU128>]) {
    for row in x {
        let row_sum: FixedU128 = row
            .iter()
            .fold(FixedU128::default(), |acc, &e| acc.saturating_add(e));
        if row_sum > FixedU128::from_inner(0) {
            row.iter_mut().for_each(|x_ij: &mut FixedU128| {
                *x_ij = x_ij.const_checked_div(row_sum).unwrap_or_default();
            });
        }
    }
}

pub fn matmul_sparse(
    sparse_matrix: &[Vec<(usize, FixedU128)>],
    vector: &[FixedU128],
    columns: usize,
) -> Vec<FixedU128> {
    let mut result: Vec<FixedU128> = vec![FixedU128::from_inner(0); columns];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (j, value) in sparse_row.iter() {
            let Some(target) = result.get_mut(*j) else {
                continue;
            };
            let Some(vector_i) = vector.get(i) else {
                continue;
            };
            *target = target.saturating_add(vector_i.saturating_mul(*value));
        }
    }
    result
}

pub fn col_normalize_sparse(
    mut sparse_matrix: Vec<Vec<(usize, FixedU128)>>,
    columns: usize,
) -> Vec<Vec<(usize, FixedU128)>> {
    let mut col_sum: Vec<FixedU128> = vec![FixedU128::from_inner(0); columns]; // assume square matrix, rows=cols

    for sparse_row in &sparse_matrix {
        for (j, value) in sparse_row {
            let Some(col_sum_j) = col_sum.get_mut(*j) else {
                continue;
            };
            *col_sum_j = col_sum_j.saturating_add(*value);
        }
    }

    for sparse_row in &mut sparse_matrix {
        for (j, value) in sparse_row {
            let Some(col_sum_j) = col_sum.get(*j) else {
                continue;
            };
            if *col_sum_j != 0.into() {
                *value = value.const_checked_div(*col_sum_j).unwrap_or_default();
            }
        }
    }

    sparse_matrix
}

pub fn row_hadamard_sparse(
    sparse_matrix: &[Vec<(usize, FixedU128)>],
    vector: &[FixedU128],
) -> Vec<Vec<(usize, FixedU128)>> {
    let mut result: Vec<Vec<(usize, FixedU128)>> = sparse_matrix.to_vec();

    for (row_idx, sparse_row) in result.iter_mut().enumerate() {
        for (_, value) in sparse_row {
            let Some(vector_i) = vector.get(row_idx) else {
                continue;
            };
            *value = value.saturating_mul(*vector_i);
        }
    }

    result
}

pub fn matmul_transpose_sparse(
    sparse_matrix: &[Vec<(usize, FixedU128)>],
    vector: &[FixedU128],
) -> Vec<FixedU128> {
    let mut result: Vec<FixedU128> = vec![FixedU128::from_inner(0); sparse_matrix.len()];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (j, value) in sparse_row {
            // Compute dividends: d_j = SUM(i) b_ji * inc_i
            // result_j = SUM(i) vector_i * matrix_ji
            // result_i = SUM(j) vector_j * matrix_ij
            let Some(vector_j) = vector.get(*j) else {
                continue;
            };
            let Some(result_i) = result.get_mut(i) else {
                continue;
            };
            *result_i = result_i.saturating_add(vector_j.saturating_mul(*value))
        }
    }
    result
}

/// Max-upscale vector and convert to u16 so max_value = u16::MAX. Assumes
/// non-negative normalized input.
pub fn vec_max_upscale_to_u16(vec: &[FixedU128]) -> Vec<u16> {
    const MAX: FixedU128 = FixedU128::from_u32(u16::MAX as u32);

    let Some(max_ele) = vec.iter().max().filter(|x| **x != 0.into()) else {
        return vec![0; vec.len()];
    };

    let ratio = MAX.const_checked_div(*max_ele).unwrap_or_default();

    vec.iter()
        .map(|e| {
            e.saturating_mul(ratio)
                .round()
                .trunc()
                .into_inner()
                .checked_div(FixedU128::DIV)
                .unwrap_or_default() as u16
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
mod tests {
    use super::*;

    macro_rules! fixed_vec {
        () => {
            vec![]
        };
        ($($x:expr),+ $(,)?) => {
            vec![$(FixedU128::from_float($x)),+]
        };
    }

    /// Reshape vector to sparse matrix with specified number of input rows,
    /// cast f32 to FixedU128.
    fn vec_to_sparse_mat_fixed(
        vector: &[f64],
        rows: usize,
        transpose: bool,
    ) -> Vec<Vec<(u16, FixedU128)>> {
        assert!(
            vector.len() % rows == 0,
            "Vector of len {:?} cannot reshape to {rows} rows.",
            vector.len()
        );
        let cols: usize = vector.len() / rows;
        let mut mat: Vec<Vec<(u16, FixedU128)>> = vec![];
        if transpose {
            for col in 0..cols {
                let mut row_vec: Vec<(u16, FixedU128)> = vec![];
                for row in 0..rows {
                    if vector[row * cols + col] > 0. {
                        row_vec.push((row as u16, FixedU128::from_float(vector[row * cols + col])));
                    }
                }
                mat.push(row_vec);
            }
        } else {
            for row in 0..rows {
                let mut row_vec: Vec<(u16, FixedU128)> = vec![];
                for col in 0..cols {
                    if vector[row * cols + col] > 0. {
                        row_vec.push((col as u16, FixedU128::from_float(vector[row * cols + col])));
                    }
                }
                mat.push(row_vec);
            }
        }
        mat
    }

    fn assert_float_compare(a: FixedU128, b: FixedU128, epsilon: FixedU128) {
        dbg!(a, b, epsilon);
        assert!(
            a.into_inner().abs_diff(b.into_inner()) <= epsilon.into_inner(),
            "a({a:?}) != b({b:?})"
        );
    }

    fn assert_vec_compare(va: &[FixedU128], vb: &[FixedU128], epsilon: FixedU128) {
        assert!(va.len() == vb.len());
        for (a, b) in va.iter().zip(vb.iter()) {
            assert_float_compare(*a, *b, epsilon);
        }
    }

    fn assert_sparse_mat_compare(
        ma: &[Vec<(u16, FixedU128)>],
        mb: &[Vec<(u16, FixedU128)>],
        epsilon: FixedU128,
    ) {
        assert!(ma.len() == mb.len());
        for row in 0..ma.len() {
            assert!(ma[row].len() == mb[row].len());
            for j in 0..ma[row].len() {
                assert!(ma[row][j].0 == mb[row][j].0);
                assert_float_compare(ma[row][j].1, mb[row][j].1, epsilon)
            }
        }
    }

    #[test]
    fn test_math_u64_normalization() {
        let min: u128 = 1;
        let mid: u128 = 10_500_000_000_000_000;
        let max: u128 = 21_000_000_000_000_000;
        let min_64: FixedU128 = FixedU128::from_inner(min);
        let mid_64: FixedU128 = FixedU128::from_inner(mid);
        let max_64: FixedU128 = FixedU128::from_inner(max);
        let max_sum: FixedU128 = FixedU128::from_inner(max);
        let min_frac: FixedU128 = min_64 / max_sum;
        assert_eq!(min_frac, FixedU128::from_float(0.0000000000000000476));
        let half: FixedU128 = mid_64 / max_sum;
        assert_eq!(half, FixedU128::from_float(0.5));
        let one: FixedU128 = max_64 / max_sum;
        assert_eq!(one, FixedU128::from_u32(1));
    }

    #[test]
    fn test_math_vec_to_sparse_mat_fixed2() {
        let test_cases = vec![
            (
                vec![0., 1., 2., 0., 10., 100.],
                vec![
                    vec![(1, FixedU128::from_u32(1)), (2, FixedU128::from_u32(2))],
                    vec![(1, FixedU128::from_u32(10)), (2, FixedU128::from_u32(100))],
                ],
                false,
            ),
            (vec![0., 0.], vec![vec![], vec![]], false),
            (
                vec![0., 1., 2., 0., 10., 100.],
                vec![
                    vec![],
                    vec![(0, FixedU128::from_u32(1)), (1, FixedU128::from_u32(10))],
                    vec![(0, FixedU128::from_u32(2)), (1, FixedU128::from_u32(100))],
                ],
                true,
            ),
            (vec![0., 0.], vec![vec![]], true),
        ];

        for (vector, target, transpose) in test_cases {
            let mat = vec_to_sparse_mat_fixed(&vector, 2, transpose);
            assert_sparse_mat_compare(&mat, &target, FixedU128::from_inner(0));
        }
    }

    #[test]
    fn test_math_normalize2() {
        let epsilon = FixedU128::from_float(0.0001);

        let test_cases = vec![
            (vec![], vec![]),
            (
                fixed_vec![1.0, 10.0, 30.0],
                fixed_vec![0.0243902437, 0.243902439, 0.7317073171],
            ),
        ];

        for (x, expected_y) in test_cases {
            let y = normalize(x.clone());
            assert_vec_compare(&y, &expected_y, epsilon);

            if !y.is_empty() {
                let sum = y
                    .iter()
                    .fold(FixedU128::from_inner(0), |acc, &val| acc + val);
                assert_float_compare(sum, FixedU128::from_u32(1), epsilon);
            }
        }
    }
}
