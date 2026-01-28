-- Create sessions table for refresh tokens
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Refresh token hash (we don't store the actual token)
    refresh_token_hash VARCHAR(255) NOT NULL UNIQUE,

    -- Device info
    user_agent TEXT,
    ip_address VARCHAR(45),

    -- Expiration
    expires_at TIMESTAMPTZ NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user's sessions
CREATE INDEX idx_sessions_user ON sessions(user_id);

-- Index for token lookup
CREATE INDEX idx_sessions_token ON sessions(refresh_token_hash);

-- Index for expired sessions cleanup (simple btree for range queries)
CREATE INDEX idx_sessions_expired ON sessions(expires_at);
