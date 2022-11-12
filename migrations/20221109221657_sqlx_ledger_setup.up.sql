CREATE TYPE DebitOrCredit AS ENUM ('debit', 'credit');
CREATE TYPE Status AS ENUM ('active');

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
  description VARCHAR,
  metadata JSONB,
  modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  UNIQUE(id, version),
  UNIQUE(code, version)
);
