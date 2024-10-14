use crate::*;
use anchor_lang::prelude::*;


pub fn add_program_admin(ctx: Context<AddProgramAdmin>) -> Result<()> {
    ctx.accounts.program_admin_proof.program_admin = ctx.accounts.program_admin.key();
    Ok(())
}