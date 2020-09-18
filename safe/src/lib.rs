//! serum-safe defines the interface for the serum safe program.

use accounts::{SafeAccount, VestingAccount};
use serde::{Deserialize, Serialize};
use solana_client_gen::prelude::*;
use solana_client_gen::solana_sdk;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

pub mod accounts;
pub mod error;
pub mod pack;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub enum SrmSafeInstruction {
        /// Initialize instruction configures the safe with an admin that is
        /// responsible for slashing people who use their locked serum for
        /// invalid purposes.
        ///
        /// Similar to a mint, this must be included in the same instruction
        /// that creates the account to initialize. Otherwise someone
        /// can take control of the account by calling. Initialize on it.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` the SafeAccount to initialize.
        /// 1. `[]`         Rent sysvar
        ///
        /// Access control assertions:
        ///  * Accounts[0].owner == SrmSafe.program_id
        #[cfg_attr(feature = "client", create_account(SafeAccount::SIZE))]
        Initialize {
            /// The mint of the SPL token to store in the safe, i.e., the
            /// SRM mint.
            mint: Pubkey,
            /// The owner of the admin account to set into the SafeAccount.
            /// This account has the power to slash deposits.
            authority: Pubkey,
        },
        /// Slash punishes a vesting account who misbehaved, punititvely
        /// revoking funds.
        ///
        /// 0. `[signer]`   the authority of the SafeAccount.
        /// 1. `[writable]` the vesting account to slash.
        ///
        /// Access control assertions:
        ///   * Accounts[0]
        Slash {
            /// The amount of SRM to slash.
            amount: u64,
        },
        /// DepositSrm initializes the deposit, transferring tokens from the controlling SPL token
        /// account to one owned by the SrmSafe program.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]  the VestingAccount representing this deposit. It is
        ///                 initialized with the data provided by the instruction.
        ///                 The owner of this account is the SrmSafe program.
        ///                 Note that it's data size is dynamic.
        /// 1. `[writable]` the depositor SRM SPL token account, transferring ownership *from*,
        ///                 itself to this program.
        /// 2. `[signer]`   the authority/owner/delegate of Accounts[1].
        /// 3. `[writable]` the SrmSafe SPL SRM vault, transferring ownership *to*.
        ///                 The owner of this account is the SrmSafe program.
        /// 4. `[]`         the SafeAccount instance.
        /// 5. `[]`         SPL token program.
        /// 6. `[]`         the rent sysvar.
        ///
        /// Access control assertions:
        ///
        ///  * Accounts[0].owner == SrmSafe.program_id
        ///  * Accounts[1].owner == SrmSafe.program_id
        ///
        #[cfg_attr(feature = "client", create_account(..))]
        DepositSrm {
            /// The beneficiary of the vesting account, i.e.,
            /// the user who will own the SRM upon vesting.
            vesting_account_beneficiary: Pubkey,
            /// The Solana slot number at which point a vesting amount unlocks.
            vesting_slots: Vec<u64>,
            /// The amount of SRM to release for each vesting_slot.
            vesting_amounts: Vec<u64>,
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
