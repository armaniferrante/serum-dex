use serum_common::pack::Pack;
use serum_safe::accounts::{LsrmReceipt, Vesting};
use serum_safe::error::{SafeError, SafeErrorCode};
use solana_sdk::account_info::{next_account_info, AccountInfo};
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;
use spl_token::pack::Pack as TokenPack;
use std::convert::Into;

pub fn handler<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> Result<(), SafeError> {
    info!("handler: burn");

    let acc_infos = &mut accounts.iter();

    let token_owner_acc_info = next_account_info(acc_infos)?;
    let token_acc_info = next_account_info(acc_infos)?;
    let mint_acc_info = next_account_info(acc_infos)?;
    let receipt_acc_info = next_account_info(acc_infos)?;
    let vesting_acc_info = next_account_info(acc_infos)?;
    let token_program_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        program_id,
        token_owner_acc_info,
        token_acc_info,
        mint_acc_info,
        receipt_acc_info,
        token_program_acc_info,
    })?;

    Vesting::unpack_mut(
        &mut vesting_acc_info.try_borrow_mut_data()?,
        &mut |vesting_acc: &mut Vesting| {
            LsrmReceipt::unpack_mut(
                &mut receipt_acc_info.try_borrow_mut_data()?,
                &mut |lsrm_receipt: &mut LsrmReceipt| {
                    state_transition(StateTransitionRequest {
                        vesting_acc,
                        lsrm_receipt,
                        token_owner_acc_info,
                        token_acc_info,
                        mint_acc_info,
                        token_program_acc_info,
                    })
                    .map_err(Into::into)
                },
            )
        },
    )?;

    Ok(())
}

fn access_control<'a>(req: AccessControlRequest<'a>) -> Result<(), SafeError> {
    info!("access-control: burn");

    let AccessControlRequest {
        program_id,
        token_owner_acc_info,
        token_acc_info,
        mint_acc_info,
        receipt_acc_info,
        token_program_acc_info,
    } = req;

    if !token_owner_acc_info.is_signer {
        return Err(SafeErrorCode::Unauthorized)?;
    }
    let account = spl_token::state::Account::unpack(&token_acc_info.try_borrow_data()?)?;
    if account.owner != *token_owner_acc_info.key {
        return Err(SafeErrorCode::InvalidAccountOwner)?;
    }
    if receipt_acc_info.owner != program_id {
        return Err(SafeErrorCode::InvalidReceipt)?;
    }
    let receipt = LsrmReceipt::unpack(&receipt_acc_info.try_borrow_data()?)?;
    if receipt.spl_acc != *token_acc_info.key {
        return Err(SafeErrorCode::UnauthorizedReceipt)?;
    }
    if !receipt.initialized {
        return Err(SafeErrorCode::InvalidReceipt)?;
    }
    if receipt.burned {
        return Err(SafeErrorCode::AlreadyBurned)?;
    }
    if receipt.mint != *mint_acc_info.key {
        return Err(SafeErrorCode::WrongCoinMint)?;
    }
    if *token_program_acc_info.key != spl_token::ID {
        return Err(SafeErrorCode::InvalidTokenProgram)?;
    }

    info!("access-control: success");

    Ok(())
}

fn state_transition(req: StateTransitionRequest) -> Result<(), SafeError> {
    info!("state-transition: burn");

    let StateTransitionRequest {
        token_owner_acc_info,
        token_acc_info,
        mint_acc_info,
        token_program_acc_info,
        vesting_acc,
        lsrm_receipt,
    } = req;

    // Burn the NFT.
    {
        info!("burning spl token");

        let burn_instr = spl_token::instruction::burn(
            token_program_acc_info.key,
            token_acc_info.key,
            mint_acc_info.key,
            token_owner_acc_info.key,
            &[],
            1,
        )?;

        solana_sdk::program::invoke_signed(
            &burn_instr,
            &[
                token_acc_info.clone(),
                mint_acc_info.clone(),
                token_owner_acc_info.clone(),
                token_program_acc_info.clone(),
            ],
            &[],
        )?;

        info!("burn succcess");
    }

    // Burn the receipt.
    {
        lsrm_receipt.burned = true;
    }

    // Update the vesting account.
    {
        vesting_acc.locked_outstanding -= 1;
    }

    info!("state-transition: success");

    Ok(())
}

struct AccessControlRequest<'a> {
    program_id: &'a Pubkey,
    token_owner_acc_info: &'a AccountInfo<'a>,
    token_acc_info: &'a AccountInfo<'a>,
    mint_acc_info: &'a AccountInfo<'a>,
    receipt_acc_info: &'a AccountInfo<'a>,
    token_program_acc_info: &'a AccountInfo<'a>,
}

struct StateTransitionRequest<'a, 'b> {
    vesting_acc: &'b mut Vesting,
    lsrm_receipt: &'b mut LsrmReceipt,
    token_owner_acc_info: &'a AccountInfo<'a>,
    token_acc_info: &'a AccountInfo<'a>,
    mint_acc_info: &'a AccountInfo<'a>,
    token_program_acc_info: &'a AccountInfo<'a>,
}
