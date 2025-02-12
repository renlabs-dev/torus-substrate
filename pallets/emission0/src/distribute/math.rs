// This is necessary because functions like `f64::{fract, floor, ceil}`
// are only implemented in `std`.
#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;
use polkadot_sdk::{
    sp_runtime::{traits::Saturating, FixedI128, FixedPointNumber},
    sp_std::vec,
    sp_std::vec::Vec,
};
use substrate_fixed::types::I96F32;

pub fn normalize(x: Vec<I96F32>) -> Vec<I96F32> {
    let sum: I96F32 = x.iter().sum();
    normalize_with_sum(x, sum)
}

#[inline]
pub fn normalize_with_sum(mut x: Vec<I96F32>, sum: I96F32) -> Vec<I96F32> {
    if sum == I96F32::from_num(0.) {
        return x;
    }

    for ele in &mut x {
        *ele = ele.saturating_div(sum);
    }

    x
}

#[allow(dead_code)]
pub fn inplace_row_normalize_64(x: &mut [Vec<I96F32>]) {
    for row in x {
        let row_sum: I96F32 = row.iter().sum();
        if row_sum > I96F32::from_num(0.0_f64) {
            row.iter_mut().for_each(|x_ij: &mut I96F32| {
                *x_ij = x_ij.checked_div(row_sum).unwrap_or_default();
            });
        }
    }
}

pub fn matmul_sparse(
    sparse_matrix: &[Vec<(usize, I96F32)>],
    vector: &[I96F32],
    columns: usize,
) -> Vec<I96F32> {
    let mut result: Vec<I96F32> = vec![I96F32::from_num(0.0); columns];
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
    mut sparse_matrix: Vec<Vec<(usize, I96F32)>>,
    columns: usize,
) -> Vec<Vec<(usize, I96F32)>> {
    let mut col_sum: Vec<I96F32> = vec![I96F32::from_num(0); columns]; // assume square matrix, rows=cols

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
            if *col_sum_j == I96F32::from_num(0.) {
                continue;
            }
            *value = value.saturating_div(*col_sum_j);
        }
    }

    sparse_matrix
}

pub fn row_hadamard_sparse(
    sparse_matrix: &[Vec<(usize, I96F32)>],
    vector: &[I96F32],
) -> Vec<Vec<(usize, I96F32)>> {
    let mut result: Vec<Vec<(usize, I96F32)>> = sparse_matrix.to_vec();

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
    sparse_matrix: &[Vec<(usize, I96F32)>],
    vector: &[I96F32],
) -> Vec<I96F32> {
    let mut result: Vec<I96F32> = vec![I96F32::from_num(0); sparse_matrix.len()];
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
pub fn vec_max_upscale_to_u16(vec: &[I96F32]) -> Vec<u16> {
    let u16_max = I96F32::from_num(u16::MAX);
    let threshold = I96F32::from_num(32768);
    let Some(max_val) = vec.iter().max() else {
        return vec![0; vec.len()];
    };

    if *max_val == I96F32::from_num(0) {
        vec.iter()
            .map(|e| e.saturating_mul(u16_max).to_num())
            .collect()
    } else if *max_val > threshold {
        let ratio =
            FixedI128::from_u32(u16::MAX as u32).div(FixedI128::from_inner(max_val.to_num()));

        vec.iter()
            .map(|e| FixedI128::from_inner(e.to_num()))
            .map(|e| {
                e.saturating_mul(ratio)
                    .round()
                    .trunc()
                    .into_inner()
                    .checked_div(FixedI128::DIV)
                    .unwrap_or_default() as u16
            })
            .collect()
    } else {
        vec.iter()
            .map(|e| {
                e.saturating_mul(u16_max)
                    .saturating_div(*max_val)
                    .round()
                    .to_num()
            })
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
mod tests {
    use substrate_fixed::types::{I96F32, U64F64};

    use super::*;

    macro_rules! fixed_vec {
        () => {
            vec![]
        };
        ($($x:expr),+ $(,)?) => {
            vec![$(I96F32::from_num($x)),+]
        };
    }

    /// Reshape vector to sparse matrix with specified number of input rows,
    /// cast f32 to I96F32.
    fn vec_to_sparse_mat_fixed(
        vector: &[f32],
        rows: usize,
        transpose: bool,
    ) -> Vec<Vec<(u16, I96F32)>> {
        assert!(
            vector.len() % rows == 0,
            "Vector of len {:?} cannot reshape to {rows} rows.",
            vector.len()
        );
        let cols: usize = vector.len() / rows;
        let mut mat: Vec<Vec<(u16, I96F32)>> = vec![];
        if transpose {
            for col in 0..cols {
                let mut row_vec: Vec<(u16, I96F32)> = vec![];
                for row in 0..rows {
                    if vector[row * cols + col] > 0. {
                        row_vec.push((row as u16, I96F32::from_num(vector[row * cols + col])));
                    }
                }
                mat.push(row_vec);
            }
        } else {
            for row in 0..rows {
                let mut row_vec: Vec<(u16, I96F32)> = vec![];
                for col in 0..cols {
                    if vector[row * cols + col] > 0. {
                        row_vec.push((col as u16, I96F32::from_num(vector[row * cols + col])));
                    }
                }
                mat.push(row_vec);
            }
        }
        mat
    }

    fn assert_float_compare(a: I96F32, b: I96F32, epsilon: I96F32) {
        assert!(I96F32::abs(a - b) <= epsilon, "a({a:?}) != b({b:?})");
    }

    fn assert_vec_compare(va: &[I96F32], vb: &[I96F32], epsilon: I96F32) {
        assert!(va.len() == vb.len());
        for (a, b) in va.iter().zip(vb.iter()) {
            assert_float_compare(*a, *b, epsilon);
        }
    }

    fn assert_sparse_mat_compare(
        ma: &[Vec<(u16, I96F32)>],
        mb: &[Vec<(u16, I96F32)>],
        epsilon: I96F32,
    ) {
        assert!(ma.len() == mb.len());
        for row in 0..ma.len() {
            assert!(ma[row].len() == mb[row].len());
            for j in 0..ma[row].len() {
                assert!(ma[row][j].0 == mb[row][j].0); // u16
                assert_float_compare(ma[row][j].1, mb[row][j].1, epsilon) // I96F32
            }
        }
    }

    #[test]
    fn test_math_u64_normalization() {
        let min: u64 = 1;
        let mid: u64 = 10_500_000_000_000_000;
        let max: u64 = 21_000_000_000_000_000;
        let min_64: I96F32 = I96F32::from_num(min);
        let mid_64: I96F32 = I96F32::from_num(mid);
        let max_64: I96F32 = I96F32::from_num(max);
        let max_sum: I96F32 = I96F32::from_num(max);
        let min_frac: I96F32 = min_64 / max_sum;
        assert_eq!(min_frac, I96F32::from_num(0.0000000000000000476));
        let half: I96F32 = mid_64 / max_sum;
        assert_eq!(half, I96F32::from_num(0.5));
        let one: I96F32 = max_64 / max_sum;
        assert_eq!(one, I96F32::from_num(1));
    }

    #[test]
    fn test_math_to_num() {
        let val: I96F32 = I96F32::from_num(u16::MAX);
        let res: u16 = val.to_num::<u16>();
        assert_eq!(res, u16::MAX);
        let vector: Vec<I96F32> = vec![val; 1000];
        let target: Vec<u16> = vec![u16::MAX; 1000];
        let output: Vec<u16> = vector.iter().map(|e: &I96F32| e.to_num::<u16>()).collect();
        assert_eq!(output, target);
        let output: Vec<u16> = vector
            .iter()
            .map(|e: &I96F32| (*e).to_num::<u16>())
            .collect();
        assert_eq!(output, target);
        let val: I96F32 = I96F32::max_value();
        let res: u128 = val.to_num::<u128>();
        let vector: Vec<I96F32> = vec![val; 1000];
        let target: Vec<u128> = vec![res; 1000];
        let output: Vec<u128> = vector.iter().map(|e: &I96F32| e.to_num()).collect();
        assert_eq!(output, target);
        let output: Vec<u128> = vector.iter().map(|e: &I96F32| e.to_num()).collect();
        assert_eq!(output, target);
        let val: I96F32 = I96F32::from_num(0);
        let res: u64 = val.to_num::<u64>();
        let vector: Vec<I96F32> = vec![val; 1000];
        let target: Vec<u64> = vec![res; 1000];
        let output: Vec<u64> = vector.iter().map(|e: &I96F32| e.to_num::<u64>()).collect();
        assert_eq!(output, target);
        let output: Vec<u64> = vector
            .iter()
            .map(|e: &I96F32| (*e).to_num::<u64>())
            .collect();
        assert_eq!(output, target);
        let val: U64F64 = U64F64::from_num(u64::MAX);
        let res: u64 = val.to_num::<u64>();
        assert_eq!(res, u64::MAX);
        let vector: Vec<U64F64> = vec![val; 1000];
        let target: Vec<u64> = vec![u64::MAX; 1000];
        let output: Vec<u64> = vector.iter().map(|e: &U64F64| e.to_num::<u64>()).collect();
        assert_eq!(output, target);
        let output: Vec<u64> = vector
            .iter()
            .map(|e: &U64F64| (*e).to_num::<u64>())
            .collect();
        assert_eq!(output, target);
    }

    #[test]
    fn test_math_vec_to_sparse_mat_fixed() {
        let vector: Vec<f32> = vec![0., 1., 2., 0., 10., 100.];
        let target: Vec<Vec<(u16, I96F32)>> = vec![
            vec![(1, I96F32::from_num(1.)), (2, I96F32::from_num(2.))],
            vec![(1, I96F32::from_num(10.)), (2, I96F32::from_num(100.))],
        ];
        let mat = vec_to_sparse_mat_fixed(&vector, 2, false);
        assert_sparse_mat_compare(&mat, &target, I96F32::from_num(0));
        let vector: Vec<f32> = vec![0., 0.];
        let target: Vec<Vec<(u16, I96F32)>> = vec![vec![], vec![]];
        let mat = vec_to_sparse_mat_fixed(&vector, 2, false);
        assert_sparse_mat_compare(&mat, &target, I96F32::from_num(0));
        let vector: Vec<f32> = vec![0., 1., 2., 0., 10., 100.];
        let target: Vec<Vec<(u16, I96F32)>> = vec![
            vec![],
            vec![(0, I96F32::from_num(1.)), (1, I96F32::from_num(10.))],
            vec![(0, I96F32::from_num(2.)), (1, I96F32::from_num(100.))],
        ];
        let mat = vec_to_sparse_mat_fixed(&vector, 2, true);
        assert_sparse_mat_compare(&mat, &target, I96F32::from_num(0));
        let vector: Vec<f32> = vec![0., 0.];
        let target: Vec<Vec<(u16, I96F32)>> = vec![vec![]];
        let mat = vec_to_sparse_mat_fixed(&vector, 2, true);
        assert_sparse_mat_compare(&mat, &target, I96F32::from_num(0));
    }

    #[test]
    fn test_math_normalize() {
        let epsilon: I96F32 = I96F32::from_num(0.0001);
        let x: Vec<I96F32> = vec![];
        let y: Vec<I96F32> = normalize(x.clone());
        assert_vec_compare(&x, &y, epsilon);
        let x: Vec<I96F32> = fixed_vec![1.0, 10.0, 30.0,];
        let y: Vec<I96F32> = normalize(x.clone());
        assert_vec_compare(
            &y,
            &[
                I96F32::from_num(0.0243902437),
                I96F32::from_num(0.243902439),
                I96F32::from_num(0.7317073171),
            ],
            epsilon,
        );
        assert_float_compare(y.iter().sum(), I96F32::from_num(1.0), epsilon);
        let x: Vec<I96F32> = fixed_vec![-1.0, 10.0, 30.0];
        let y: Vec<I96F32> = normalize(x.clone());
        assert_vec_compare(
            &y,
            &[
                I96F32::from_num(-0.0256410255),
                I96F32::from_num(0.2564102563),
                I96F32::from_num(0.769230769),
            ],
            epsilon,
        );
        assert_float_compare(y.iter().sum(), I96F32::from_num(1.0), epsilon);
    }
}
