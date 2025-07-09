use crate::handlers::migrate::json_diff;
use crate::models::AppState;
use crate::models::migrate::ProjectDiffs;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tower_sessions::session::Error;

#[derive(Debug, Deserialize)]
pub struct PreviewQuery {
    pub source_id: String,
    pub dest_id: String,
    pub auth: bool,
    pub postgrest: bool,
    pub edge_functions: bool,
    pub secrets: bool,
    pub postgres: bool,
}

#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    pub configs: Vec<ProjectDiffs>,
}

pub async fn preview_handler(
    State(_app_state): State<AppState>,
    Query(params): Query<PreviewQuery>,
    session: Session,
) -> impl IntoResponse {
    eprintln!("params: {:?}", params);

    let token_result: Result<Option<String>, Error> = session.get("supabase_access_token").await;
    let Ok(Some(token)) = token_result else {
        return Err("Management API is Unauthorized".to_string());
    };

    let mut project_config: Vec<ProjectDiffs> = Vec::with_capacity(5);

    if params.auth {
        let src_url = format!("/projects/{}/config/auth", params.source_id);
        let dest_url = format!("/projects/{}/config/auth", params.dest_id);

        process_config_diffs(&session, &token, src_url, dest_url, &mut project_config).await?;
    }

    Ok(Json(project_config))
}

pub async fn process_config_diffs(
    session: &Session,
    token: &String,
    src_url: String,
    dest_url: String,
    project_config: &mut Vec<ProjectDiffs>,
) -> Result<String, String> {
    let Ok(src_cfg) = mgmt_api_get(&token, src_url).await else {
        return Err("Error fetching source auth config".to_string());
    };

    let Ok(dest_cfg) = mgmt_api_get(&token, dest_url).await else {
        return Err("Error fetching destination auth config".to_string());
    };

    let Ok(source) = serde_json::from_str(&src_cfg) else {
        return Err("Error parsing source auth config".to_string());
    };

    let Ok(dest) = serde_json::from_str(&dest_cfg) else {
        return Err("Error parsing destination auth config".to_string());
    };

    let Some(config_entry) = json_diff("Auth".to_string(), source, dest).await else {
        return Err("Error parsing auth config diffs".to_string());
    };

    project_config.push(config_entry);

    if let Err(e) = session.insert("Auth", &src_cfg).await {
        eprintln!("Failed to insert preview results into session: {:?}", e);
    }

    Ok(String::new())
}

pub async fn mgmt_api_get(token: &String, url: String) -> Result<String, String> {
    use reqwest::header::{ACCEPT, AUTHORIZATION};

    let constructed_url = format!("https://api.supabase.com/v1{}", url);

    let client = reqwest::Client::new();
    let api_result = client
        .get(&constructed_url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/json")
        .send()
        .await;

    let Ok(api_response) = api_result else {
        return Err("Error getting response from API".to_string());
    };
    let response_status = &api_response.status().is_success();

    let Ok(api_text) = api_response.text().await else {
        return Err("Error reading response body as text".to_string());
    };

    if *response_status {
        Ok(api_text)
    } else {
        let error_text = format!("HTTP error - {}", api_text);
        Ok(error_text)
    }
}
