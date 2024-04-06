use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub active: bool,
    pub current_version: u64,
    pub creater: Pubkey,
    pub bump: u8,
    pub total_deposits: u64,
}
impl ProgramState {
    pub fn init(&mut self, bump: u8, creater: Pubkey) {
        self.active = true;
        self.current_version = 1;
        self.bump = bump;
        self.creater = creater;
    
    }
    pub fn increment_version(&mut self) {
        self.current_version += 1;
    }
}

#[account]
#[derive(InitSpace)]
pub struct ChallengeState {
    pub active: bool,
    pub user: Pubkey,
    pub accpet_user: Pubkey,
    pub is_accepted: bool,
    pub creater: Pubkey,
    pub winner: Pubkey,
    pub amount: u64,
}
#[account]
#[derive(InitSpace)]
pub struct Deposit {
    pub authority: Pubkey,
    pub amount: u64,
}
