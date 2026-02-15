//! Plans endpoint - list available subscription plans

use axum::{extract::State, Json};
use serde::Serialize;

use super::models::*;
use super::service::get_plan_limits;
use crate::error::ApiResult;
use crate::state::AppState;

/// Plans response
#[derive(Serialize)]
pub struct PlansResponse {
    data: Vec<Plan>,
}

/// List available plans
pub async fn list_plans(
    State(_state): State<AppState>,
) -> ApiResult<Json<PlansResponse>> {
    let plans = vec![
        Plan {
            tier: PlanTier::Free,
            name: "Free".to_string(),
            description: "Pour démarrer avec FeedMind".to_string(),
            price_monthly: 0,
            price_annual: 0,
            features: vec![
                PlanFeature {
                    name: "Flux RSS".to_string(),
                    description: "Jusqu'à 100 flux".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Règles de filtrage".to_string(),
                    description: "Jusqu'à 10 règles".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "IA de base".to_string(),
                    description: "10K tokens/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "API".to_string(),
                    description: "1K appels/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Import/Export OPML".to_string(),
                    description: "Import et export de vos flux".to_string(),
                    included: true,
                },
            ],
            limits: get_plan_limits(PlanTier::Free),
        },
        Plan {
            tier: PlanTier::Pro,
            name: "Pro".to_string(),
            description: "Pour les utilisateurs avancés".to_string(),
            price_monthly: 999, // 9.99€
            price_annual: 9990, // 99.90€ (2 mois offerts)
            features: vec![
                PlanFeature {
                    name: "Flux RSS".to_string(),
                    description: "Jusqu'à 10 000 flux".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Règles de filtrage".to_string(),
                    description: "Jusqu'à 500 règles".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "IA avancée".to_string(),
                    description: "500K tokens/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "API".to_string(),
                    description: "50K appels/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Règles IA".to_string(),
                    description: "Filtrage intelligent par IA".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Priorité de refresh".to_string(),
                    description: "Vos flux sont mis à jour en priorité".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "BYOK".to_string(),
                    description: "Utilisez vos propres clés API".to_string(),
                    included: true,
                },
            ],
            limits: get_plan_limits(PlanTier::Pro),
        },
        Plan {
            tier: PlanTier::Team,
            name: "Team".to_string(),
            description: "Pour les équipes et organisations".to_string(),
            price_monthly: 2999, // 29.99€
            price_annual: 29990, // 299.90€ (2 mois offerts)
            features: vec![
                PlanFeature {
                    name: "Tout Pro inclus".to_string(),
                    description: "Toutes les fonctionnalités Pro".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "IA illimitée".to_string(),
                    description: "2M tokens/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "API haute fréquence".to_string(),
                    description: "200K appels/mois".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Support prioritaire".to_string(),
                    description: "Réponse sous 24h".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Webhooks".to_string(),
                    description: "Notifications en temps réel".to_string(),
                    included: true,
                },
                PlanFeature {
                    name: "Analytics avancées".to_string(),
                    description: "Statistiques détaillées".to_string(),
                    included: true,
                },
            ],
            limits: get_plan_limits(PlanTier::Team),
        },
    ];

    Ok(Json(PlansResponse { data: plans }))
}
