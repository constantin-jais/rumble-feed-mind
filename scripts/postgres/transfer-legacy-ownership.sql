-- One-time upgrade for databases created by the historical single `feedmind`
-- principal. Run as a database administrator after provision-roles.sql and
-- before `feedmind-cli migrate`. This script intentionally names only Feed Radar
-- objects; it never uses cluster-wide REASSIGN OWNED.

ALTER SCHEMA public OWNER TO feed_radar_owner;
SELECT format('ALTER DATABASE %I OWNER TO feed_radar_owner', current_database())\gexec

ALTER TABLE _sqlx_migrations OWNER TO feed_radar_owner;
ALTER TABLE users OWNER TO feed_radar_owner;
ALTER TABLE folders OWNER TO feed_radar_owner;
ALTER TABLE feeds OWNER TO feed_radar_owner;
ALTER TABLE articles OWNER TO feed_radar_owner;
ALTER TABLE rules OWNER TO feed_radar_owner;
ALTER TABLE rule_evaluations OWNER TO feed_radar_owner;
ALTER TABLE tags OWNER TO feed_radar_owner;
ALTER TABLE article_tags OWNER TO feed_radar_owner;
ALTER TABLE sessions OWNER TO feed_radar_owner;
ALTER TABLE stripe_customers OWNER TO feed_radar_owner;
ALTER TABLE subscriptions OWNER TO feed_radar_owner;
ALTER TABLE usage_records OWNER TO feed_radar_owner;
ALTER TABLE usage_daily OWNER TO feed_radar_owner;
ALTER TABLE invoices OWNER TO feed_radar_owner;
ALTER TABLE payment_methods OWNER TO feed_radar_owner;
ALTER TABLE billing_events OWNER TO feed_radar_owner;
ALTER TABLE dunning_history OWNER TO feed_radar_owner;
ALTER TABLE webhook_events OWNER TO feed_radar_owner;
ALTER TABLE feed_categories OWNER TO feed_radar_owner;

ALTER FUNCTION update_updated_at_column() OWNER TO feed_radar_owner;
ALTER FUNCTION update_feed_unread_count() OWNER TO feed_radar_owner;
ALTER FUNCTION update_feed_categories() OWNER TO feed_radar_owner;

ALTER TYPE account_status OWNER TO feed_radar_owner;
ALTER TYPE subscription_status OWNER TO feed_radar_owner;
ALTER TYPE billing_interval OWNER TO feed_radar_owner;
ALTER TYPE dunning_action OWNER TO feed_radar_owner;
ALTER TYPE usage_type OWNER TO feed_radar_owner;
