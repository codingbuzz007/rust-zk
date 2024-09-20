use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signer::keypair::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use borsh::BorshSerialize;
use svm::{SVM};

fn main() {

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let payer = Keypair::new();

    let svm = SVM::new(vec![0.5, -0.3], 0.1);  
    let input_data = vec![1.0, 2.0];  

    let instruction_data = (svm, input_data).try_to_vec().unwrap();

    let program_id = Pubkey::from_str("YOUR_PROGRAM_ID").unwrap();

    let instruction = Instruction::new_with_borsh(program_id, &instruction_data, vec![]);

    // sign the transaction.
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Send the transaction.
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}
