use std::sync::Arc;

use axum::{extract::Extension, response::IntoResponse, Json};
use serde::Serialize;

pub(crate) async fn healthcheck(
    Extension(app): Extension<Arc<crate::Application>>,
) -> impl IntoResponse {
    Json(HealthcheckResponse {
        status: "available".to_string(),
        environment: app.config.env.clone(),
        version: crate::VERSION.to_string(),
    })
}

#[derive(Serialize)]
struct HealthcheckResponse {
    status: String,
    environment: String,
    version: String,
}
