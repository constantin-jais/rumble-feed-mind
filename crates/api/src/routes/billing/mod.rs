//! Billing API routes
//!
//! Handles subscription management, usage tracking, invoices, and payment methods.

mod invoices;
mod models;
mod payment_methods;
mod plans;
mod service;
mod subscriptions;
mod usage;
mod webhooks;

pub use models::*;

use crate::state::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};

/// Build billing routes
pub fn router() -> Router<AppState> {
    Router::new()
        // Plans
        .route("/api/v1/billing/plans", get(plans::list_plans))
        // Subscription management
        .route(
            "/api/v1/billing/subscription",
            get(subscriptions::get_subscription),
        )
        .route(
            "/api/v1/billing/subscribe",
            post(subscriptions::create_subscription),
        )
        .route(
            "/api/v1/billing/change-plan",
            post(subscriptions::change_plan),
        )
        .route(
            "/api/v1/billing/cancel",
            post(subscriptions::cancel_subscription),
        )
        .route(
            "/api/v1/billing/reactivate",
            post(subscriptions::reactivate_subscription),
        )
        // Usage
        .route("/api/v1/billing/usage", get(usage::get_current_usage))
        .route(
            "/api/v1/billing/usage/history",
            get(usage::get_usage_history),
        )
        // Invoices
        .route("/api/v1/billing/invoices", get(invoices::list_invoices))
        .route("/api/v1/billing/invoices/{id}", get(invoices::get_invoice))
        .route(
            "/api/v1/billing/invoices/{id}/pdf",
            get(invoices::get_invoice_pdf),
        )
        // Payment methods
        .route(
            "/api/v1/billing/payment-methods",
            get(payment_methods::list_payment_methods),
        )
        .route(
            "/api/v1/billing/payment-methods",
            post(payment_methods::add_payment_method),
        )
        .route(
            "/api/v1/billing/payment-methods/{id}",
            delete(payment_methods::delete_payment_method),
        )
        .route(
            "/api/v1/billing/payment-methods/{id}/default",
            post(payment_methods::set_default_payment_method),
        )
        // Stripe portal session
        .route(
            "/api/v1/billing/portal-session",
            post(subscriptions::create_portal_session),
        )
        // Stripe checkout session
        .route(
            "/api/v1/billing/checkout-session",
            post(subscriptions::create_checkout_session),
        )
        // Webhook
        .route("/webhooks/stripe", post(webhooks::handle_webhook))
}
