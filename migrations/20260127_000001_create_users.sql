-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),

    -- Subscription tier: 'free', 'pro_trial', 'pro', 'team'
    tier VARCHAR(20) NOT NULL DEFAULT 'free',
    trial_ends_at TIMESTAMPTZ,

    -- BYOK API keys (encrypted with AES-256-GCM)
    anthropic_key_encrypted BYTEA,
    google_key_encrypted BYTEA,
    encryption_key_version INTEGER DEFAULT 1,

    -- Settings
    settings JSONB NOT NULL DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,

    -- Soft delete for GDPR
    deleted_at TIMESTAMPTZ
);

-- Index for email lookups
CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;

-- Index for active users
CREATE INDEX idx_users_active ON users(id) WHERE deleted_at IS NULL;

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
