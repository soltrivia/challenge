use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::*;

#[derive(Accounts)]
pub struct InitProgram<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
            init,
            payer = signer,
            space = 8 + ProgramState::INIT_SPACE,
            seeds = [PROGRAM_STATE_SEED.as_ref()],
            bump
        )]
    pub program_state: Account<'info, ProgramState>,
}
pub fn process(ctx: Context<InitProgram>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    program_state.init(ctx.bumps.program_state, ctx.accounts.signer.key());

    Ok(())
}
