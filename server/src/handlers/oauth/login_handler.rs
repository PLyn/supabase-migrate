use crate::models::AppState;
use crate::models::oauth::OAuthSessionData;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use oauth2::{CsrfToken, PkceCodeChallenge};
use tower_sessions::Session;

pub async fn login_handler(
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let csrf_token = CsrfToken::new_random();

    let mut url = reqwest::Url::parse("https://api.supabase.com/v1/oauth/authorize")
        .expect("Failed to parse auth URL");

    url.query_pairs_mut()
        .append_pair("client_id", &app_state.config.client_id)
        .append_pair("redirect_uri", &app_state.config.redirect_url.as_str()) // This is the backend's callback URL
        .append_pair("response_type", "code")
        .append_pair("state", csrf_token.secret())
        .append_pair("code_challenge", &pkce_challenge.as_str())
        .append_pair("code_challenge_method", "S256");

    let constructed_url = url.to_string();

    let session_data = OAuthSessionData {
        pkce_verifier_secret: Some(pkce_verifier.secret().to_string()),
        csrf_token_secret: Some(csrf_token.secret().to_string()),
    };

    // Store the data in the session
    session.insert("oauth_data", session_data).await.unwrap();

    // Instead of redirecting, return the URL as JSON
    Redirect::to(&constructed_url).into_response()
}
