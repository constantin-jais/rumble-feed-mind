-- Billing tables for Stripe integration
-- Supports: subscriptions, usage tracking, invoices, dunning

-- Enum types for billing
CREATE TYPE account_status AS ENUM ('active', 'grace_period', 'suspended');
CREATE TYPE subscription_status AS ENUM ('active', 'trialing', 'past_due', 'canceled', 'incomplete', 'incomplete_expired', 'paused', 'unpaid');
CREATE TYPE billing_interval AS ENUM ('month', 'year');
CREATE TYPE dunning_action AS ENUM ('email_sent', 'downgrade', 'suspend', 'restore');
CREATE TYPE usage_type AS ENUM ('ai_tokens', 'api_calls');

-- Add account_status to users table
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS account_status account_status NOT NULL DEFAULT 'active',
    ADD COLUMN IF NOT EXISTS suspended_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS suspension_reason TEXT;

-- Index for finding suspended users
CREATE INDEX IF NOT EXISTS idx_users_account_status ON users(account_status) WHERE deleted_at IS NULL;

-- ============================================================================
-- STRIPE CUSTOMERS
-- Maps internal users to Stripe customer IDs
-- ============================================================================
CREATE TABLE stripe_customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    -- Stripe identifiers
    stripe_customer_id VARCHAR(255) NOT NULL UNIQUE,

    -- Customer data (cached from Stripe)
    email VARCHAR(255),
    name VARCHAR(255),
    currency VARCHAR(3) DEFAULT 'eur',

    -- Default payment method (cached)
    default_payment_method_id VARCHAR(255),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stripe_customers_stripe_id ON stripe_customers(stripe_customer_id);
CREATE INDEX idx_stripe_customers_user_id ON stripe_customers(user_id);

CREATE TRIGGER update_stripe_customers_updated_at
    BEFORE UPDATE ON stripe_customers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SUBSCRIPTIONS
-- Cache of Stripe subscription state + dunning management
-- ============================================================================
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stripe_customer_id UUID NOT NULL REFERENCES stripe_customers(id) ON DELETE CASCADE,

    -- Stripe identifiers
    stripe_subscription_id VARCHAR(255) NOT NULL UNIQUE,
    stripe_price_id VARCHAR(255) NOT NULL,
    stripe_product_id VARCHAR(255) NOT NULL,

    -- Plan info
    plan_name VARCHAR(50) NOT NULL, -- 'pro', 'team'
    billing_interval billing_interval NOT NULL,

    -- Status
    status subscription_status NOT NULL DEFAULT 'active',

    -- Billing dates
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,
    billing_cycle_anchor TIMESTAMPTZ, -- Normalized anchor date

    -- Trial info
    trial_start TIMESTAMPTZ,
    trial_end TIMESTAMPTZ,

    -- Cancellation info
    cancel_at_period_end BOOLEAN NOT NULL DEFAULT FALSE,
    canceled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Dunning state
    dunning_started_at TIMESTAMPTZ, -- When grace period started
    last_payment_error TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX idx_subscriptions_stripe_id ON subscriptions(stripe_subscription_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
CREATE INDEX idx_subscriptions_dunning ON subscriptions(dunning_started_at) WHERE dunning_started_at IS NOT NULL;

CREATE TRIGGER update_subscriptions_updated_at
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- USAGE RECORDS
-- Individual usage events (IA tokens, API calls)
-- ============================================================================
CREATE TABLE usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    subscription_id UUID REFERENCES subscriptions(id) ON DELETE SET NULL,

    -- Usage type and quantity
    usage_type usage_type NOT NULL,
    quantity BIGINT NOT NULL,

    -- Context (for debugging/auditing)
    metadata JSONB DEFAULT '{}',

    -- For Stripe metered billing sync
    stripe_usage_record_id VARCHAR(255),
    synced_to_stripe BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_usage_records_user_id ON usage_records(user_id);
CREATE INDEX idx_usage_records_recorded_at ON usage_records(recorded_at);
CREATE INDEX idx_usage_records_type ON usage_records(usage_type);
CREATE INDEX idx_usage_records_not_synced ON usage_records(synced_to_stripe) WHERE synced_to_stripe = FALSE;

-- ============================================================================
-- USAGE DAILY
-- Aggregated daily usage for dashboards (faster queries)
-- ============================================================================
CREATE TABLE usage_daily (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Date (day granularity)
    date DATE NOT NULL,

    -- Aggregated counts
    ai_tokens BIGINT NOT NULL DEFAULT 0,
    api_calls BIGINT NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(user_id, date)
);

CREATE INDEX idx_usage_daily_user_date ON usage_daily(user_id, date);

CREATE TRIGGER update_usage_daily_updated_at
    BEFORE UPDATE ON usage_daily
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- INVOICES
-- Cache of Stripe invoices for quick access
-- ============================================================================
CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    subscription_id UUID REFERENCES subscriptions(id) ON DELETE SET NULL,

    -- Stripe identifiers
    stripe_invoice_id VARCHAR(255) NOT NULL UNIQUE,
    stripe_invoice_number VARCHAR(100),

    -- Invoice details
    status VARCHAR(50) NOT NULL, -- 'draft', 'open', 'paid', 'uncollectible', 'void'
    currency VARCHAR(3) NOT NULL DEFAULT 'eur',

    -- Amounts (in smallest currency unit, e.g., cents)
    amount_due BIGINT NOT NULL,
    amount_paid BIGINT NOT NULL DEFAULT 0,
    amount_remaining BIGINT NOT NULL,
    subtotal BIGINT NOT NULL,
    tax BIGINT DEFAULT 0,
    total BIGINT NOT NULL,

    -- URLs
    hosted_invoice_url TEXT,
    invoice_pdf TEXT,

    -- Dates
    period_start TIMESTAMPTZ,
    period_end TIMESTAMPTZ,
    due_date TIMESTAMPTZ,
    paid_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_invoices_user_id ON invoices(user_id);
CREATE INDEX idx_invoices_stripe_id ON invoices(stripe_invoice_id);
CREATE INDEX idx_invoices_status ON invoices(status);

CREATE TRIGGER update_invoices_updated_at
    BEFORE UPDATE ON invoices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- PAYMENT METHODS
-- Metadata about payment methods (card details cached from Stripe)
-- ============================================================================
CREATE TABLE payment_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stripe_customer_id UUID NOT NULL REFERENCES stripe_customers(id) ON DELETE CASCADE,

    -- Stripe identifier
    stripe_payment_method_id VARCHAR(255) NOT NULL UNIQUE,

    -- Card details (safe to store - no full number)
    card_brand VARCHAR(50), -- 'visa', 'mastercard', etc.
    card_last4 VARCHAR(4),
    card_exp_month INTEGER,
    card_exp_year INTEGER,

    -- Flags
    is_default BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_methods_user_id ON payment_methods(user_id);
CREATE INDEX idx_payment_methods_stripe_id ON payment_methods(stripe_payment_method_id);

CREATE TRIGGER update_payment_methods_updated_at
    BEFORE UPDATE ON payment_methods
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- BILLING EVENTS
-- Audit trail for billing-related events (compliance)
-- ============================================================================
CREATE TABLE billing_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Event type
    event_type VARCHAR(100) NOT NULL, -- 'subscription.created', 'payment.succeeded', etc.

    -- Source
    stripe_event_id VARCHAR(255), -- For idempotency

    -- Event data (full payload for debugging)
    payload JSONB NOT NULL DEFAULT '{}',

    -- Processing status
    processed BOOLEAN NOT NULL DEFAULT TRUE,
    error_message TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_billing_events_user_id ON billing_events(user_id);
CREATE INDEX idx_billing_events_stripe_id ON billing_events(stripe_event_id);
CREATE INDEX idx_billing_events_type ON billing_events(event_type);
CREATE INDEX idx_billing_events_created_at ON billing_events(created_at);

-- ============================================================================
-- DUNNING HISTORY
-- Track dunning actions for audit and analysis
-- ============================================================================
CREATE TABLE dunning_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    subscription_id UUID REFERENCES subscriptions(id) ON DELETE SET NULL,

    -- Action taken
    action dunning_action NOT NULL,

    -- Details
    details JSONB DEFAULT '{}', -- Email sent, tier before/after, etc.

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dunning_history_user_id ON dunning_history(user_id);
CREATE INDEX idx_dunning_history_subscription_id ON dunning_history(subscription_id);
CREATE INDEX idx_dunning_history_action ON dunning_history(action);

-- ============================================================================
-- WEBHOOK IDEMPOTENCY
-- Prevent duplicate webhook processing
-- ============================================================================
CREATE TABLE webhook_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stripe_event_id VARCHAR(255) NOT NULL UNIQUE,
    event_type VARCHAR(100) NOT NULL,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Auto-cleanup: events older than 30 days can be safely deleted
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webhook_events_stripe_id ON webhook_events(stripe_event_id);
CREATE INDEX idx_webhook_events_created_at ON webhook_events(created_at);
