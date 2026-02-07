-- Create feeds table
CREATE TABLE feeds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES folders(id) ON DELETE SET NULL,

    -- Feed info
    url VARCHAR(2048) NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    site_url VARCHAR(2048),
    icon_url VARCHAR(2048),

    -- Feed type: 'rss', 'atom', 'json'
    feed_type VARCHAR(20),

    -- Polling priority: 'hot' (15min), 'warm' (1h), 'cold' (4h)
    priority VARCHAR(20) NOT NULL DEFAULT 'warm',

    -- Polling state
    last_fetched_at TIMESTAMPTZ,
    last_successful_fetch_at TIMESTAMPTZ,
    etag VARCHAR(255),
    last_modified VARCHAR(255),

    -- Error tracking
    error_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    last_error_at TIMESTAMPTZ,

    -- Stats
    article_count INTEGER NOT NULL DEFAULT 0,
    unread_count INTEGER NOT NULL DEFAULT 0,

    -- Display settings
    position INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique URL per user
    UNIQUE(user_id, url)
);

-- Index for user's feeds
CREATE INDEX idx_feeds_user ON feeds(user_id);

-- Index for folder
CREATE INDEX idx_feeds_folder ON feeds(folder_id);

-- Index for feeds needing refresh
CREATE INDEX idx_feeds_refresh ON feeds(priority, last_fetched_at)
    WHERE error_count < 5;

-- Trigger for updated_at
CREATE TRIGGER update_feeds_updated_at
    BEFORE UPDATE ON feeds
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
