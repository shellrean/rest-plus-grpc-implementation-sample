SELECT id, account_number, owner_name, balance, currency, created_at, updated_at
FROM bank_accounts
WHERE id = $1
