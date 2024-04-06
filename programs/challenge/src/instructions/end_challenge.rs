use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

#[derive(Accounts)]
pub struct EndChallenge<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        constraint = program_state.active == true // @ DropError::ProgramDisabled
)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        //init,
        mut,
        seeds =[&challenge.key().to_bytes(),b"vault"],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub challenge: Box<Account<'info, ChallengeState>>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub winner: UncheckedAccount<'info>, //#[account(mut)]
                                         //pub vault: SystemAccount<'info>,
}
pub fn process(ctx: Context<EndChallenge>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    let challenge = &mut ctx.accounts.challenge;
    let ckey = challenge.key();
    let seeds = &[ckey.as_ref(), "vault".as_bytes(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.winner.to_account_info(),
        },
        signer,
    );
    let res = transfer(cpi_ctx, challenge.amount * 2);
    if res.is_ok() {
        msg!("transfer ok")
    }

    Ok(())
}
