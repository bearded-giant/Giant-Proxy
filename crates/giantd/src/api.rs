use crate::config::{self, AppConfig, RuleState};
use crate::events::{EventBus, ProxyEvent};
use crate::rules::Rule;
use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub rules: Arc<RwLock<Vec<Rule>>>,
    pub active_profile: Arc<RwLock<Option<String>>>,
    pub event_bus: Arc<EventBus>,
    pub started_at: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/status", get(get_status))
        .route("/start", post(start_proxy))
        .route("/stop", post(stop_proxy))
        .route("/profiles", get(list_profiles))
        .route("/profiles/{name}", get(get_profile))
        .route("/use/{profile}", post(switch_profile))
        .route("/rules/{id}", get(get_rule))
        .route("/rules/{id}", put(update_rule))
        .route("/rules/{id}/toggle", post(toggle_rule))
        .route("/rules", post(add_rule))
        .route("/rules/{id}", delete(remove_rule))
        .route("/rules/reorder", post(reorder_rules))
        .route("/events", get(event_stream))
        .route("/logs", get(get_logs))
        .route("/env", get(get_env_snippet))
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "ok": true,
        "version": env!("CARGO_PKG_VERSION"),
        "pid": std::process::id()
    }))
}

async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    let rules = state.rules.read().await;
    let profile = state.active_profile.read().await;
    let config = state.config.read().await;
    let started_at = state.started_at.read().await;

    let rule_states: Vec<RuleState> = rules
        .iter()
        .map(|r| RuleState {
            id: r.id.clone(),
            enabled: r.enabled,
            matched_count: 0,
        })
        .collect();

    Json(json!({
        "running": profile.is_some(),
        "profile": *profile,
        "rules": rule_states,
        "listen_addr": format!("127.0.0.1:{}", config.listen_port),
        "routing_mode": config.routing_mode,
        "started_at": started_at.map(|t| t.to_rfc3339()),
    }))
}

async fn list_profiles() -> impl IntoResponse {
    match config::list_profiles() {
        Ok(profiles) => Json(json!({ "profiles": profiles })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn switch_profile(
    State(state): State<AppState>,
    Path(profile_name): Path<String>,
) -> impl IntoResponse {
    match config::load_profile(&profile_name) {
        Ok(profile) => {
            let rules_loaded = profile.rules.len();
            *state.rules.write().await = profile.rules;
            *state.active_profile.write().await = Some(profile_name.clone());

            state.event_bus.send(ProxyEvent::ProfileSwitched {
                profile: profile_name,
                rules_loaded,
            });

            Json(json!({ "ok": true, "rules_loaded": rules_loaded })).into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn toggle_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<String>,
) -> impl IntoResponse {
    let mut rules = state.rules.write().await;
    if let Some(rule) = rules.iter_mut().find(|r| r.id == rule_id) {
        rule.enabled = !rule.enabled;
        let enabled = rule.enabled;

        state.event_bus.send(ProxyEvent::RuleToggled {
            rule_id: rule_id.clone(),
            enabled,
        });

        Json(json!({ "id": rule_id, "enabled": enabled })).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "rule not found" })),
        )
            .into_response()
    }
}

async fn get_profile(Path(name): Path<String>) -> impl IntoResponse {
    match config::load_profile(&name) {
        Ok(profile) => {
            let rules: Vec<serde_json::Value> = profile
                .rules
                .iter()
                .map(|r| {
                    json!({
                        "id": r.id,
                        "enabled": r.enabled,
                        "preserve_host": r.preserve_host,
                        "priority": r.priority,
                    })
                })
                .collect();
            Json(json!({
                "meta": profile.meta,
                "rules": rules,
            }))
            .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn start_proxy() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn stop_proxy() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn get_rule(Path(_id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn update_rule(Path(_id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn add_rule() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn remove_rule(Path(_id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn reorder_rules() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn event_stream(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let mut rx = state.event_bus.subscribe();
    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            if socket
                .send(axum::extract::ws::Message::Text(json.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}

async fn get_logs() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "not yet implemented" })),
    )
}

async fn get_env_snippet(State(state): State<AppState>) -> impl IntoResponse {
    let config = state.config.read().await;
    let ca_path = config::config_dir().join("ca").join("giant-proxy-ca.pem");
    let snippet =
        crate::routing::generate_env_snippet(config.listen_port, &ca_path, &config.bypass_hosts);
    Json(json!({ "shell_snippet": snippet }))
}
