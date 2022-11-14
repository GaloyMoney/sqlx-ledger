CREATE TYPE DebitOrCredit AS ENUM ('debit', 'credit');
CREATE TYPE Status AS ENUM ('active');
CREATE TYPE Layer AS ENUM ('settled', 'pending', 'encumbered');

CREATE TABLE accounts (
  id UUID NOT NULL,
  version INT NOT NULL,
  code VARCHAR(80) NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR,
  status Status NOT NULL,
  normal_balance_type DebitOrCredit NOT NULL,
  metadata JSONB,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version),
  UNIQUE(code, version),
  UNIQUE(name, version)
);

CREATE TABLE journals (
  id UUID NOT NULL,
  version INT NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR,
  status Status NOT NULL,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version),
  UNIQUE(name, version)
);

CREATE TABLE tx_templates (
  id UUID NOT NULL,
  code VARCHAR(80) NOT NULL,
  version INT NOT NULL,
  params JSONB,
  tx_input JSONB NOT NULL,
  entries JSONB NOT NULL,
  description VARCHAR,
  metadata JSONB,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version),
  UNIQUE(code, version)
);

CREATE TABLE transactions (
  id UUID NOT NULL,
  version INT NOT NULL,
  journal_id UUID NOT NULL,
  tx_template_id UUID NOT NULL,
  correlation_id UUID NOT NULL,
  effective Date NOT NULL,
  external_id UUID NOT NULL,
  description VARCHAR,
  metadata JSONB,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version)
);

CREATE TABLE entries (
  id UUID NOT NULL,
  version INT NOT NULL,
  transaction_id UUID NOT NULL,
  account_id UUID NOT NULL,
  journal_id UUID NOT NULL,
  entry_type VARCHAR NOT NULL,
  layer Layer NOT NULL,
  units Numeric NOT NULL,
  currency VARCHAR NOT NULL,
  direction DebitOrCredit NOT NULL,
  sequence INT NOT NULL,
  description VARCHAR,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version)
);
