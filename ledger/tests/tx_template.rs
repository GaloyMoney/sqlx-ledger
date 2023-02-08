mod helpers;

use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{tx_template::*, *};

#[tokio::test]
async fn test_tx_template() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let params = vec![ParamDefinition::builder()
        .name("input1")
        .r#type(ParamDataType::STRING)
        .default_expr("'input'")
        .build()
        .unwrap()];
    let tx_input = TxInput::builder()
        .effective("1")
        .journal_id("1")
        .build()
        .unwrap();
    let entries = vec![EntryInput::builder()
        .entry_type("'TEST_DR'")
        .account_id("param.recipient")
        .layer("'Settled'")
        .direction("'Settled'")
        .units("1290")
        .currency("'BTC'")
        .build()
        .unwrap()];
    let new_template = NewTxTemplate::builder()
        .id(uuid::Uuid::new_v4())
        .code(code)
        .params(params)
        .tx_input(tx_input)
        .entries(entries)
        .build()
        .unwrap();
    SqlxLedger::new(&pool)
        .tx_templates()
        .create(new_template)
        .await
        .unwrap();

    Ok(())
}
