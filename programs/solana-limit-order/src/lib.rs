use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::solana_program::system_program;

declare_id!("7GwRvvPGFVRwMRtQgHnTDKvzLS1oPDNCMUzH1b5qCm1N");

#[program]
pub mod solana_limit_order {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn create_order(
        ctx: Context<CreateOrderContext>,
        maker_asset_address: String,
        taker_asset_address: String,
        maker_address: Pubkey,
        maker_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {

        let order = &mut ctx.accounts.order;

        order.salt = state.salt_index;
        order.maker_asset = maker_asset_address;
        order.taker_asset = taker_asset_address;
        order.maker = maker_address;
        order.making_amount = maker_amount;
        order.taking_amount = taker_amount;


        ctx.accounts.order.maker_amount = maker_amount;
     
        let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.order.maker_amount,
        )?;

        Ok(())
    }
    
    pub fn delete_order(_ctx: Context<DeleteOrderContext>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateStateContext<'info> {
    #[account(
        init,
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct CreateOrderContext<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub maker: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref()],
        bump,
        payer = maker,
        token::mint = mint,
        token::authority = maker,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = maker_make_token_account.amount >= maker_amount
    )]
    pub maker_make_token_account: Account<'info, TokenAccount>,
    pub maker_taker_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, OrderAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,

#[derive(Accounts)]
pub struct DeleteOrderContext<'info> {
    #[account(mut, has_one = maker, close = maker)]
    pub order: Account<'info, OrderAccount>,
    pub maker: Signer<'info>,
}

#[account]
pub struct StateAccount {
    pub salt_index: u64,
    pub authority: Pubkey,
}

#[account]
pub struct OrderAccount {
    
    pub maker_asset: String,
    
    pub taker_asset: String,
    
    pub maker: Pubkey,
    
    // pub receiver: Pubkey,
    
    // pub allowed_sender: Pubkey,
    
    pub making_amount: u64,
    
    pub taking_amount: u64,
    
    // pub maker_asset_data: String,
    
    // pub taker_asset_data: String,
    
    // pub get_maker_amount: String,
    
    // pub get_taker_amount: String,
    
}
