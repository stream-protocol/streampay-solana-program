use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
    token,
};
use std::convert::TryFrom;

const PUBKEY_BYTES: usize = 32;
const U64_BYTES: usize = 8;
const INITIALIZE_STREAM_OPCODE: u8 = 0;

// ===============================
// PaymentStream (StreamPay) Struct & Impl
// ===============================

#[derive(Debug, Default, PartialEq)]
pub struct PaymentStream {
    pub is_initialized: bool,
    pub recipient: Pubkey,
    pub start_time: u64,
    pub interval: u64,
    pub payment_amount: u64,
    pub remaining_balance: u64,
}

impl Sealed for PaymentStream {}

impl IsInitialized for PaymentStream {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl PaymentStream {
    pub fn validate(&self) -> Result<(), PaymentStreamError> {
        match (self.interval, self.payment_amount) {
            (0, _) => Err(PaymentStreamError::InvalidInterval),
            (_, 0) => Err(PaymentStreamError::InvalidPaymentAmount),
            _ => Ok(()),
        }
    }
}

// ===============================
// PaymentStreamInstruction Enum & Impl
// ===============================

#[derive(Debug, Clone)]
pub enum PaymentStreamInstruction {
    InitializeStream {
        recipient: Pubkey,
        start_time: u64,
        interval: u64,
        payment_amount: u64,
    },
}

impl PaymentStreamInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input[0] != INITIALIZE_STREAM_OPCODE {
            return Err(ProgramError::InvalidInstructionData);
        }

        let recipient = Pubkey::new(&input[1..1 + PUBKEY_BYTES]);
        let start_time = u64::from_le_bytes(TryFrom::try_from(&input[1 + PUBKEY_BYTES..1 + PUBKEY_BYTES + U64_BYTES])?);
        let interval = u64::from_le_bytes(TryFrom::try_from(&input[1 + PUBKEY_BYTES + U64_BYTES..1 + 2 * PUBKEY_BYTES + U64_BYTES])?);
        let payment_amount = u64::from_le_bytes(TryFrom::try_from(&input[1 + 2 * PUBKEY_BYTES + U64_BYTES..1 + 3 * PUBKEY_BYTES + 2 * U64_BYTES])?);

        Ok(PaymentStreamInstruction::InitializeStream {
            recipient,
            start_time,
            interval,
            payment_amount,
        })
    }
}

// ===============================
// Errors
// ===============================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentStreamError {
    NotRentExempt,
    AlreadyInitialized,
    InvalidInterval,
    InvalidPaymentAmount,
}

impl From<PaymentStreamError> for ProgramError {
    fn from(e: PaymentStreamError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// ===============================
// Entrypoint & Core Logic
// ===============================

entrypoint!(process_instruction);

fn process_initialize_stream(instruction: PaymentStreamInstruction, accounts: &[AccountInfo]) -> ProgramResult {
    let payment_stream_acc = &accounts[1];
    let rent = Rent::from_account_info(next_account_info(accounts.iter())?)?;

    if !rent.is_exempt(payment_stream_acc.lamports(), payment_stream_acc.data_len()) {
        return Err(PaymentStreamError::NotRentExempt.into());
    }

    let mut payment_stream = PaymentStream::unpack(&payment_stream_acc.data.borrow())?;
    if payment_stream.is_initialized() {
        return Err(PaymentStreamError::AlreadyInitialized.into());
    }

    if let PaymentStreamInstruction::InitializeStream {
        recipient,
        start_time,
        interval,
        payment_amount,
    } = instruction {
        payment_stream.is_initialized = true;
        payment_stream.recipient = recipient;
        payment_stream.start_time = start_time;
        payment_stream.interval = interval;
        payment_stream.payment_amount = payment_amount;
        payment_stream.remaining_balance = payment_amount;

        payment_stream.validate()?;
    }

    PaymentStream::pack(payment_stream, &mut payment_stream_acc.data.borrow_mut())?;

    // (rest of the function)

    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("StreamPay Solana program processing instruction");

    if program_id != &accounts[0].owner {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = PaymentStreamInstruction::unpack(instruction_data)?;

    match instruction {
        PaymentStreamInstruction::InitializeStream { .. } => {
            process_initialize_stream(instruction, accounts)
        }
    }
}
