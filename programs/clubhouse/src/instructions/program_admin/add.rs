use anchor_lang::prelude::*;

use crate::AddProgramAdmin;


pub fn add_program_admin(ctx: Context<AddProgramAdmin>) -> Result<()> {
    ctx.accounts.program_admin_proof.program_admin = ctx.accounts.program_admin.key();
    Ok(())
}