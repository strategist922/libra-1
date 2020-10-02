use crate::account::Account;
use libra_types::transaction::{SignedTransaction, TransactionArgument};
use stdlib::transaction_scripts::StdlibScript;
use crate::gas_costs;
use libra_types::account_config::LBR_NAME;
use move_core_types::account_address::AccountAddress;

/// This is test infrastructure. Helps build a signed transaction script of the MinerState module.

pub fn e2e_miner_state_fixtures(
    sender: &Account,
    seq_num: u64
) -> SignedTransaction {
    let args = vec![];
    sender.create_signed_txn_with_args(
        StdlibScript::MinerStateTestHelper
            .compiled_bytes()
            .into_vec(),
        vec![],
        args,
        seq_num,
        gas_costs::TXN_RESERVED,
        0,
        LBR_NAME.to_owned(),
    )
}

pub fn e2e_submit_proof_txn(
    sender: &Account,
    seq_num: u64,
    challenge: Vec<u8>,
    solution: Vec<u8>,
) -> SignedTransaction {
    let args = vec![
        TransactionArgument::U8Vector(challenge),
        TransactionArgument::U8Vector(solution),
    ];
    sender.create_signed_txn_with_args(
        StdlibScript::MinerState
            .compiled_bytes()
            .into_vec(),
        vec![],
        args,
        seq_num,
        gas_costs::TXN_RESERVED,
        0,
        LBR_NAME.to_owned(),
    )
}

pub fn e2e_onboarding_tx(sender: &Account, seq_num: u64, challenge: Vec<u8>, solution: Vec<u8>, expected_address: AccountAddress) -> SignedTransaction {
    let args = vec![
        TransactionArgument::U8Vector(challenge),
        TransactionArgument::U8Vector(solution),
        TransactionArgument::Address(expected_address),
    ];
    sender.create_signed_txn_with_args(
        StdlibScript::MinerStateOnboarding
            .compiled_bytes()
            .into_vec(),
        vec![],
        args,
        seq_num,
        gas_costs::TXN_RESERVED * 4,
        0,
        LBR_NAME.to_owned(),
    )
}