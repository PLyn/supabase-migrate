use crate::models::AppState;
use crate::models::migrate::ProjectList;

use axum::{Json, extract::State, response::IntoResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use tower_sessions::Session;
use tower_sessions::session::Error;

pub async fn project_list_handler(
    State(_app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let token_result: Result<Option<String>, Error> = session.get("supabase_access_token").await;
    let Ok(Some(token)) = token_result else {
        return Err("Management API is Unauthorized".to_string());
    };

    let constructed_url = format!("https://api.supabase.com/v1/projects");

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

    let Ok(project_list) = api_response.json::<Vec<ProjectList>>().await else {
        return Err("Error getting Projects".to_string());
    };

    Ok(Json(project_list))
}
