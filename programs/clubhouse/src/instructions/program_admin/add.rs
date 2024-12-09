use anchor_lang::prelude::*;

use crate::ProgramAdminProof;


pub fn add_program_admin(ctx: Context<AddProgramAdmin>) -> Result<()> {
    ctx.accounts.program_admin_proof.program_admin = ctx.accounts.program_admin.key();
    Ok(())
}


#[derive(Accounts)]
pub struct AddProgramAdmin<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    ///CHECK: not relevant what type of account the admin is
    pub program_admin: UncheckedAccount<'info>,
    #[account(seeds=[crate::id().as_ref()], 
        seeds::program=anchor_lang::solana_program::bpf_loader_upgradeable::id(), 
        bump, 
        constraint=program_data.upgrade_authority_address == Some(signer.key()) @ crate::errors::ErrorCodes::ProgramAuthorityMismatch
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(init, payer=signer, space=8+64, seeds=[b"program_admin", program_admin.key().as_ref()], bump)]
    pub program_admin_proof: Account<'info, ProgramAdminProof>,
    pub system_program: Program<'info, System>,
}