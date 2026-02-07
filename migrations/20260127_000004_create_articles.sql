-- Create articles table
CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feed_id UUID NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Article identifiers (for deduplication)
    guid VARCHAR(2048) NOT NULL,
    url VARCHAR(2048),

    -- Content
    title VARCHAR(500) NOT NULL,
    author VARCHAR(255),
    summary TEXT,
    content TEXT,

    -- Media
    image_url VARCHAR(2048),

    -- Dates
    published_at TIMESTAMPTZ,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Reading state
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    read_at TIMESTAMPTZ,
    is_starred BOOLEAN NOT NULL DEFAULT FALSE,
    starred_at TIMESTAMPTZ,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
    hidden_at TIMESTAMPTZ,
    hidden_by_rule_id UUID,

    -- Word count for reading time estimation
    word_count INTEGER,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique guid per feed
    UNIQUE(feed_id, guid)
);

-- Index for user's articles
CREATE INDEX idx_articles_user ON articles(user_id);

-- Index for feed's articles
CREATE INDEX idx_articles_feed ON articles(feed_id);

-- Index for unread articles
CREATE INDEX idx_articles_unread ON articles(user_id, is_read, published_at DESC)
    WHERE is_read = FALSE AND is_hidden = FALSE;

-- Index for starred articles
CREATE INDEX idx_articles_starred ON articles(user_id, starred_at DESC)
    WHERE is_starred = TRUE;

-- Index for hidden articles (for cleanup)
CREATE INDEX idx_articles_hidden ON articles(user_id, hidden_at)
    WHERE is_hidden = TRUE;

-- Index for article search (basic)
CREATE INDEX idx_articles_title_search ON articles USING gin(to_tsvector('english', title));

-- Trigger for updated_at
CREATE TRIGGER update_articles_updated_at
    BEFORE UPDATE ON articles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger to update feed unread count
CREATE OR REPLACE FUNCTION update_feed_unread_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE feeds SET
            article_count = article_count + 1,
            unread_count = unread_count + 1
        WHERE id = NEW.feed_id;
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.is_read = FALSE AND NEW.is_read = TRUE THEN
            UPDATE feeds SET unread_count = unread_count - 1 WHERE id = NEW.feed_id;
        ELSIF OLD.is_read = TRUE AND NEW.is_read = FALSE THEN
            UPDATE feeds SET unread_count = unread_count + 1 WHERE id = NEW.feed_id;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE feeds SET
            article_count = article_count - 1,
            unread_count = CASE WHEN OLD.is_read THEN unread_count ELSE unread_count - 1 END
        WHERE id = OLD.feed_id;
    END IF;
    RETURN COALESCE(NEW, OLD);
END;
$$ language 'plpgsql';

CREATE TRIGGER update_feed_counts
    AFTER INSERT OR UPDATE OR DELETE ON articles
    FOR EACH ROW
    EXECUTE FUNCTION update_feed_unread_count();
