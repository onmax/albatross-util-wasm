use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogType {
    PayFee,
    Transfer,
    HTLCCreate,
    HTLCTimeoutResolve,
    HTLCRegularTransfer,
    HTLCEarlyResolve,
    VestingCreate,
    CreateValidator,
    UpdateValidator,
    ValidatorFeeDeduction,
    DeactivateValidator,
    ReactivateValidator,
    UnparkValidator,
    CreateStaker,
    Stake,
    StakerFeeDeduction,
    UpdateStaker,
    RetireValidator,
    DeleteValidator,
    Unstake,
    PayoutReward,
    Park,
    Slash,
    RevertContract,
    FailedTransaction,
}
