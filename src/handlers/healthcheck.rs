use std::sync::Arc;

use axum::{extract::Extension, response::IntoResponse, Json};
use serde_json::json;

pub(crate) async fn healthcheck(
    Extension(app): Extension<Arc<crate::Application>>,
) -> impl IntoResponse {
    Json(json!({
        "status": "available",
        "system_info": {
            "environment": app.config.env,
            "version": crate::VERSION,
        },
    }))
}
