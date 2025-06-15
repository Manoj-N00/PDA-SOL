use solana_program::{account_info::{next_account_info ,AccountInfo},entrypoint::{self ,ProgramResult}, pubkey::Pubkey};

entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let pda = next_account_info(iter);
    let user_acc = next_account_info(iter);
    let sysyem_program= next_account_info(iter);
}