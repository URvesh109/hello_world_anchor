use anchor_lang::prelude::*;
use solana_program::msg;

declare_id!("6xBtgAHiZvQUNcApL36wj3a94soEocHc5QaowVR8Kyr");

#[program]
pub mod hello_world {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("Hello World Rust program");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
