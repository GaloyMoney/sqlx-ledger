# Sqlx-Ledger

This crate provides primitives for double sided accounting built on top of the [sqlx](https://github.com/launchbadge/sqlx) Postgres integration.

It features:
* Accounts can have balances on multiple journals
* Multi currency / multi layer support
* Template based transaction inputs for consistency
* CEL based template interpolation
* Event streaming to be notified of changes

The CEL interpreter is not complete but provides enough to support basic use cases.
More will be added as the need arises.

To use it copy the [migrations](./migrations) into your project and add the crate via `cargo add sqlx-ledger`.

Check out the [docs](https://docs.rs/sqlx-ledger/latest/sqlx_ledger/) for an example of how to use it
