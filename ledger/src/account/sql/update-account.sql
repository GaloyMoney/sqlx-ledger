INSERT INTO sqlx_ledger_accounts
  (id, version, code, name, normal_balance_type, description, status, metadata, created_at)
(
 SELECT id, version + 1, code, name, normal_balance_type, COALESCE($2, description), status, COALESCE($3, metadata), created_at
 FROM sqlx_ledger_accounts WHERE id = $1 ORDER BY version DESC LIMIT 1
)
