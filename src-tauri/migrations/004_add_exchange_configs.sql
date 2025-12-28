-- Exchange configurations table
-- Stores encrypted API credentials for different exchanges

CREATE TABLE IF NOT EXISTS exchange_configs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,
    is_testnet INTEGER DEFAULT 0 NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_exchange_configs_user_id ON exchange_configs(user_id);
CREATE INDEX IF NOT EXISTS idx_exchange_configs_exchange_name ON exchange_configs(exchange_name);
CREATE UNIQUE INDEX IF NOT EXISTS idx_exchange_configs_user_exchange ON exchange_configs(user_id, exchange_name, created_at);
