-- Create tags table
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name VARCHAR(100) NOT NULL,
    color VARCHAR(7), -- Hex color like #FF5733

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique tag name per user
    UNIQUE(user_id, name)
);

-- Index for user's tags
CREATE INDEX idx_tags_user ON tags(user_id);

-- Create article_tags junction table
CREATE TABLE article_tags (
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,

    -- Who/what applied this tag
    applied_by VARCHAR(20) NOT NULL DEFAULT 'user', -- 'user', 'rule'
    rule_id UUID REFERENCES rules(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (article_id, tag_id)
);

-- Index for tag's articles
CREATE INDEX idx_article_tags_tag ON article_tags(tag_id);
