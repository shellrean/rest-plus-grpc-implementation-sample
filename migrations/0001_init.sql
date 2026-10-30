CREATE TABLE IF NOT EXISTS bank_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_number VARCHAR(50) NOT NULL UNIQUE,
    owner_name VARCHAR(255) NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO bank_accounts (id, account_number, owner_name, balance, currency)
VALUES
    ('11111111-1111-1111-1111-111111111111', 'ACC-1001', 'John Doe', 155000, 'USD'),
    ('22222222-2222-2222-2222-222222222222', 'ACC-1002', 'Jane Smith', 427500, 'USD')
ON CONFLICT (account_number) DO NOTHING;
