// +---------------------------------------------------------------------+
// |                                                                     |
// |                                                                     |
// |   ___  ____ _  _ ____ _    ____ ___  ____ ___     ___  _   _        |
// |   |  \ |___ |  | |___ |    |  | |__] |___ |  \    |__]  \_/     .   |
// |   |__/ |___  \/  |___ |___ |__| |    |___ |__/    |__]   |      .   |
// |                                                                     |
// |   _  _ _ ____  _ _ _                                                |
// |   |  | | |__/  | | |                                                |
// |    \/  | |  \ _| | |___                                             |
// |                                                                     |
// |                                                                     |
// +---------------------------------------------------------------------+

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("AFPwsL4KdPowEBSbREcED8qboPYega1WZQ5xCrV2eqZK");

#[program]
pub mod tokenswap_contract {
    use super::*;

    pub fn swap_tokens(ctx: Context<SwapTokens>, from_amount: u64) -> Result<()> {
        // Minimum miktar kontrolü
        const MINIMUM_SWAP_AMOUNT: u64 = 1_000_000; // Örnek: 1 token minimum
        if from_amount < MINIMUM_SWAP_AMOUNT {
            return Err(ErrorCode::AmountTooLow.into());
        }

        // Oran ve komisyon hesaplamaları burada yapılacak (örnek için sabit değerler kullanılmıştır)
        let rate = 1; // 1:1 sabit oran
        let fee = from_amount / 100; // %1 komisyon
        let net_amount = from_amount.checked_mul(rate).ok_or(ErrorCode::Overflow)?;
        let to_amount = net_amount.checked_sub(fee).ok_or(ErrorCode::Overflow)?;

        // SPL Token programına CPI çağrısı yaparak transfer işlemi
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, to_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>, // Sender token hesabı
    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // Reciever token hesabı
    /// CHECK: Bu kontrol güvenlik amacı ile yerleştirildi.
    #[account(signer)]
    pub authority: AccountInfo<'info>, // Otorite (audit için önemli)
    pub token_program: Program<'info, Token>, // Token programı
}

#[error_code]
pub enum ErrorCode {
    #[msg("Hesaplama sırasında taşma oluştu.")]
    Overflow,
    #[msg("Transfer miktarı çok düşük.")]
    AmountTooLow,
}
