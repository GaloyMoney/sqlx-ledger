use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{journal::*, tx_template::*, *};

#[tokio::test]
async fn post_transaction() -> anyhow::Result<()> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());

    let pg_con = format!("postgres://ledger:ledger@{pg_host}:5432/ledger");
    let pool = sqlx::PgPool::connect(&pg_con).await?;

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let new_journal = NewJournal::builder().name(name).build().unwrap();
    let ledger = SqlxLedger::new(&pool);

    let journal_id = ledger.journals().create(new_journal).await.unwrap();

    let params = vec![
        ParamDefinition::builder()
            .name("journal_id")
            .r#type(ParamDataType::UUID)
            .build()
            .unwrap(),
        ParamDefinition::builder()
            .name("effective")
            .r#type(ParamDataType::DATE)
            .default_expr("date()")
            .build()
            .unwrap(),
    ];
    let new_template = NewTxTemplate::builder()
        .code(&code)
        .params(params)
        .tx_input(
            TxInput::builder()
                .effective("params.effective")
                .journal_id("params.journal_id")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    ledger.tx_templates().create(new_template).await.unwrap();
    let mut params = TxParams::new();
    params.insert("journal_id", journal_id);
    ledger.post_transaction(code, Some(params)).await.unwrap();
    Ok(())
}
