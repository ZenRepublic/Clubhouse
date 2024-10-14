pub mod house;
pub mod program_admin;
pub mod campaign;
pub mod game;

use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Transfer};


pub fn execute_token_transfer<'a>(
    amount: u64,
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    signer_seeds: Option<&[&[&[u8]]]>
) -> Result<()>{
    let accounts = anchor_spl::token::Transfer {
        from,
        to,
        authority
    };
    let ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(token_program, accounts);
    token::transfer(match signer_seeds {
        Some(seeds) => ctx.with_signer(seeds),
        None => ctx,
    }, amount)
}

pub fn execute_lamport_transfer<'a>(
    amount: u64,
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    signer_seeds: Option<&[&[&[u8]]]>
) -> Result<()> {
    let accounts = system_program::Transfer{
        from,
        to
    };
    let ctx = CpiContext::new(system_program, accounts);
    system_program::transfer(match signer_seeds {
        Some(seeds) => ctx.with_signer(seeds),
        None => ctx,
    }, amount)
}

pub fn execute_token_close<'a>(
    account: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    signer_seeds: Option<&[&[&[u8]]]>
) -> Result<()> {
    let accounts = anchor_spl::token::CloseAccount {
        account,
        destination,
        authority
    };
    let ctx = CpiContext::new(token_program, accounts);
    token::close_account(match signer_seeds {
        Some(seeds) => ctx.with_signer(seeds),
        None => ctx,
    })
}

pub fn execute_token_burn<'a>(
    amount: u64,
    mint: AccountInfo<'a>,
    from: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    signer_seeds: Option<&[&[&[u8]]]>
) -> Result<()> {
    let accounts = anchor_spl::token::Burn {
        from,
        mint,
        authority
    };
    let ctx = CpiContext::new(token_program, accounts);
    token::burn(match signer_seeds {
        Some(seeds) => ctx.with_signer(seeds),
        None => ctx
    }, amount)
}
