use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked}};

use crate::state::{EscrowState, ErrorCode};


#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub maker: UncheckedAccount<'info>,

    #[account(mut, 
        associated_token::mint=mint_b, 
        associated_token::authority=taker
    )]
    pub taker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut, 
        seeds=[b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump=escrow.bump)]
    pub escrow: Account<'info, EscrowState>,

    #[account(mut,
        associated_token::mint=mint_a, 
        associated_token::authority=escrow, 
        constraint = vault.owner == escrow.key() @ ErrorCode::InvalidVault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(init, 
        associated_token::mint=mint_a, 
        associated_token::authority=taker,
        payer=taker,
    )]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(init, 
        associated_token::mint=mint_b, 
        associated_token::authority=maker,
        payer=taker
    )]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Take<'info> {
    pub fn transfer(&mut self, amount: u64)-> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.taker_mint_b_ata.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint_b.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self, seeds: u64)-> Result<()> {
        let amount = self.vault.amount;
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let maker_key = self.maker.key();
        let seeds_bytes = seeds.to_le_bytes();

        let signer_seeds = &[b"escrow".as_ref(), maker_key.as_ref(), seeds_bytes.as_ref(), &[self.escrow.bump]];

        let signer = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

        Ok(())
    }
}