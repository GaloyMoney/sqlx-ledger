use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use std::collections::HashMap;

use crate::{entry::*, error::*, primitives::*, transaction::NewTransaction};
use cel_interpreter::{CelContext, CelExpression};

use super::{param_definition::ParamDefinition, tx_params::TxParams};

#[derive(Deserialize)]
pub(crate) struct TxInputCel {
    effective: CelExpression,
    journal_id: CelExpression,
    correlation_id: Option<CelExpression>,
    external_id: Option<CelExpression>,
    description: Option<CelExpression>,
    metadata: Option<CelExpression>,
}

#[derive(Deserialize)]
pub(crate) struct EntryCel {
    entry_type: CelExpression,
    account_id: CelExpression,
    layer: CelExpression,
    direction: CelExpression,
    units: CelExpression,
    currency: CelExpression,
    description: Option<CelExpression>,
}

pub(crate) struct TxTemplateCore {
    pub(super) id: TxTemplateId,
    pub(super) _code: String,
    pub(super) params: Option<Vec<ParamDefinition>>,
    pub(super) tx_input: TxInputCel,
    pub(super) entries: Vec<EntryCel>,
}

impl TxTemplateCore {
    pub(crate) fn prep_tx(
        mut self,
        params: TxParams,
    ) -> Result<(NewTransaction, Vec<NewEntry>), SqlxLedgerError> {
        let mut tx_builder = NewTransaction::builder();
        tx_builder.tx_template_id(self.id);

        let ctx = params.to_context(self.params.take())?;

        let journal_id: Uuid = self.tx_input.journal_id.try_evaluate(&ctx)?;
        tx_builder.journal_id(journal_id.into());

        let effective: NaiveDate = self.tx_input.effective.try_evaluate(&ctx)?;
        tx_builder.effective(effective);

        if let Some(correlation_id) = self.tx_input.correlation_id.as_ref() {
            let correlation_id: Uuid = correlation_id.try_evaluate(&ctx)?;
            tx_builder.correlation_id(correlation_id.into());
        }

        if let Some(external_id) = self.tx_input.external_id.as_ref() {
            let external_id: Uuid = external_id.try_evaluate(&ctx)?;
            tx_builder.external_id(external_id.into());
        }

        if let Some(description) = self.tx_input.description.as_ref() {
            let description: String = description.try_evaluate(&ctx)?;
            tx_builder.description(description);
        }

        if let Some(metadata) = self.tx_input.metadata.as_ref() {
            let metadata: serde_json::Value = metadata.try_evaluate(&ctx)?;
            tx_builder.metadata(metadata);
        }

        let tx = tx_builder.build().expect("tx_build should succeed");
        let entries = self.prep_entries(ctx)?;

        Ok((tx, entries))
    }

    fn prep_entries(mut self, ctx: CelContext) -> Result<Vec<NewEntry>, SqlxLedgerError> {
        let mut new_entries = Vec::new();
        let mut totals = HashMap::new();
        for entry in self.entries.drain(..) {
            let mut builder = NewEntry::builder();
            let account_id: Uuid = entry.account_id.try_evaluate(&ctx)?;
            builder.account_id(account_id.into());

            let entry_type: String = entry.entry_type.try_evaluate(&ctx)?;
            builder.entry_type(entry_type);

            let layer: Layer = entry.layer.try_evaluate(&ctx)?;
            builder.layer(layer);

            let units: Decimal = entry.units.try_evaluate(&ctx)?;
            let currency: Currency = entry.currency.try_evaluate(&ctx)?;
            let direction: DebitOrCredit = entry.direction.try_evaluate(&ctx)?;

            let total = totals.entry(currency).or_insert(Decimal::ZERO);
            match direction {
                DebitOrCredit::Debit => *total -= units,
                DebitOrCredit::Credit => *total += units,
            };
            builder.units(units);
            builder.currency(currency);
            builder.direction(direction);

            if let Some(description) = entry.description.as_ref() {
                let description: String = description.try_evaluate(&ctx)?;
                builder.description(description);
            }

            new_entries.push(builder.build().expect("Couldn't build entry"));
        }

        for (k, v) in totals {
            if v != Decimal::ZERO {
                return Err(SqlxLedgerError::UnbalancedTransaction(k, v));
            }
        }

        Ok(new_entries)
    }
}
