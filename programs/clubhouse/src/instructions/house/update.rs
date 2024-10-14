use anchor_lang::prelude::*;

use crate::state::{common::HouseConfig, House};


pub fn update_house(ctx: Context<UpdateHouse>, house_config: HouseConfig) -> Result<()> {
    let house = &mut ctx.accounts.house;
    house.config = house_config;
    Ok(())
}


#[derive(Accounts)]
pub struct UpdateHouse<'info> {
    #[account(mut, has_one=house_admin)]
    pub house: Box<Account<'info, House>>,
    pub house_admin: Signer<'info>,
}


