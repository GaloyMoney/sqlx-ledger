use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{account::NewAccount, *};

#[tokio::test]
async fn test_account() -> anyhow::Result<()> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());

    let pg_con = format!("postgres://ledger:ledger@{pg_host}:5432/ledger");
    let pool = sqlx::PgPool::connect(&pg_con).await?;

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let new_account = NewAccount::builder()
        .name(format!("Test Account {code}"))
        .code(code)
        .build()
        .unwrap();
    SqlxLedger::new(&pool)
        .accounts()
        .create(new_account)
        .await
        .unwrap();

    Ok(())
}
