-- Add categories support for articles
-- Phase 1: Store categories, allow filtering
-- Phase 2: Filter at subscription level

-- ============================================================================
-- Phase 1: Categories on articles
-- ============================================================================

-- Column to store article categories as JSONB array
ALTER TABLE articles ADD COLUMN categories JSONB NOT NULL DEFAULT '[]'::jsonb;

-- GIN index for efficient containment queries (@>, ?|, etc.)
CREATE INDEX idx_articles_categories ON articles USING GIN (categories);

-- ============================================================================
-- Aggregation table for UI (discovered categories per feed)
-- ============================================================================

CREATE TABLE feed_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feed_id UUID NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    category VARCHAR(255) NOT NULL,
    article_count INTEGER NOT NULL DEFAULT 0,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(feed_id, category)
);

-- Index for fast lookup by feed
CREATE INDEX idx_feed_categories_feed ON feed_categories(feed_id);

-- ============================================================================
-- Phase 2: Category filter at subscription level
-- ============================================================================

-- Filter configuration on feeds
-- Format: { "mode": "include"|"exclude", "categories": ["cat1", "cat2"] }
-- NULL means no filter (import all categories)
ALTER TABLE feeds ADD COLUMN category_filter JSONB DEFAULT NULL;

-- ============================================================================
-- Helper function: Update feed_categories aggregation
-- ============================================================================

CREATE OR REPLACE FUNCTION update_feed_categories()
RETURNS TRIGGER AS $$
DECLARE
    cat TEXT;
BEGIN
    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
        -- Process each category in the new article
        FOR cat IN SELECT jsonb_array_elements_text(NEW.categories)
        LOOP
            INSERT INTO feed_categories (feed_id, category, article_count, first_seen_at, last_seen_at)
            VALUES (NEW.feed_id, cat, 1, NOW(), NOW())
            ON CONFLICT (feed_id, category) DO UPDATE SET
                article_count = feed_categories.article_count + 1,
                last_seen_at = NOW();
        END LOOP;
    END IF;

    IF TG_OP = 'DELETE' THEN
        -- Decrement counts for deleted article's categories
        FOR cat IN SELECT jsonb_array_elements_text(OLD.categories)
        LOOP
            UPDATE feed_categories
            SET article_count = article_count - 1
            WHERE feed_id = OLD.feed_id AND category = cat;
        END LOOP;

        -- Clean up zero-count categories
        DELETE FROM feed_categories
        WHERE feed_id = OLD.feed_id AND article_count <= 0;
    END IF;

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_feed_categories_trigger
    AFTER INSERT OR DELETE ON articles
    FOR EACH ROW
    EXECUTE FUNCTION update_feed_categories();
