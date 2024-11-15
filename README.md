ICP RUST WALLET
The wallet interacts with the Internet Computer's ICRC-1 and ICRC-2 token standards, allowing users to deposit, hold, and transfer tokens.

Key Features:
Deposit Tokens:

Users must first approve the wallet to transfer tokens on their behalf.
Once approved, users can deposit tokens by calling the deposit_tokens function and specifying the amount they want to deposit.
The deposited tokens are then held within the wallet contract, and the user's balance is updated accordingly.
Hold Tokens:

The wallet securely holds tokens in the user's account within the contract. The balances are mapped to each user's Principal ID.
Users can check their wallet balance at any time by calling the get_token_balance function.
Transfer Tokens:

Users can transfer tokens to another account by calling the send_tokens function.
To withdraw tokens back to their personal account, users simply call send_tokens, passing their own Principal ID and the amount they wish to withdraw.
The wallet verifies that the user has enough tokens before executing the transfer.
Prerequisites
Before getting started, ensure that the following are installed:

DFINITY SDK (DFX)
Rust programming language and necessary libraries for building Rust-based canisters.
Project Structure
This project includes two main functionalities: Ledger Interactions for deposits and transfers of tokens.

The core components are:

PrincipalKey: A structure that wraps around the Principal of the caller, which acts as the unique identifier for each user in the system.
Stable Memory Management: The balances are stored using a Stable BTree Map, backed by stable memory provided by the Internet Computer.
Steps for Deployment
Step 1: Start the Local Replica
Ensure the Internet Computer is running in the background:

dfx start --background --clean
Step 2: Create a New Identity for Minting
Create a new identity (minter) for minting tokens and set it as the active identity:

dfx identity new minter
dfx identity use minter
export MINTER=$(dfx identity get-principal)
Transfers from the minting account will create Mint transactions, and transfers to the minting account will create Burn transactions.

Step 3: Switch Back to the Default Identity
Switch back to your default identity and record its principal. This principal will be used to mint the initial balance during deployment:

dfx identity use default
export DEFAULT=$(dfx identity get-principal)
Step 4: Deploy the ICRC-1 Ledger
Now you are ready to deploy the ICRC-1 ledger. This step sets the minting account, mints 100 tokens to the default principal, and sets a transfer fee:

dfx deploy icrc1_ledger_canister --argument "(variant { Init =
record {
     token_symbol = \"ICRC1\";
     token_name = \"L-ICRC1\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 0;
     metadata = vec {};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; 10_000_000_000; }; };
     archive_options = record {
         num_blocks_to_archive = 1000;
         trigger_threshold = 2000;
         controller_id = principal \"${MINTER}\";
     };
	 feature_flags = opt record {
      icrc2 = true;
    };
 }
})"
Step 5: Verify Ledger Deployment
You can verify that the ledger is working by checking the token balance for the DEFAULT account:

dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"${DEFAULT}\";
})"
The balance should show the tokens minted in the previous step.

Step 6: Deploy the Token Wallet Canister
Deploy the token_wallet_backend canister that manages the token balances and interacts with the ledger:

dfx deploy token_wallet_backend
Step 7: Approve Token Wallet Canister to Spend Tokens
Before the backend canister can interact with the ledger on behalf of a user, the user needs to approve the backend canister to spend tokens on their behalf:

dfx canister call --identity default icrc1_ledger_canister icrc2_approve "(
  record {
    spender= record {
      owner = principal \"$(dfx canister id token_wallet_backend)\";
    };
    amount = 1_000_000: nat;
  }
)"
Step 8: Test Token Wallet Functions
You can interact with the token wallet backend by calling the following functions:

Check Token Balance:
This call will check the balance of the tokens in the wallet:

dfx canister call token_wallet_backend get_token_balance
Deposit Tokens:
You can deposit tokens into your wallet with the following command (e.g., 100 tokens):

dfx canister call token_wallet_backend deposit_tokens '(100)'
Send Tokens:
You can also send tokens from your wallet to another principal. You can modify the wallet functions to send tokens from one account to another by adjusting the transfer function logic.

dfx canister call token_wallet_backend send_tokens "(50, principal \"<recipient-principal-id>\")"
To check the balance of a specific principal, use the following command:

dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"<principal-id>\";
})"
Interacting with the Token Wallet
1. Get Token Balance
This query function checks the current token balance of the calling user.

dfx canister call token_wallet_backend get_token_balance
The balance is stored in the stable memory (BALANCE_MAP) and associated with the Principal of the user. If no balance exists for the caller, it returns 0.

2. Deposit Tokens
This function allows users to deposit tokens into the wallet backend from their own account. The deposited tokens are recorded in the stable memory.

dfx canister call token_wallet_backend deposit_tokens '(100)'
References
Creating a token
Dfinity Examples
