use crate::models::AppState;
use crate::models::migrate::{AuthConfigStruct, MigrateProjectStruct};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_sessions::Session;
use tower_sessions::session::Error;

pub async fn migrate_handler(
    State(_app_state): State<AppState>,
    Query(params): Query<MigrateProjectStruct>,
    session: Session,
) -> impl IntoResponse {
    let token_result: Result<Option<String>, Error> = session.get("supabase_access_token").await;
    let Ok(Some(token)) = token_result else {
        return Err("Management API is Unauthorized".to_string());
    };

    if params.auth {
        let auth_json_result: Result<Option<String>, Error> = session.get("Auth").await;
        let Ok(Some(auth_json)) = auth_json_result else {
            return Err("Error getting Auth config from session".to_string());
        };

        let Ok(auth_config): Result<AuthConfigStruct, _> = serde_json::from_str(&auth_json) else {
            return Err("Error parsing session auth config".to_string());
        };

        let patched_config: AuthConfigStruct = auth_config.remove_smtp_fields_if_disabled();

        let Ok(patched_json): Result<String, _> = serde_json::to_string(&patched_config) else {
            return Err("Error converting auth config to JSOn".to_string());
        };

        let auth_url = format!("/projects/{}/config/auth", params.dest_id);
        let constructed_url = format!("https://api.supabase.com/v1{}", auth_url);
        let client = reqwest::Client::new();

        let patch_result = client
            .patch(&constructed_url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .body(patched_json)
            .send()
            .await;

        //get rid of session data regardless of results
        session.remove::<String>("Auth").await.ok();

        match patch_result {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err("Error migrating auth config".to_string());
                }
            }
            Err(err) => return Err(format!("Error sending patch request: {}", err)),
        }
    }

    Ok(String::new())
}
