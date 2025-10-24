use anchor_lang::prelude::*;

use crate::state::{HouseConfig, House};


pub fn update_house(ctx: Context<UpdateHouse>, house_config: HouseConfig, uri: Option<String>) -> Result<()> {
    ctx.accounts.house.config = house_config;
    ctx.accounts.house.uri = uri;
    Ok(())
}


#[derive(Accounts)]
pub struct UpdateHouse<'info> {
    #[account(mut, has_one=house_admin)]
    pub house: Box<Account<'info, House>>,
    pub house_admin: Signer<'info>,
}


