use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

pub enum SrmSafeInstruction {
    DepositSrm { amount: u64 },
}

pub fn deposit(program_id: Pubkey, amount: u64) -> Instruction {
    let accounts = vec![];
    let data = vec![];
    Instruction {
        program_id,
        accounts,
        data,
    }
}
