use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

#[ic_cdk::update]
async fn deposit_tokens(amount: u64, recepient : Principal) -> Result<BlockIndex, String> {
    let caller = ic_cdk::caller();

    let transfer_from_args = TransferFromArgs {
        from: Account::from(caller),
        memo: None,
        amount: Nat::from(amount),
        spender_subaccount: None,
        fee: None,
        to: Account::from(recepient),
        created_at_time: None,
    };

    let transfer_result = ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").expect("Could not decode the principal."),
        "icrc2_transfer_from",
        (transfer_from_args,),
    )
    .await;

    match transfer_result {
        Ok((Ok(index),)) => Ok(index),
        Ok((Err(e),)) => Err(format!("Ledger transfer error: {:?}", e)),
        Err(e) => Err(format!("Failed to call the ledger: {:?}", e)),
    }
}

#[ic_cdk::update]
async fn send_tokens(amount: u64, to: Principal) -> Result<BlockIndex, String> {
    let caller = ic_cdk::caller();

    let transfer_args = TransferArg {
        memo: None,
        amount: Nat::from(amount),
        from_subaccount: None,
        fee: None,
        to: Account::from(to),
        created_at_time: None,
    };

    let transfer_result = ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").expect("Could not decode the principal."),
        "icrc1_transfer",
        (transfer_args,),
    )
    .await;

    match transfer_result {
        Ok((Ok(index),)) => Ok(index),
        Ok((Err(e),)) => Err(format!("Ledger transfer error: {:?}", e)),
        Err(e) => Err(format!("Failed to call the ledger: {:?}", e)),
    }
}

#[ic_cdk::update]
async fn get_balance() -> Result<Nat, String> {
    let caller = ic_cdk::caller();

    let account = Account {
        owner: caller,
        subaccount: None,
    };

    let result: Result<(Nat,), _> = ic_cdk::call(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").expect("Could not decode the principal."),
        "icrc1_balance_of",
        (account,),
    )
    .await;

    match result {
        Ok((balance,)) => Ok(balance),
        Err(e) => Err(format!("Failed to call ledger: {:?}", e)),
    }
}

#[ic_cdk::update]
async fn example_usage(amount: u64, to: Principal) -> Result<BlockIndex, String> {
    deposit_tokens(amount,to).await?;
    send_tokens(amount, to).await
}

ic_cdk::export_candid!();


