-- Create folders table for organizing feeds
CREATE TABLE folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name VARCHAR(100) NOT NULL,
    parent_id UUID REFERENCES folders(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique folder name per user at same level
    UNIQUE(user_id, parent_id, name)
);

-- Index for user's folders
CREATE INDEX idx_folders_user ON folders(user_id);

-- Index for parent folders
CREATE INDEX idx_folders_parent ON folders(parent_id);

-- Trigger for updated_at
CREATE TRIGGER update_folders_updated_at
    BEFORE UPDATE ON folders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
