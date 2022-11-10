CREATE TYPE DebitOrCredit AS ENUM ('debit', 'credit');
CREATE TYPE AccountStatus AS ENUM ('active');

CREATE TABLE accounts_current (
  id UUID PRIMARY KEY,
  version INT NOT NULL,
  code VARCHAR(80) UNIQUE NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR NOT NULL,
  status AccountStatus NOT NULL,
  normal_balance_type DebitOrCredit NOT NULL,
  metadata JSONB,
  modified_at TIMESTAMP NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE accounts_history (
  id UUID,
  version INT NOT NULL,
  code VARCHAR(80) UNIQUE NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR NOT NULL,
  status AccountStatus NOT NULL,
  normal_balance_type DebitOrCredit NOT NULL,
  metadata JSONB,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE(id, version)
);
