-- Create rules table
CREATE TABLE rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Rule info
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Rule type: 'regex', 'ai' (AI rules for V1.1)
    rule_type VARCHAR(20) NOT NULL DEFAULT 'regex',

    -- Rule configuration (depends on type)
    -- For regex: { "pattern": "...", "fields": ["title", "content"], "case_sensitive": false }
    -- For AI: { "prompt": "...", "model": "claude-3-haiku" }
    config JSONB NOT NULL,

    -- Action: 'hide', 'star', 'tag', 'mark_read'
    action VARCHAR(20) NOT NULL,
    action_params JSONB, -- e.g., { "tags": ["important"] }

    -- Scope: apply to specific feed/folder or all
    feed_id UUID REFERENCES feeds(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES folders(id) ON DELETE CASCADE,

    -- Priority (higher = evaluated first)
    priority INTEGER NOT NULL DEFAULT 0,

    -- Stop evaluating other rules if this matches
    stop_on_match BOOLEAN NOT NULL DEFAULT FALSE,

    -- State
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    -- Stats
    match_count INTEGER NOT NULL DEFAULT 0,
    last_match_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user's rules
CREATE INDEX idx_rules_user ON rules(user_id);

-- Index for active rules ordered by priority
CREATE INDEX idx_rules_active ON rules(user_id, priority DESC) WHERE is_active = TRUE;

-- Index for feed-specific rules
CREATE INDEX idx_rules_feed ON rules(feed_id) WHERE feed_id IS NOT NULL;

-- Index for folder-specific rules
CREATE INDEX idx_rules_folder ON rules(folder_id) WHERE folder_id IS NOT NULL;

-- Trigger for updated_at
CREATE TRIGGER update_rules_updated_at
    BEFORE UPDATE ON rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
