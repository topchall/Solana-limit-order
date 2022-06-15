use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::solana_program::system_program;

declare_id!("7GwRvvPGFVRwMRtQgHnTDKvzLS1oPDNCMUzH1b5qCm1N");

#[program]
pub mod solana_limit_order {
    use super::*;

    pub fn create_state(ctx: Context<CreateStateContext>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();

        state.salt_index = 1;

        Ok(())
    }

    pub fn create_order(
        ctx: Context<CreateOrderContext>,
        maker_asset_address: String,
        taker_asset_address: String,
        maker_address: Pubkey,
        maker_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        let order = &mut ctx.accounts.order;

        order.salt = state.salt_index;
        order.maker_asset = maker_asset_address;
        order.taker_asset = taker_asset_address;
        order.maker = maker_address;
        order.making_amount = maker_amount;
        order.taking_amount = taker_amount;

        state.salt_index += 1;

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
pub struct CreateOrderContext<'info> {
    #[account(
        mut,
        seeds = [b"state".as_ref()],
        bump
    )]
    pub state: Account<'info, StateAccount>,

    #[account(
        init,
        seeds = [b"order".as_ref(), state.salt_index.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<OrderAccount>() + 8
    )]
    pub order: Account<'info, OrderAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

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
    
    pub salt: u64,
    
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
