#![allow(dead_code)]

use polkadot_sdk::sp_std::{cmp::Ordering, collections::btree_map::BTreeMap, vec, vec::Vec};
use substrate_fixed::{
    transcendental::{exp, ln},
    types::I96F32,
};

// This is necessary because functions like `f64::{fract, floor, ceil}`
// are only implemented in `std`.
#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;

// Return true when vector sum is zero.
pub fn is_zero(vector: &[I96F32]) -> bool {
    vector.iter().sum::<I96F32>() == I96F32::from_num(0)
}

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

pub fn fixed64_to_u64(x: I96F32) -> u64 {
    x.to_num::<u64>()
}

pub fn vec_fixed64_to_u64(vec: Vec<I96F32>) -> Vec<u64> {
    vec.into_iter().map(fixed64_to_u64).collect()
}

pub fn matmul_64(matrix: &[Vec<I96F32>], vector: &[I96F32]) -> Vec<I96F32> {
    let Some(first_row) = matrix.first() else {
        return vec![];
    };
    let cols = first_row.len();
    if cols == 0 {
        return vec![];
    }
    assert!(matrix.len() == vector.len());
    matrix
        .iter()
        .zip(vector)
        .fold(vec![I96F32::from_num(0.0); cols], |acc, (row, vec_val)| {
            row.iter()
                .zip(acc)
                .map(|(m_val, acc_val)| {
                    // Compute ranks: r_j = SUM(i) w_ij * s_i
                    // Compute trust scores: t_j = SUM(i) w_ij * s_i
                    // result_j = SUM(i) vector_i * matrix_ij
                    acc_val
                        .checked_add(vec_val.checked_mul(*m_val).unwrap_or_default())
                        .unwrap_or(acc_val)
                })
                .collect()
        })
}

pub fn u16_proportion_to_fixed(x: u16) -> I96F32 {
    I96F32::from_num(x).saturating_div(I96F32::from_num(u16::MAX))
}

pub fn fixed_proportion_to_u16(x: I96F32) -> u16 {
    (x.saturating_mul(I96F32::from_num(u16::MAX))).to_num()
}

// Return a new sparse matrix with a masked out diagonal of input sparse matrix.
pub fn mask_diag_sparse(sparse_matrix: &[Vec<(u16, I96F32)>]) -> Vec<Vec<(u16, I96F32)>> {
    let n: usize = sparse_matrix.len();
    let mut result: Vec<Vec<(u16, I96F32)>> = vec![vec![]; n];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (j, value) in sparse_row.iter() {
            if i != (*j as usize) {
                let Some(row) = result.get_mut(i) else {
                    continue;
                };

                row.push((*j, *value));
            }
        }
    }
    result
}

pub fn mat_ema_alpha_vec_sparse(
    new: &[Vec<(u16, I96F32)>],
    old: &[Vec<(u16, I96F32)>],
    alpha: &[I96F32],
) -> Vec<Vec<(u16, I96F32)>> {
    assert_eq!(
        new.len(),
        old.len(),
        "New and old matrices must have the same number of rows"
    );
    let zero = I96F32::from_num(0.0);
    let one = I96F32::from_num(1.0);

    new.iter()
        .zip(old)
        .map(|(new_row, old_row)| {
            let mut row_map: BTreeMap<u16, I96F32> = new_row
                .iter()
                .map(|&(j, value)| {
                    let alpha_val = alpha.get(j as usize).copied().unwrap_or(zero);
                    (j, alpha_val.saturating_mul(value))
                })
                .collect();

            old_row.iter().for_each(|&(j, value)| {
                let alpha_val = alpha.get(j as usize).copied().unwrap_or(zero);
                let old_component = one.saturating_sub(alpha_val).saturating_mul(value);
                row_map
                    .entry(j)
                    .and_modify(|e| *e = e.saturating_add(old_component))
                    .or_insert(old_component);
            });

            row_map.into_iter().filter(|&(_, v)| v > zero).collect()
        })
        .collect()
}

pub fn calculate_logistic_params(
    alpha_high: I96F32,
    alpha_low: I96F32,
    consensus_high: I96F32,
    consensus_low: I96F32,
) -> (I96F32, I96F32) {
    if consensus_high <= consensus_low
        || alpha_low == I96F32::from_num(0)
        || alpha_high == I96F32::from_num(0)
    {
        return (I96F32::from_num(0), I96F32::from_num(0));
    }

    let calc_term = |alpha: I96F32| {
        ln((I96F32::from_num(1).saturating_div(alpha)).saturating_sub(I96F32::from_num(1)))
            .unwrap_or(I96F32::from_num(0.0))
    };

    let a = (calc_term(alpha_high).saturating_sub(calc_term(alpha_low)))
        .saturating_div(consensus_low.saturating_sub(consensus_high));
    let b = calc_term(alpha_low)
        .saturating_add(a)
        .saturating_mul(consensus_low);

    (a, b)
}

pub fn compute_alpha_values(consensus: &[I96F32], a: I96F32, b: I96F32) -> Vec<I96F32> {
    let alpha: Vec<I96F32> = consensus
        .iter()
        .map(|&c| {
            let exp_val =
                exp(b.saturating_sub(a.saturating_mul(c))).unwrap_or(I96F32::from_num(0.0));
            I96F32::from_num(1.0).saturating_div(I96F32::from_num(1.0).saturating_add(exp_val))
        })
        .collect();

    alpha
}

trait Lerp {
    fn lerp(self, other: Self, t: Self) -> Self;
}
impl Lerp for I96F32 {
    fn lerp(self, other: Self, t: Self) -> Self {
        self.saturating_add(other.saturating_sub(self))
            .saturating_mul(t)
    }
}

pub fn quantile(data: &[I96F32], quantile: f64) -> I96F32 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let len = sorted.len();
    if len == 0 {
        return I96F32::from_num(0);
    }
    let pos = quantile * (len.saturating_sub(1)) as f64;
    let (low, high) = (pos.floor() as usize, pos.ceil() as usize);
    if low == high {
        sorted.get(low).cloned().unwrap_or(I96F32::from_num(0))
    } else {
        sorted
            .get(low)
            .cloned()
            .unwrap_or(I96F32::from_num(0))
            .lerp(
                sorted.get(high).cloned().unwrap_or(I96F32::from_num(0)),
                I96F32::from_num(pos.fract()),
            )
    }
}

/// Normalizes (sum to 1 except 0) each row (dim=0) of a sparse matrix in-place.
pub fn inplace_row_normalize_sparse(sparse_matrix: &mut [Vec<(u16, I96F32)>]) {
    for sparse_row in sparse_matrix.iter_mut() {
        let row_sum: I96F32 = sparse_row.iter().map(|(_j, value)| *value).sum();
        if row_sum != I96F32::from_num(0) {
            sparse_row
                .iter_mut()
                .for_each(|(_j, value)| *value = value.saturating_div(row_sum));
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

pub fn linear_consensus(
    stake: &[I96F32],
    score: &[Vec<(u16, I96F32)>],
    columns: u16,
) -> Vec<I96F32> {
    let zero: I96F32 = I96F32::from_num(0);
    let mut consensus: Vec<I96F32> = vec![zero; columns as usize];
    let mut total_stake: I96F32 = zero;

    for (validator_stake, validator_score) in stake.iter().zip(score.iter()) {
        if *validator_stake <= zero {
            continue;
        }

        total_stake = total_stake.saturating_add(*validator_stake);

        for (column, value) in validator_score.iter() {
            if let Some(consensus_value) = consensus.get_mut(*column as usize) {
                *consensus_value =
                    consensus_value.saturating_add(value.saturating_mul(*validator_stake));
            }
        }
    }

    if total_stake > zero {
        for consensus_value in consensus.iter_mut() {
            *consensus_value = consensus_value.saturating_div(total_stake);
        }
    }

    consensus
}

pub fn weighted_median_col_sparse(
    stake: &[I96F32],
    score: &[Vec<(u16, I96F32)>],
    columns: u16,
    majority: I96F32,
) -> Vec<I96F32> {
    let rows = stake.len();
    let zero: I96F32 = I96F32::from_num(0);
    let use_stake = stake.iter().copied().filter(|&s| s > zero).collect();
    let use_stake = normalize(use_stake);

    let stake_sum: I96F32 = use_stake.iter().sum();
    let stake_idx: Vec<usize> = (0..use_stake.len()).collect();
    let minority: I96F32 = stake_sum.saturating_sub(majority);
    let mut use_score: Vec<Vec<I96F32>> = vec![vec![zero; use_stake.len()]; columns as usize];

    let mut median: Vec<I96F32> = vec![zero; columns as usize];
    let mut k: usize = 0;

    for r in 0..rows {
        let Some(stake_r) = stake.get(r) else {
            continue;
        };
        let Some(score_r) = score.get(r) else {
            continue;
        };
        if *stake_r <= zero {
            continue;
        }
        for (c, val) in score_r.iter() {
            let Some(use_score_c) = use_score.get_mut(*c as usize) else {
                continue;
            };
            let Some(use_score_c_k) = use_score_c.get_mut(k) else {
                continue;
            };
            *use_score_c_k = *val;
        }
        k = k.saturating_add(1);
    }

    for c in 0..columns as usize {
        let Some(median_c) = median.get_mut(c) else {
            continue;
        };
        let Some(use_score_c) = use_score.get(c) else {
            continue;
        };
        *median_c = weighted_median(
            &use_stake,
            use_score_c,
            &stake_idx,
            minority,
            zero,
            stake_sum,
        );
    }

    median
}

// Stake-weighted median score finding algorithm, based on a mid pivot binary search.
// Normally a random pivot is used, but to ensure full determinism the mid point is chosen instead.
// Assumes relatively random score order for efficiency, typically less than O(nlogn) complexity.
//
// # Args:
// 	* 'stake': ( &Vec<I96F32> ):
//         - stake, assumed to be normalized.
//
// 	* 'score': ( &Vec<I96F32> ):
//         - score for which median is sought, 0 <= score <= 1
//
// 	* 'partition_idx' ( &Vec<usize> ):
// 		- indices as input partition
//
// 	* 'minority' ( I96F32 ):
// 		- minority_ratio = 1 - majority_ratio
//
// 	* 'partition_lo' ( I96F32 ):
// 		- lower edge of stake for partition, where partition is a segment [lo, hi] inside stake
//     integral [0, 1].
//
// 	* 'partition_hi' ( I96F32 ):
// 		- higher edge of stake for partition, where partition is a segment [lo, hi] inside stake
//     integral [0, 1].
//
// # Returns:
//     * 'median': ( I96F32 ):
//         - median via random pivot binary search.
//
pub fn weighted_median(
    stake: &Vec<I96F32>,
    score: &Vec<I96F32>,
    partition_idx: &[usize],
    minority: I96F32,
    partition_lo: I96F32,
    partition_hi: I96F32,
) -> I96F32 {
    let n = partition_idx.len();
    if n == 0 {
        return I96F32::from_num(0);
    }
    if n == 1 {
        let Some(partition_idx_0) = partition_idx.first() else {
            return I96F32::from_num(0);
        };
        let Some(score_idx) = score.get(*partition_idx_0) else {
            return I96F32::from_num(0);
        };
        return *score_idx;
    }
    assert!(stake.len() == score.len());
    let mid_idx: usize = n / 2;
    let Some(partition_idx_mid_idx) = partition_idx.get(mid_idx) else {
        return I96F32::from_num(0);
    };
    let Some(pivot) = score.get(*partition_idx_mid_idx) else {
        return I96F32::from_num(0);
    };
    let mut lo_stake: I96F32 = I96F32::from_num(0);
    let mut hi_stake: I96F32 = I96F32::from_num(0);
    let mut lower: Vec<usize> = vec![];
    let mut upper: Vec<usize> = vec![];
    for &idx in partition_idx.iter() {
        let Some(score_idx) = score.get(idx) else {
            continue;
        };
        let Some(stake_idx) = stake.get(idx) else {
            continue;
        };
        if *score_idx == *pivot {
            continue;
        }
        if *score_idx < *pivot {
            lo_stake = lo_stake.saturating_add(*stake_idx);
            lower.push(idx);
        } else {
            hi_stake = hi_stake.saturating_add(*stake_idx);
            upper.push(idx);
        }
    }
    if (partition_lo.saturating_add(lo_stake) <= minority)
        && (minority < partition_hi.saturating_sub(hi_stake))
    {
        return *pivot;
    } else if (minority < partition_lo.saturating_add(lo_stake)) && !lower.is_empty() {
        return weighted_median(
            stake,
            score,
            &lower,
            minority,
            partition_lo,
            partition_lo.saturating_add(lo_stake),
        );
    } else if (partition_hi.saturating_sub(hi_stake) <= minority) && !upper.is_empty() {
        return weighted_median(
            stake,
            score,
            &upper,
            minority,
            partition_hi.saturating_sub(hi_stake),
            partition_hi,
        );
    }
    *pivot
}

// Sum across each row (dim=0) of a sparse matrix.
pub fn row_sum_sparse(sparse_matrix: &[Vec<(u16, I96F32)>]) -> Vec<I96F32> {
    let rows = sparse_matrix.len();
    let mut result: Vec<I96F32> = vec![I96F32::from_num(0); rows];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (_j, value) in sparse_row.iter() {
            let Some(result_i) = result.get_mut(i) else {
                continue;
            };

            *result_i = result_i.saturating_add(*value);
        }
    }
    result
}

// Return sparse matrix with values above column threshold set to threshold value.
pub fn col_clip_sparse(
    sparse_matrix: &[Vec<(u16, I96F32)>],
    col_threshold: &[I96F32],
) -> Vec<Vec<(u16, I96F32)>> {
    let mut result: Vec<Vec<(u16, I96F32)>> = vec![vec![]; sparse_matrix.len()];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (j, value) in sparse_row.iter() {
            let Some(col_threshold_j) = col_threshold.get(*j as usize) else {
                continue;
            };
            let Some(result_i) = result.get_mut(i) else {
                continue;
            };

            if *col_threshold_j < *value {
                result_i.push((*j, *col_threshold_j));
            } else {
                result_i.push((*j, *value));
            }
        }
    }
    result
}

pub fn mask_rows_sparse(
    mask: &[bool],
    sparse_matrix: &[Vec<(u16, I96F32)>],
) -> Vec<Vec<(u16, I96F32)>> {
    if mask.is_empty() {
        return sparse_matrix.to_vec();
    }

    let n: usize = sparse_matrix.len();
    assert_eq!(n, mask.len());

    let mut result: Vec<Vec<(u16, I96F32)>> = vec![vec![]; n];
    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        let Some(mask_i) = mask.get(i) else {
            continue;
        };
        let Some(result_i) = result.get_mut(i) else {
            continue;
        };
        if !mask_i {
            result_i.clone_from(sparse_row);
        }
    }

    result
}

pub fn vec_mask_sparse_matrix(
    sparse_matrix: &[Vec<(u16, I96F32)>],
    first_vector: &[u64],
    second_vector: &[u64],
    mask_fn: impl Fn(u64, u64) -> bool,
) -> Option<Vec<Vec<(u16, I96F32)>>> {
    let n: usize = sparse_matrix.len();
    let mut result: Vec<Vec<(u16, I96F32)>> = vec![vec![]; n];

    for (i, sparse_row) in sparse_matrix.iter().enumerate() {
        for (j, value) in sparse_row.iter() {
            if !mask_fn(*first_vector.get(i)?, *second_vector.get(*j as usize)?) {
                let Some(result_i) = result.get_mut(i) else {
                    continue;
                };

                result_i.push((*j, *value));
            }
        }
    }

    Some(result)
}

pub fn inplace_mask_vector(mask: &[bool], vector: &mut [I96F32]) {
    if mask.is_empty() {
        return;
    }

    assert_eq!(mask.len(), vector.len());
    let zero: I96F32 = I96F32::from_num(0.0);

    vector
        .iter_mut()
        .enumerate()
        .filter(|(idx, _)| *mask.get(*idx).unwrap_or(&false))
        .for_each(|(_, v)| *v = zero);
}

pub fn inplace_normalize_64(x: &mut [I96F32]) {
    let x_sum: I96F32 = x.iter().sum();
    if x_sum == I96F32::from_num(0) {
        return;
    }

    for x in x {
        *x = x.saturating_div(x_sum);
    }
}

pub fn vec_fixed64_to_fixed32(vec: Vec<I96F32>) -> Vec<I96F32> {
    vec.into_iter().map(I96F32::from_num).collect()
}

pub fn is_topk(vector: &[I96F32], k: usize) -> Vec<bool> {
    let n: usize = vector.len();
    let mut result: Vec<bool> = vec![true; n];
    if n <= k {
        return result;
    }

    let mut idxs: Vec<usize> = (0..n).collect();
    idxs.sort_by_key(|&idx| *vector.get(idx).unwrap_or(&I96F32::from_num(0))); // ascending stable sort
    let Some(idxs_n_sub_k) = idxs.get(..(n.saturating_sub(k))) else {
        return result;
    };
    for idx in idxs_n_sub_k {
        let Some(result_idx) = result.get_mut(*idx) else {
            continue;
        };
        *result_idx = false;
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

pub fn mat_ema_sparse(
    new: &[Vec<(u16, I96F32)>],
    old: &[Vec<(u16, I96F32)>],
    alpha: I96F32,
) -> Vec<Vec<(u16, I96F32)>> {
    assert_eq!(new.len(), old.len());
    let n = new.len(); // assume square matrix, rows=cols
    let zero: I96F32 = I96F32::from_num(0.0);
    let one_minus_alpha: I96F32 = I96F32::from_num(1.0).saturating_sub(alpha);
    let mut result: Vec<Vec<(u16, I96F32)>> = vec![vec![]; n];
    for i in 0..new.len() {
        let mut row: Vec<I96F32> = vec![zero; n];
        let Some(new_i) = new.get(i) else {
            continue;
        };
        let Some(old_i) = old.get(i) else {
            continue;
        };
        for (j, value) in new_i.iter() {
            let Some(row_j) = row.get_mut(*j as usize) else {
                continue;
            };
            *row_j = row_j.saturating_add(alpha.saturating_mul(*value));
        }
        for (j, value) in old_i.iter() {
            let Some(row_j) = row.get_mut(*j as usize) else {
                continue;
            };
            *row_j = row_j.saturating_add(one_minus_alpha.saturating_mul(*value));
        }
        for (j, value) in row.iter().enumerate() {
            let Some(result_i) = result.get_mut(i) else {
                continue;
            };
            if *value > zero {
                result_i.push((j as u16, *value))
            }
        }
    }
    result
}

/// Max-upscale vector and convert to u16 so max_value = u16::MAX. Assumes non-negative normalized
/// input.
pub fn vec_max_upscale_to_u16(vec: &[I96F32]) -> Vec<u16> {
    let u16_max: I96F32 = I96F32::from_num(u16::MAX);
    let threshold: I96F32 = I96F32::from_num(32768);
    let max_value: Option<&I96F32> = vec.iter().max();
    match max_value {
        Some(val) => {
            if *val == I96F32::from_num(0) {
                return vec
                    .iter()
                    .map(|e: &I96F32| e.saturating_mul(u16_max).to_num::<u16>())
                    .collect();
            }
            if *val > threshold {
                return vec
                    .iter()
                    .map(|e: &I96F32| {
                        e.saturating_mul(u16_max.saturating_div(*val))
                            .round()
                            .to_num::<u16>()
                    })
                    .collect();
            }
            vec.iter()
                .map(|e: &I96F32| {
                    e.saturating_mul(u16_max)
                        .saturating_div(*val)
                        .round()
                        .to_num::<u16>()
                })
                .collect()
        }
        None => {
            let sum: I96F32 = vec.iter().sum();
            vec.iter()
                .map(|e: &I96F32| {
                    e.saturating_mul(u16_max)
                        .saturating_div(sum)
                        .to_num::<u16>()
                })
                .collect()
        }
    }
}

pub fn vecdiv(x: &[I96F32], y: &[I96F32]) -> Vec<I96F32> {
    assert_eq!(x.len(), y.len());
    let n = x.len();
    let mut result: Vec<I96F32> = vec![I96F32::from_num(0); n];
    for i in 0..n {
        let Some(y_i) = y.get(i) else {
            continue;
        };
        if *y_i != I96F32::from_num(0.) {
            let Some(result_i) = result.get_mut(i) else {
                continue;
            };
            let Some(x_i) = x.get(i) else {
                continue;
            };
            *result_i = x_i.saturating_div(*y_i);
        }
    }
    result
}

// Max-upscale each column (dim=1) of a sparse matrix in-place.
pub fn inplace_col_max_upscale_sparse(sparse_matrix: &mut [Vec<(u16, I96F32)>], columns: u16) {
    let mut col_max: Vec<I96F32> = vec![I96F32::from_num(0.0); columns as usize]; // assume square matrix, rows=cols
    for sparse_row in sparse_matrix.iter() {
        for (j, value) in sparse_row.iter() {
            let Some(col_max_j) = col_max.get_mut(*j as usize) else {
                continue;
            };
            if *col_max_j < *value {
                *col_max_j = *value;
            }
        }
    }
    for sparse_row in sparse_matrix.iter_mut() {
        for (j, value) in sparse_row.iter_mut() {
            let Some(col_max_j) = col_max.get(*j as usize) else {
                continue;
            };
            if *col_max_j == I96F32::from_num(0.) {
                continue;
            }
            *value = value.saturating_div(*col_max_j);
        }
    }
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
mod tests {
    use super::*;
    use substrate_fixed::types::{I96F32, U64F64};

    macro_rules! fixed_vec {
        () => {
            vec![]
        };
        ($($x:expr),+ $(,)?) => {
            vec![$(I96F32::from_num($x)),+]
        };
    }

    /// Reshape vector to sparse matrix with specified number of input rows, cast f32 to I96F32.
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

    #[test]
    fn test_math_inplace_row_normalize_sparse() {
        let epsilon: I96F32 = I96F32::from_num(0.0001);
        let vector: Vec<f32> = vec![
            0., 1., 0., 2., 0., 3., 4., 0., 1., 0., 2., 0., 3., 0., 1., 0., 0., 2., 0., 3., 4., 0.,
            10., 0., 100., 1000., 0., 10000., 0., 0., 0., 0., 0., 0., 0., 1., 1., 1., 1., 1., 1.,
            1.,
        ];
        let mut mat = vec_to_sparse_mat_fixed(&vector, 6, false);
        inplace_row_normalize_sparse(&mut mat);
        let target: Vec<f32> = vec![
            0., 0.1, 0., 0.2, 0., 0.3, 0.4, 0., 0.166666, 0., 0.333333, 0., 0.5, 0., 0.1, 0., 0.,
            0.2, 0., 0.3, 0.4, 0., 0.0009, 0., 0.009, 0.09, 0., 0.9, 0., 0., 0., 0., 0., 0., 0.,
            0.142857, 0.142857, 0.142857, 0.142857, 0.142857, 0.142857, 0.142857,
        ];
        assert_sparse_mat_compare(&mat, &vec_to_sparse_mat_fixed(&target, 6, false), epsilon);
        let vector: Vec<f32> = vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let target: Vec<f32> = vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let mut mat = vec_to_sparse_mat_fixed(&vector, 3, false);
        inplace_row_normalize_sparse(&mut mat);
        assert_sparse_mat_compare(
            &mat,
            &vec_to_sparse_mat_fixed(&target, 3, false),
            I96F32::from_num(0),
        );
    }

    #[test]
    fn test_math_mask_diag_sparse() {
        let vector: Vec<f32> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
        let target: Vec<f32> = vec![0., 2., 3., 4., 0., 6., 7., 8., 0.];
        let mat = vec_to_sparse_mat_fixed(&vector, 3, false);
        let result = mask_diag_sparse(&mat);
        assert_sparse_mat_compare(
            &result,
            &vec_to_sparse_mat_fixed(&target, 3, false),
            I96F32::from_num(0),
        );
        let vector: Vec<f32> = vec![1., 0., 0., 0., 5., 0., 0., 0., 9.];
        let target: Vec<f32> = vec![0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let mat = vec_to_sparse_mat_fixed(&vector, 3, false);
        let result = mask_diag_sparse(&mat);
        assert_sparse_mat_compare(
            &result,
            &vec_to_sparse_mat_fixed(&target, 3, false),
            I96F32::from_num(0),
        );
        let vector: Vec<f32> = vec![0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let target: Vec<f32> = vec![0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let mat = vec_to_sparse_mat_fixed(&vector, 3, false);
        let result = mask_diag_sparse(&mat);
        assert_sparse_mat_compare(
            &result,
            &vec_to_sparse_mat_fixed(&target, 3, false),
            I96F32::from_num(0),
        );
    }
}
