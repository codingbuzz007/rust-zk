use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SVM {
    weights: Vec<f64>,
    bias: f64,
}

impl SVM {
    /// Create a new SVM.
    pub fn new(weights: Vec<f64>, bias: f64) -> Self {
        Self { weights, bias }
    }

    pub fn predict(&self, point: Vec<f64>) -> i32 {
        let mut sum = 0.0;
        for (i, w) in self.weights.iter().enumerate() {
            sum += w * point[i];
        }
        if sum + self.bias >= 0.0 { 1 } else { -1 }
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("SVM: Processing instruction");

    let data: (SVM, Vec<f64>) = BorshDeserialize::try_from_slice(instruction_data)?;

    let prediction = data.0.predict(data.1);

    msg!("SVM Prediction: {}", prediction);

    Ok(())
}
