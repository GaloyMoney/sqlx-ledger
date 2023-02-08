mod helpers;

use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{account::NewAccount, *};

#[tokio::test]
async fn test_account() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let new_account = NewAccount::builder()
        .id(uuid::Uuid::new_v4())
        .name(format!("Test Account {code}"))
        .code(code)
        .build()
        .unwrap();
    let ledger = SqlxLedger::new(&pool);
    let id = ledger.accounts().create(new_account).await.unwrap();
    ledger
        .accounts()
        .update::<()>(id, Some("new description".to_string()), None)
        .await
        .unwrap();

    Ok(())
}
