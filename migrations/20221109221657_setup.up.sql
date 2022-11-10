CREATE TYPE DebitOrCredit AS ENUM ('debit', 'credit');
CREATE TYPE Status AS ENUM ('active');

CREATE TABLE accounts_current (
  id UUID PRIMARY KEY,
  version INT NOT NULL,
  code VARCHAR(80) UNIQUE NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR,
  status Status NOT NULL,
  normal_balance_type DebitOrCredit NOT NULL,
  metadata JSONB,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE accounts_history (
  id UUID,
  version INT NOT NULL,
  code VARCHAR(80) NOT NULL,
  name VARCHAR(80) NOT NULL,
  description VARCHAR,
  status Status NOT NULL,
  normal_balance_type DebitOrCredit NOT NULL,
  metadata JSONB,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version)
);
