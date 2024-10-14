use anchor_lang::prelude::*;

use crate::ProgramAdminProof;


pub fn remove_program_admin(_: Context<RemoveProgramAdmin>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveProgramAdmin<'info> {
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
    
    #[account(mut, close=program_admin_proof, seeds=[b"program_admin", program_admin.key().as_ref()], bump)]
    pub program_admin_proof: Account<'info, ProgramAdminProof>,
    pub system_program: Program<'info, System>,
}



