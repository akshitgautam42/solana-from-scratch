use solana_program::{entrypoint, 
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id:&Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult{
   msg!(
    "process_instruction:{}: {} accounts,data={:?}",
    program_id,accounts.len(),instruction_data

   );

   Ok(())
}

