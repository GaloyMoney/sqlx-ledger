use sqlx_ledger::*;

#[tokio::test]
async fn test_account() -> anyhow::Result<()> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());

    let pg_con = format!("postgres://ledger:ledger@{pg_host}:5432/ledger");
    let pool = sqlx::PgPool::connect(&pg_con).await?;

    // SqlxLedger::new(&pool).accounts().create().await.unwrap();

    Ok(())
}
