# icp_token_wallet
## Setup

1. Install Rust: https://www.rust-lang.org/
2. Install DFINITY SDK: `sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"`
3. Clone this repository and navigate to the project directory.

## Deployment

1. Start the local ICP network: `dfx start --background`
2. Deploy the project: `dfx deploy`

## Testing

1. Run the unit tests: `cargo test`
