use pallet_evm::{
    ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult,
};
use polkadot_sdk::{
    frame_system::RawOrigin,
    pallet_balances,
    sp_runtime::traits::{Dispatchable, UniqueSaturatedInto},
    sp_std,
};
use sp_std::vec;

use crate::{
    Runtime, RuntimeCall,
    precompiles::{bytes_to_account_id, get_method_id, get_slice},
};

pub const BALANCE_TRANSFER_INDEX: u64 = 2048;

pub struct BalanceTransferPrecompile;

impl BalanceTransferPrecompile {
    pub fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input = handle.input();

        // Check method signature
        let method = get_slice(input, 0, 4)?;
        if method != get_method_id("transfer(bytes32)") {
            return Ok(PrecompileOutput {
                exit_status: ExitSucceed::Returned,
                output: vec![],
            });
        }

        // Get the transfer amount from the transaction
        let amount = handle.context().apparent_value;
        if amount.is_zero() {
            return Ok(PrecompileOutput {
                exit_status: ExitSucceed::Returned,
                output: vec![],
            });
        }

        // Hardcoded precompile substrate address (equivalent to 0x800)
        const SOURCE_ADDRESS: [u8; 32] = [
            0x07, 0xec, 0x71, 0x2a, 0x5d, 0x38, 0x43, 0x4d, 0xdd, 0x03, 0x3f, 0x8f, 0x02, 0x4e,
            0xcd, 0xfc, 0x4b, 0xb5, 0x95, 0x1c, 0x13, 0xc3, 0x08, 0x5c, 0x39, 0x9c, 0x8a, 0x5f,
            0x62, 0x93, 0x70, 0x5d,
        ];

        // Get destination address from input
        let destination_bytes = get_slice(input, 4, 36)?;

        // Convert addresses to AccountId32
        let source = bytes_to_account_id(&SOURCE_ADDRESS)?;
        let destination = bytes_to_account_id(destination_bytes)?;

        // Create the transfer call
        let transfer_call =
            RuntimeCall::Balances(pallet_balances::Call::<Runtime>::transfer_allow_death {
                dest: destination.into(),
                value: amount.unique_saturated_into(),
            });

        transfer_call
            .dispatch(RawOrigin::Signed(source).into())
            .map_err(|_| PrecompileFailure::Error {
                exit_status: ExitError::OutOfFund,
            })?;

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: vec![],
        })
    }
}
