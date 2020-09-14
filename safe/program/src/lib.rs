//! Program entrypoint

use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
#[cfg(feature = "program")]
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

// todo: remove this shouldn't be necessary
#[cfg(not(feature = "program"))]
macro_rules! info {
    ($($i:expr),*) => { { ($($i),*) } };
}

solana_sdk::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    info!("ARMANI TESTING****************************************");
    Ok(())
}
