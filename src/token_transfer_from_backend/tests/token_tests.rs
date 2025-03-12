use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::NumTokens;
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use token_transfer_from_backend::TransferArgs;
use tokio::test;

trait MockLedger {
    fn mock_transfer(&self, args: TransferFromArgs) -> Result<Nat, TransferFromError>;
}

struct MockLedgerImpl {
    response: Result<Nat, TransferFromError>,
}

impl MockLedger for MockLedgerImpl {
    fn mock_transfer(&self, _args: TransferFromArgs) -> Result<Nat, TransferFromError> {
        self.response.clone()
    }
}

#[test]
async fn test_transfer() {
    let recipient = Account {
        owner: Principal::from_text("2vxsx-fae").unwrap(),
        subaccount: None,
    };

    let transfer_args = TransferArgs {
        amount: NumTokens::from(1_000_000_000u128), 
        to_account: recipient,
    };

    let mock_ledger = MockLedgerImpl {
        response: Ok(Nat::from(12345u128)), 
    };

    let result = mock_ledger.mock_transfer(TransferFromArgs {
        from: Account::from(Principal::anonymous()),
        memo: None,
        amount: transfer_args.amount,
        spender_subaccount: None,
        fee: None,
        to: transfer_args.to_account,
        created_at_time: None,
    });

    assert!(result.is_ok(), "Transfer failed: {:?}", result);
    assert_eq!(result.unwrap(), Nat::from(12345u128), "Unexpected block index"); 
}
