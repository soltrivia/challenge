use anchor_lang::prelude::*;

declare_id!("VEvkwFjBdmAEhfvQdB8qxnwnmfz5ygUkjyZCZgiEhKc");
pub mod constants;
pub mod instructions;
//pub mod constants;
use constants::*;
use instructions::*;
pub mod errors;
pub mod states;
use states::*;
#[program]
pub mod challenge {
    use super::*;

    pub fn initialize(ctx: Context<InitProgram>) -> Result<()> {
        init_program::process(ctx)
    }
    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        deposit_sol::process(ctx, amount)
    }
    pub fn create_challenge(ctx: Context<CreateChallenge>, amount: u64) -> Result<()> {
        create_challenge::process(ctx, amount)
    }
    pub fn join_challenge(ctx: Context<JoinChallenge>) -> Result<()> {
        join_challenge::process(ctx)
    }
    pub fn end_challenge(ctx: Context<EndChallenge>) -> Result<()> {
        end_challenge::process(ctx)
    }
}
