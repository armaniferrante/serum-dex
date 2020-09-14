use solana_client_gen::solana_client_gen;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

#[solana_client_gen]
pub mod instruction {
    use super::*;

    pub enum SrmSafeInstruction {
        /// DepositSrm initializes the deposit, transferring tokens from the controlling SPL token
        /// account to one owned by the SrmSafe program.
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   the payer SRM SPL token account, transferring ownership *from*.
        ///                 The owner of this account is expected to be Alameda.
        /// 1. `[writable]` the SrmSafe SPL account vault, transferring ownership *to*.
        ///                 The owner of this account is the SrmSafe program.
        /// 2. `[writable]  the vesting account representing the user's deposit. It is
        ///                 initialized with the data provided by the instruction.
        ///                 The owner of this account is the SrmSafe program.
        ///
        /// Access control assertions:
        ///
        ///  * Accounts[1].owner == SrmSafe.program_id
        ///  * Accounts[1].owner == SrmSafe.program_id
        ///
        DepositSrm {
            // Slot number, after which the SRM can be withdrawn.
            slot_number: u64,
            // SPL wallet owner, i.e., the account owner of the account
            // that will be withdrawn to in the future.
            user_spl_wallet_owner: Pubkey,
            // Amount of SRM to deposit.
            amount: u64,
            // Amount of locked SRM minted. Initialized to 0 on deposit.
            lsrm_amount: u64,
        },

        /// WithdrawSrm withdraws the given amount from the SrmSafe SPL account vault,
        /// updating the user's vesting account.
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   the vesting account's `user_spl_wallet_owner`. I.e., the
        ///                 owner of the spl wallet assigned to the vesting account.
        /// 1. `[writable]` the vesting account to withdraw from.
        /// 2. `[writable]` the SRM SPL token account to withdraw to.
        /// 3. `[writable]` the SrmSafe SPL account vault from which we are transferring
        ///                 ownership of the SRM out of.
        ///
        /// Access control assertions:
        ///
        ///  * VestingAccount.owner == SrmSafe.program_id
        ///  * VestingAccountInner.user_spl_wallet_owner == Accounts[0]
        ///  * Solana::current_slot() >= VestingAccountInner.slot_number
        ///
        WithdrawSrm {
            // Amount of SRM to withdraw.
            amount: u64,
        },

        /// MintLockedSrm mints an lSRM token and sends it to the depositor's lSRM SPL account,
        /// adjusting the vesting account's metadata as needed--increasing the amount of
        /// lSRM minted so that subsequent withdrawals will be affected by any outstanding
        /// locked srm associated with a vesting account.
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   the vesting account's `user_spl_wallet_owner`. I.e., the
        ///                 owner of the spl wallet assigned to the vesting account.
        /// 1. `[writable]` the lSRM SPL token account to send the newly minted lSRM to.
        /// 2. `[writable]` the vesting account.
        ///
        /// Access control assertions:
        ///
        ///  * VestingAccount.owner == SrmSafe.program_id
        ///  * VestingAccountInner.user_spl_wallet_owner == Accounts[0]
        ///  * VestingAccountInner.amount - VestingAccountInner.lsrm_amount >= amount
        ///
        MintLockedSrm {
            // Amount of lSRM to mint.
            amount: u64,
        },

        /// BurnLockedSrm destroys the lSRM associated with the vesting account, updating
        /// the vesting account's metadata so that subsequent withdrawals are not affected
        /// by the burned lSRM.
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   the owner of the lSRM SPL token account to burn from.
        /// 1. `[writable]` the lSRM SPL token account to burn from.
        /// 2. `[writable]` the vesting account.
        ///
        /// Access control assertions:
        ///
        ///  * VestingAccount.owner == SrmSafe.program_id
        ///  * VestingAccountInner.user_spl_wallet_owner == Accounts[0]
        ///  * VestingAccountInner.lsrm_amount >= amount
        ///
        /// Note that the signer, i.e., the owner of the lSRM SPL token account must be
        /// equal to the vesting' account's spl wallet owner, i.e. `user_spl_wallet_owner`.
        /// This means the same address must be the owner of *both* the lSRM account and
        /// the final SRM wallet account to withdraw from.
        ///
        BurnLockedSrm {
            // Amount of lSRM to burn.
            amount: u64,
        },
    }
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
