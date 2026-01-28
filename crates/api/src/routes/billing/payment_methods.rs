//! Payment method endpoints

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use super::models::*;
use super::service::BillingService;
use crate::error::{ApiError, ApiResult};
use crate::extractors::auth::CurrentUser;
use crate::state::AppState;

/// Response wrapper
#[derive(Serialize)]
pub struct DataResponse<T> {
    data: T,
}

/// List response with metadata
#[derive(Serialize)]
pub struct ListResponse<T> {
    data: Vec<T>,
    meta: ListMeta,
}

#[derive(Serialize)]
pub struct ListMeta {
    total: usize,
}

/// Success response
#[derive(Serialize)]
pub struct SuccessResponse {
    success: bool,
}

/// List payment methods
pub async fn list_payment_methods(
    State(state): State<AppState>,
    user: CurrentUser,
) -> ApiResult<Json<ListResponse<PaymentMethodResponse>>> {
    let stripe = state.stripe().ok_or_else(|| ApiError::BadRequest("Billing not enabled".to_string()))?;
    let service = BillingService::new(state.db(), stripe, state.stripe_config());

    let methods = service.list_payment_methods(user.id).await?;
    let total = methods.len();

    Ok(Json(ListResponse {
        data: methods.into_iter().map(|m| m.into()).collect(),
        meta: ListMeta { total },
    }))
}

/// Add payment method
pub async fn add_payment_method(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<AddPaymentMethodRequest>,
) -> ApiResult<Json<DataResponse<PaymentMethodResponse>>> {
    let stripe = state.stripe().ok_or_else(|| ApiError::BadRequest("Billing not enabled".to_string()))?;
    let service = BillingService::new(state.db(), stripe, state.stripe_config());

    let method = service.add_payment_method(user.id, &req.payment_method_id, req.set_default).await?;

    Ok(Json(DataResponse { data: method.into() }))
}

/// Delete payment method
pub async fn delete_payment_method(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DataResponse<SuccessResponse>>> {
    let stripe = state.stripe().ok_or_else(|| ApiError::BadRequest("Billing not enabled".to_string()))?;
    let service = BillingService::new(state.db(), stripe, state.stripe_config());

    service.delete_payment_method(user.id, id).await?;

    Ok(Json(DataResponse {
        data: SuccessResponse { success: true },
    }))
}

/// Set payment method as default
pub async fn set_default_payment_method(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DataResponse<PaymentMethodResponse>>> {
    let stripe = state.stripe().ok_or_else(|| ApiError::BadRequest("Billing not enabled".to_string()))?;
    let service = BillingService::new(state.db(), stripe, state.stripe_config());

    let method = service.set_default_payment_method(user.id, id).await?;

    Ok(Json(DataResponse { data: method.into() }))
}
