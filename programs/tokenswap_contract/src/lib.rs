use anchor_lang::prelude::*;

declare_id!("HKzdPahMuyfoLogNQMUfpgQxk5m4BGfimZbZmixJwdzf");

#[program]
pub mod tokenswap_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
