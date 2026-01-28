-- Create rule_evaluations table for explainability
CREATE TABLE rule_evaluations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    rule_id UUID NOT NULL REFERENCES rules(id) ON DELETE CASCADE,

    -- Result
    matched BOOLEAN NOT NULL,

    -- Explanation (for AI rules or detailed regex matches)
    explanation TEXT,

    -- For regex: which part of the content matched
    matched_text TEXT,
    matched_field VARCHAR(50), -- 'title', 'content', 'author'

    -- Action taken
    action_taken VARCHAR(20),

    -- Timestamps
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique evaluation per article/rule pair
    UNIQUE(article_id, rule_id)
);

-- Index for article evaluations
CREATE INDEX idx_evaluations_article ON rule_evaluations(article_id);

-- Index for rule evaluations
CREATE INDEX idx_evaluations_rule ON rule_evaluations(rule_id);

-- Index for matched evaluations
CREATE INDEX idx_evaluations_matched ON rule_evaluations(rule_id, evaluated_at DESC)
    WHERE matched = TRUE;
