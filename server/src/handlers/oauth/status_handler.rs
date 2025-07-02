use crate::models::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use tower_sessions::Session;
use tower_sessions::session::Error;

pub async fn status_handler(
    State(_app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let token_result: Result<Option<String>, Error> = session.get("supabase_access_token").await;
    if let Ok(Some(_token)) = token_result {
        eprintln!("auth true");
        "true"
    } else {
        eprintln!("auth false");
        "false"
    }
}
