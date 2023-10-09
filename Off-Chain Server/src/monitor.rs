// monitor.rs

use solana_client::{rpc_client::RpcClient, thin_client::ThinClient};
use solana_program::pubkey::Pubkey;

fn get_stream_payment_data(stream_pubkey: Pubkey) -> Result<PaymentStream, ProgramError> {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let account_data = rpc_client.get_account_data(&stream_pubkey)?;
    PaymentStream::unpack(&account_data)
}
