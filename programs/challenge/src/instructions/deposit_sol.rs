use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [signer.key().as_ref()],
            payer = signer,
            space = 8+Deposit::INIT_SPACE,
            bump)]
    pub deposit: Account<'info, Deposit>,

    pub system_program: Program<'info, System>,
    #[account(
        mut,
        constraint = program_state.active == true // @ DropError::ProgramDisabled
)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds = [
            &program_state.key().to_bytes(),
            SOL_LEG_SEED
        ],
        bump
    )]
    pub bank: SystemAccount<'info>,
}
pub fn process(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
    msg!("deposit_sol {}", amount);
    msg!("deposit_sol {}", ctx.accounts.signer.key());
    let program_state = &mut ctx.accounts.program_state;

    program_state.total_deposits += amount;
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.bank.to_account_info(),
            },
        ),
        amount,
    )?;
    let deposit = &mut ctx.accounts.deposit;
    deposit.authority = ctx.accounts.signer.key();
    deposit.amount += amount;

    Ok(())
}
