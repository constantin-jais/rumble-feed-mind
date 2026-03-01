//! Billing models and DTOs

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// ENUMS
// ============================================================================

/// Account status for dunning management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AccountStatus {
    #[default]
    Active,
    GracePeriod,
    Suspended,
}

/// Subscription status (mirrors Stripe)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Trialing,
    PastDue,
    Canceled,
    Incomplete,
    IncompleteExpired,
    Paused,
    Unpaid,
}

/// Billing interval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "billing_interval", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BillingInterval {
    Month,
    Year,
}

/// Usage type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "usage_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UsageType {
    AiTokens,
    ApiCalls,
}

/// Dunning action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dunning_action", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DunningAction {
    EmailSent,
    Downgrade,
    Suspend,
    Restore,
}

// ============================================================================
// PLAN DEFINITIONS
// ============================================================================

/// Plan tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTier {
    Free,
    Pro,
    Team,
}

impl PlanTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlanTier::Free => "free",
            PlanTier::Pro => "pro",
            PlanTier::Team => "team",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pro" => PlanTier::Pro,
            "team" => PlanTier::Team,
            _ => PlanTier::Free,
        }
    }
}

/// Plan details for display
#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub tier: PlanTier,
    pub name: String,
    pub description: String,
    pub price_monthly: i64, // cents
    pub price_annual: i64,  // cents (total for year)
    pub features: Vec<PlanFeature>,
    pub limits: PlanLimits,
}

/// Feature included in a plan
#[derive(Debug, Clone, Serialize)]
pub struct PlanFeature {
    pub name: String,
    pub description: String,
    pub included: bool,
}

/// Plan limits
#[derive(Debug, Clone, Serialize)]
pub struct PlanLimits {
    pub feeds: u32,
    pub rules: u32,
    pub articles: u32,
    pub api_calls_per_month: u32,
    pub ai_tokens_per_month: u32,
}

// ============================================================================
// DATABASE MODELS
// ============================================================================

/// Stripe customer mapping
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct StripeCustomer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stripe_customer_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub currency: Option<String>,
    pub default_payment_method_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subscription record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stripe_customer_id: Uuid,
    pub stripe_subscription_id: String,
    pub stripe_price_id: String,
    pub stripe_product_id: String,
    pub plan_name: String,
    pub billing_interval: BillingInterval,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub billing_cycle_anchor: Option<DateTime<Utc>>,
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    pub dunning_started_at: Option<DateTime<Utc>>,
    pub last_payment_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Usage record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UsageRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub usage_type: UsageType,
    pub quantity: i64,
    pub metadata: serde_json::Value,
    pub stripe_usage_record_id: Option<String>,
    pub synced_to_stripe: bool,
    pub recorded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Daily usage aggregate
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UsageDaily {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub ai_tokens: i64,
    pub api_calls: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub stripe_invoice_id: String,
    pub stripe_invoice_number: Option<String>,
    pub status: String,
    pub currency: String,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub amount_remaining: i64,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub total: i64,
    pub hosted_invoice_url: Option<String>,
    pub invoice_pdf: Option<String>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payment method record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct PaymentMethod {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stripe_customer_id: Uuid,
    pub stripe_payment_method_id: String,
    pub card_brand: Option<String>,
    pub card_last4: Option<String>,
    pub card_exp_month: Option<i32>,
    pub card_exp_year: Option<i32>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Billing event (audit trail)
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct BillingEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub stripe_event_id: Option<String>,
    pub payload: serde_json::Value,
    pub processed: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Dunning history entry
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DunningHistoryEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub action: DunningAction,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// API REQUEST/RESPONSE TYPES
// ============================================================================

/// Request to create a subscription
#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan: PlanTier,
    pub interval: BillingInterval,
    /// Payment method ID (optional, will use default if not provided)
    pub payment_method_id: Option<String>,
}

/// Request to change plan
#[derive(Debug, Deserialize)]
pub struct ChangePlanRequest {
    pub plan: PlanTier,
    pub interval: BillingInterval,
    /// If true, change takes effect immediately with prorated charges
    #[serde(default = "default_true")]
    pub prorate: bool,
}

fn default_true() -> bool {
    true
}

/// Request to cancel subscription
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    /// Reason for cancellation (optional)
    pub reason: Option<String>,
    /// If true, cancel immediately instead of at period end
    #[serde(default)]
    pub immediate: bool,
}

/// Request to add payment method
#[derive(Debug, Deserialize)]
pub struct AddPaymentMethodRequest {
    /// Stripe PaymentMethod ID (created via Stripe Elements on frontend)
    pub payment_method_id: String,
    /// Set as default payment method
    #[serde(default)]
    pub set_default: bool,
}

/// Request for Stripe portal/checkout session
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    /// Return URL after session completes
    pub return_url: String,
    /// For checkout: the plan to subscribe to
    pub plan: Option<PlanTier>,
    /// For checkout: the billing interval
    pub interval: Option<BillingInterval>,
}

/// Subscription response (API)
#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub plan: PlanTier,
    pub interval: BillingInterval,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
}

impl From<Subscription> for SubscriptionResponse {
    fn from(sub: Subscription) -> Self {
        Self {
            id: sub.id,
            plan: PlanTier::from_str(&sub.plan_name),
            interval: sub.billing_interval,
            status: sub.status,
            current_period_start: sub.current_period_start,
            current_period_end: sub.current_period_end,
            cancel_at_period_end: sub.cancel_at_period_end,
            canceled_at: sub.canceled_at,
            trial_end: sub.trial_end,
        }
    }
}

/// Current usage response
#[derive(Debug, Serialize)]
pub struct CurrentUsageResponse {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub ai_tokens: UsageMetric,
    pub api_calls: UsageMetric,
}

/// Usage metric with limit info
#[derive(Debug, Serialize)]
pub struct UsageMetric {
    pub used: i64,
    pub limit: i64,
    pub percentage: f64,
}

/// Usage history entry
#[derive(Debug, Serialize)]
pub struct UsageHistoryEntry {
    pub date: NaiveDate,
    pub ai_tokens: i64,
    pub api_calls: i64,
}

/// Invoice response (API)
#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub number: Option<String>,
    pub status: String,
    pub currency: String,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub total: i64,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub hosted_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Invoice> for InvoiceResponse {
    fn from(inv: Invoice) -> Self {
        Self {
            id: inv.id,
            number: inv.stripe_invoice_number,
            status: inv.status,
            currency: inv.currency,
            amount_due: inv.amount_due,
            amount_paid: inv.amount_paid,
            total: inv.total,
            period_start: inv.period_start,
            period_end: inv.period_end,
            due_date: inv.due_date,
            paid_at: inv.paid_at,
            pdf_url: inv.invoice_pdf,
            hosted_url: inv.hosted_invoice_url,
            created_at: inv.created_at,
        }
    }
}

/// Payment method response (API)
#[derive(Debug, Serialize)]
pub struct PaymentMethodResponse {
    pub id: Uuid,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub exp_month: Option<i32>,
    pub exp_year: Option<i32>,
    pub is_default: bool,
}

impl From<PaymentMethod> for PaymentMethodResponse {
    fn from(pm: PaymentMethod) -> Self {
        Self {
            id: pm.id,
            brand: pm.card_brand,
            last4: pm.card_last4,
            exp_month: pm.card_exp_month,
            exp_year: pm.card_exp_year,
            is_default: pm.is_default,
        }
    }
}

/// Session response (portal/checkout)
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub url: String,
}
