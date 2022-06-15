#![cfg(feature = "test-bpf")]

use borsh::ser::BorshSerialize;
use solana_program::instruction::AccountMeta;
use solana_program::{system_program, sysvar};
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use solana_test_framework::*;

#[tokio::test]
async fn poc() {
    // @fixme modify
    let program_id = "";
    let program_name = "";

    let solana_program_id = system_program::ID;

    // generate some keys
    let payer = Keypair::new();

    // Deploy program
    // @fixme MODIFY PROCESS INSTRUCTION
    let mut program = ProgramTest::new(
        program_name,
        program_id,
        processor!(MODIFY PROCESS INSTRUCTION),
    );

    program.add_account(
        payer.pubkey(),
        Account {
            lamports: 1_000_000_000_000_000,
            ..Account::default()
        },
    );

    let mut program_context = program.start_with_context().await;
    let mut recent_blockhash = program_context.last_blockhash.clone();

    // Get PDA
    let (pda1, _) = Pubkey::find_program_address(&[b""], &program_id);

    // Define accounts
    let acc = vec![
        AccountMeta::new(program_id, false),
        AccountMeta::new_readonly(payer.pubkey(), false),
        AccountMeta::new_readonly(solana_program_id, false),
    ];

    // Create instruction
    // @fixme modify INSTRUCTION
    let mut ix = Instruction {
        program_id,
        data: INSTRUCTION {}.try_to_vec().unwrap(),
        accounts: acc,
    };

    // create transaction
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

    tx.partial_sign(&[&payer], recent_blockhash);
    // process transactions
    program_context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap();

    let mut recent_blockhash = program_context.last_blockhash.clone();

    // Get account
    let mut acc = banks_client.get_account(pda1).await.unwrap().unwrap();

    // Deserialize
    // @fixme modify type
    let mut pda_state: TYPE =
        TYPE::try_deserialize(&mut acc.data.as_ref()).unwrap();
    println!("{:?}", pda_state);
}
