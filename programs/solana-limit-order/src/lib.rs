use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::solana_program::system_program;

declare_id!("7GwRvvPGFVRwMRtQgHnTDKvzLS1oPDNCMUzH1b5qCm1N");

#[program]
pub mod solana_limit_order {
    use super::*;

    pub fn create_order(
        ctx: Context<CreateOrderContext>,
        sell_coin: String,
        buy_coin: String,
        limit_price: f32,
        sell_amount: f32,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let trader = &ctx.accounts.trader;

        order.trader = *trader.key;
        order.sell_coin = sell_coin;
        order.buy_coin = buy_coin;
        order.limit_price = limit_price;
        order.sell_amount = sell_amount;

        Ok(())
    }
    
    pub fn delete_order(_ctx: Context<DeleteOrderContext>) -> Result {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrderContext<'info> {
    #[account(
        init,
        payer = trader,
        space = size_of::<OrderAccount>() + 8
    )]
    pub order: Account<'info, OrderAccount>,

    #[account(mut)]
    pub trader: Signer<'info>,
    /// CHECK:
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeleteOrderContext<'info> {
    #[account(mut, has_one = trader, close = trader)]
    pub order: Account<'info, OrderAccount>,
    pub trader: Signer<'info>,
}


#[account]
pub struct OrderAccount {
    pub trader: Pubkey,
    pub sell_coin: String,
    pub buy_coin: String,
    pub limit_price: f32,
    pub sell_amount: f32,
}
