# StreamPay Solana Program

StreamPay Solana program designed to manage payment streams on the Solana blockchain. This solution enables users to set up scheduled payments over a specified time frame, ensuring both flexibility and security in transactions.

## Description

StreamPay allows for the creation of streaming payment contracts where a specific amount is periodically transferred from a sender to a recipient until the end of the contract or until the funds are depleted.

**Features:**

- Initialize a payment stream with specified parameters such as recipient, start time, payment interval, and amount.
- Smart contract ensures only valid and approved transactions are processed.
- Utilizes Solana's efficiency and low transaction costs.

## Architecture

### Data Structures

#### PaymentStream

- **is_initialized**: A boolean indicating if the payment stream is initialized.
- **recipient**: The public key of the recipient of the payment stream.
- **start_time**: The time at which the first payment will be made.
- **interval**: The time interval between payments.
- **payment_amount**: The amount to be transferred in each payment.
- **remaining_balance**: The total amount left to be transferred.

### Instructions

1. **InitializeStream**: This instruction is used to create a new payment stream. Required parameters include the recipient's public key, start time, interval, and payment amount.

### Custom Errors

StreamPay uses custom errors for improved debugging and error handling, which include:

- **AccountNotRentExempt**: Thrown when an account is not rent-exempt.
- **AccountAlreadyInitialized**: Thrown when trying to initialize an already initialized account.
- **MintsDoNotMatch**: Thrown when the mints of the sender and receiver tokens do not match.
- **SameSenderAndRecipient**: Thrown when the sender and recipient are the same.

### Entry Points

The program contains a single entry point, `process_instruction`, which handles and routes incoming instructions appropriately.

## Installation Guide

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI (solana)](https://docs.solana.com/cli/install-solana-cli-tools)

### Steps

1. **Clone the Repository**

   ```bash
   git clone https://github.com/stream-protocol/streampay-solana-program
   ```

2. **Navigate to the Project Directory**

   ```bash
   cd streampay-solana-program
   ```

3. **Build the Program**

   ```bash
   cargo build-bpf
   ```

4. **Deploy to Solana Devnet**

   ```bash
   solana deploy target/deploy/streampay.so --url https://devnet.solana.com
   ```

5. **Set Default Cluster** (Optional)

   ```bash
   solana config set --url https://devnet.solana.com
   ```

6. **Fund Your Solana Wallet**
   Get free SOL from the [faucet](https://solana-web3.js.org/modules.html#airdrop) for devnet testing.

7. **Deploy to Other Clusters**
   Replace the URL with the appropriate cluster URL for testnet or mainnet.

8. **Interact with Your Program**
   Use the Solana CLI or integrate with a frontend application.

## StreamPay Program Address

**Account pubkey**: 6sxq893Fh1RJLJH8VypcvnFvPf6NJ72Sx4um1DfTPM3n

**Wallet pubkey**: 3g7d5spF5LNXHi8wNi8efH97njgcTfFrANNr1ChHH1Tr

## Contribution

Contributions are welcome! Please fork the repository and open a pull request with your changes or open an issue if you encounter any problems.

## License

This project is licensed under the [MIT License](LICENSE).
