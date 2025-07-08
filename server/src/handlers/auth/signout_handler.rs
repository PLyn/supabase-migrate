use crate::models::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tower_sessions::Session;
use tower_sessions::session::Error;

pub async fn signout_handler(
    State(_app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let token_result: Result<Option<String>, Error> = session.get("supabase_access_token").await;

    match token_result {
        Ok(Some(_token)) => match session.remove::<String>("supabase_access_token").await {
            Ok(_) => {
                eprintln!("Successfully signed out user");
                StatusCode::OK
            }
            Err(_) => {
                eprintln!("Failed to remove token from session");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Ok(None) => {
            // No token found - user wasn't signed in
            eprintln!("User was not signed in");
            StatusCode::BAD_REQUEST
        }
        Err(_) => {
            // Session error
            eprintln!("Session error occurred");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
