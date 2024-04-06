use anchor_lang::accounts::unchecked_account;
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(mut)]
    pub creater: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        constraint = program_state.active == true, // @ DropError::ProgramDisabled
        has_one = creater
)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub ower: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creater,
        space = 8 + ChallengeState::INIT_SPACE,
        //bump,
    )]
    pub challenge: Box<Account<'info, ChallengeState>>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        //init,
        mut,
        seeds =[&challenge.key().to_bytes(),b"vault"],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
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
    pub deposit: Account<'info, Deposit>, //pub bank: SystemAccount<'info>,
}
pub fn process(ctx: Context<CreateChallenge>, amount: u64) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    let pkey = program_state.key();
    let seeds = &[pkey.as_ref(), "liq_sol".as_bytes(), &[ctx.bumps.bank]];
    let signer = &[&seeds[..]];
    let deposit = &mut ctx.accounts.deposit;
    deposit.amount = deposit.amount.checked_sub(amount).unwrap();
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.bank.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    let res = transfer(cpi_ctx, amount);
    if res.is_ok() {
        msg!("transfer ok")
    }
    ctx.accounts.challenge.set_inner(ChallengeState {
        user: ctx.accounts.ower.key(),
        amount: amount,
        is_accepted: false,
        active: false,
        winner: Pubkey::default(),
        accpet_user: Pubkey::default(),
        creater: ctx.accounts.creater.key(),
        //bump: ctx.bumps.user,
    });
    Ok(())
}
