mod helpers;

use rand::distributions::{Alphanumeric, DistString};
use sqlx_ledger::{account::*, event::*, journal::*, tx_template::*, *};

#[tokio::test]
async fn post_transaction() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;

    let tx_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let new_journal = NewJournal::builder().name(name).build().unwrap();
    let ledger = SqlxLedger::new(&pool);

    let journal_id = ledger.journals().create(new_journal).await.unwrap();
    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let new_account = NewAccount::builder()
        .id(uuid::Uuid::new_v4())
        .name(format!("Test Sender Account {code}"))
        .code(code)
        .build()
        .unwrap();
    let sender_account_id = ledger.accounts().create(new_account).await.unwrap();
    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let new_account = NewAccount::builder()
        .id(uuid::Uuid::new_v4())
        .name(format!("Test Recipient Account {code}"))
        .code(code)
        .build()
        .unwrap();
    let recipient_account_id = ledger.accounts().create(new_account).await.unwrap();

    let params = vec![
        ParamDefinition::builder()
            .name("recipient")
            .r#type(ParamDataType::UUID)
            .build()
            .unwrap(),
        ParamDefinition::builder()
            .name("sender")
            .r#type(ParamDataType::UUID)
            .build()
            .unwrap(),
        ParamDefinition::builder()
            .name("journal_id")
            .r#type(ParamDataType::UUID)
            .build()
            .unwrap(),
        ParamDefinition::builder()
            .name("external_id")
            .r#type(ParamDataType::STRING)
            .build()
            .unwrap(),
        ParamDefinition::builder()
            .name("effective")
            .r#type(ParamDataType::DATE)
            .default_expr("date()")
            .build()
            .unwrap(),
    ];
    let entries = vec![
        EntryInput::builder()
            .entry_type("'TEST_DR'")
            .account_id("params.sender")
            .layer("SETTLED")
            .direction("DEBIT")
            .units("1290")
            .currency("'BTC'")
            .build()
            .unwrap(),
        EntryInput::builder()
            .entry_type("'TEST_CR'")
            .account_id("params.recipient")
            .layer("SETTLED")
            .direction("CREDIT")
            .units("1290")
            .currency("'BTC'")
            .build()
            .unwrap(),
    ];
    let new_template = NewTxTemplate::builder()
        .id(uuid::Uuid::new_v4())
        .code(&tx_code)
        .params(params)
        .tx_input(
            TxInput::builder()
                .effective("params.effective")
                .journal_id("params.journal_id")
                .external_id("params.external_id")
                .metadata(r#"{"foo": "bar"}"#)
                .build()
                .unwrap(),
        )
        .entries(entries)
        .build()
        .unwrap();
    ledger.tx_templates().create(new_template).await.unwrap();

    let events = ledger.events(Default::default()).await?;
    let mut sender_account_balance_events = events
        .account_balance(journal_id, sender_account_id)
        .await
        .expect("event subscriber closed");
    let mut all_events = events.all().expect("event subscriber closed");
    let mut journal_events = events
        .journal(journal_id)
        .await
        .expect("event subscriber closed");

    let external_id = uuid::Uuid::new_v4().to_string();
    let mut params = TxParams::new();
    params.insert("journal_id", journal_id);
    params.insert("sender", sender_account_id);
    params.insert("recipient", recipient_account_id);
    params.insert("external_id", external_id.clone());

    ledger
        .post_transaction(TransactionId::new(), &tx_code, Some(params))
        .await
        .unwrap();
    let transactions = ledger
        .transactions()
        .list_by_external_ids(vec![external_id.clone()])
        .await?;
    assert_eq!(transactions.len(), 1);

    let entries = ledger
        .entries()
        .list_by_transaction_ids(vec![transactions[0].id])
        .await?;

    assert!(entries.get(&transactions[0].id).is_some());
    assert_eq!(entries.get(&transactions[0].id).unwrap().len(), 2);

    assert_eq!(
        sender_account_balance_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );
    let next_event = all_events.recv().await.unwrap();
    assert_eq!(next_event.r#type, SqlxLedgerEventType::TransactionCreated);
    assert_eq!(
        all_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );
    let after_events = ledger
        .events(EventSubscriberOpts {
            after_id: Some(next_event.id),
            ..Default::default()
        })
        .await?;
    assert_eq!(
        after_events.all().unwrap().recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );
    assert_eq!(
        all_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );
    assert_eq!(
        journal_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::TransactionCreated
    );
    assert_eq!(
        journal_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );
    assert_eq!(
        journal_events.recv().await.unwrap().r#type,
        SqlxLedgerEventType::BalanceUpdated
    );

    Ok(())
}
