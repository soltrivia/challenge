use crate::constants::*;
use crate::errors::*;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;
#[derive(Accounts)]
pub struct JoinChallenge<'info> {
    #[account(mut)]
    pub creater: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        constraint = program_state.active == true // @ DropError::ProgramDisabled
)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut,
        has_one = creater
    )]
    pub challenge: Box<Account<'info, ChallengeState>>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(constraint = ower.key() != challenge.user.key() @DropError::SingleAccount)]
    pub ower: UncheckedAccount<'info>,
    #[account(mut)]
    //pub vault: SystemAccount<'info>,
    pub vault: SystemAccount<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut,
        seeds = [
            &program_state.key().to_bytes(),
            SOL_LEG_SEED
        ],
        bump
    )]
    pub bank: SystemAccount<'info>,
    #[account(
        constraint = ower.key() == deposit.authority.key() @DropError::OnlyUseOwnData,
    )]
    pub deposit: Account<'info, Deposit>,
}
pub fn process(ctx: Context<JoinChallenge>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    let pkey = program_state.key();
    let seeds = &[pkey.as_ref(), "liq_sol".as_bytes(), &[ctx.bumps.bank]];
    let signer = &[&seeds[..]];

    let challenge = &mut ctx.accounts.challenge;
    challenge.accpet_user = ctx.accounts.ower.key();
    challenge.is_accepted = true;
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.bank.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    let deposit = &mut ctx.accounts.deposit;
    deposit.amount = deposit.amount.checked_sub(challenge.amount).unwrap();
    let res = transfer(cpi_ctx, challenge.amount);
    if res.is_ok() {
        msg!("transfer ok")
    }

    Ok(())
}
