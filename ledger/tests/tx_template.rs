use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{tx_template::*, *};

#[tokio::test]
async fn test_tx_template() -> anyhow::Result<()> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());

    let pg_con = format!("postgres://ledger:ledger@{pg_host}:5432/ledger");
    let pool = sqlx::PgPool::connect(&pg_con).await?;

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let params = vec![ParamDefinition::builder()
        .name("input1")
        .r#type(ParamDataType::STRING)
        .default_expr("'input'")
        .build()
        .unwrap()];
    let new_template = NewTxTemplate::builder()
        .code(code)
        .params(params)
        .tx_input(
            TxInput::builder()
                .effective("1")
                .journal_id("1")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    SqlxLedger::new(&pool)
        .tx_templates()
        .create(new_template)
        .await
        .unwrap();

    Ok(())
}
