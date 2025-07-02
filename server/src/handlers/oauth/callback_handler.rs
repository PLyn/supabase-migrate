use crate::models::AppState;
use crate::models::oauth::{CallbackParams, OAuthSessionData, TokenResponse};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::PkceCodeVerifier;
use tower_sessions::Session;

pub async fn callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    let client_addr = app_state.config.client_addr.to_owned();

    eprintln!(
        "OAuth callback received. Code: {}, State: {}",
        params.code, params.state
    );

    // Retrieve OAuth data from session
    let oauth_session_data: Option<OAuthSessionData> = match session.get("oauth_data").await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to retrieve session data: {:?}", e);
            return Redirect::to(
                format!("{}/auth?status=error&reason=session_error", client_addr).as_str(),
            )
            .into_response();
        }
    };

    eprintln!(
        "Session ID: {:?} to get oauth retrieved from session: {:?}",
        session.id(),
        oauth_session_data
    );

    let oauth_data = match oauth_session_data {
        Some(data) => data,
        None => {
            eprintln!("No oauth_data found in session");
            return Redirect::to(
                format!("{}/auth?status=error&reason=no_session_data", client_addr).as_str(),
            )
            .into_response();
        }
    };

    // Clean up session data
    session.remove::<OAuthSessionData>("oauth_data").await.ok();

    // Validate PKCE verifier
    let pkce_verifier_secret = match oauth_data.pkce_verifier_secret {
        Some(secret) => secret,
        None => {
            eprintln!("No PKCE verifier found in session");
            return Redirect::to(
                format!("{}/auth?status=error&reason=no_pkce", client_addr).as_str(),
            )
            .into_response();
        }
    };

    // Validate CSRF token
    let original_csrf_secret = match oauth_data.csrf_token_secret {
        Some(token) => token,
        None => {
            eprintln!("No CSRF token found in session");
            return Redirect::to(
                format!("{}/auth?status=error&reason=no_csrf", client_addr).as_str(),
            )
            .into_response();
        }
    };

    // Check CSRF token match
    if original_csrf_secret != params.state {
        eprintln!(
            "CSRF token mismatch. Expected: {}, Got: {}",
            original_csrf_secret, params.state
        );
        return Redirect::to(
            format!("{}/auth?status=error&reason=csrf_mismatch", client_addr).as_str(),
        )
        .into_response();
    }

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_secret);
    let client = reqwest::Client::new();

    let form_params = [
        ("client_id", app_state.config.client_id.as_str()),
        ("client_secret", app_state.config.client_secret.as_str()),
        ("code", params.code.as_str()),
        ("code_verifier", pkce_verifier.secret()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", app_state.config.redirect_url.as_str()),
    ];

    // Exchange authorization code for access token
    let response = match client
        .post("https://api.supabase.com/v1/oauth/token")
        .form(&form_params)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to exchange token: {:?}", e);
            return Redirect::to(
                format!(
                    "{}/auth?status=error&reason=token_exchange_failed",
                    client_addr
                )
                .as_str(),
            )
            .into_response();
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not read error body".to_string());
        eprintln!("Failed to exchange token (HTTP {}): {}", status, error_text);

        let error_reason = match status.as_u16() {
            400 => "bad_request",
            401 => "unauthorized",
            403 => "forbidden",
            _ => "token_exchange_error",
        };
        return Redirect::to(
            format!("{}/auth?status=error&reason={}", client_addr, error_reason).as_str(),
        )
        .into_response();
    }

    let token_data = match response.json::<TokenResponse>().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse token response: {:?}", e);
            return Redirect::to(
                format!("{}/auth?status=error&reason=token_parse_error", client_addr).as_str(),
            )
            .into_response();
        }
    };

    // Store access token in session
    if let Err(e) = session
        .insert("supabase_access_token", token_data.access_token.clone())
        .await
    {
        eprintln!("Failed to store access token in session: {:?}", e);
        return Redirect::to(
            format!(
                "{}/auth?status=error&reason=session_store_error",
                client_addr
            )
            .as_str(),
        )
        .into_response();
    }

    if let Some(refresh_token) = token_data.refresh_token {
        eprintln!(
            "Refresh Token received (store securely if needed for long-term use): {}",
            refresh_token
        );
        // Optionally store refresh token in session as well
        // session.insert("supabase_refresh_token", refresh_token).await.ok();
    }

    eprintln!("Authentication successful");

    // Redirect back to frontend on success
    Redirect::to(format!("{}/auth?status=success", client_addr).as_str()).into_response()
}
