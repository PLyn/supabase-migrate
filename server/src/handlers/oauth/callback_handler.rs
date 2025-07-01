use crate::models::AppState;
use crate::models::oauth::{CallbackParams, OAuthSessionData};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use oauth2::PkceCodeVerifier;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Serialize)]
struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> Response {
    eprintln!(
        "OAuth callback received. Code: {}, State: {}",
        params.code, params.state
    );

    // Retrieve OAuth data from session
    let oauth_data: Option<OAuthSessionData> = match session.get("oauth_data").await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to retrieve session data: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: None,
                    error: Some("Failed to retrieve session data".to_string()),
                }),
            )
                .into_response();
        }
    };

    eprintln!(
        "Session ID: {:?} to get oauth retrieved from session: {:?}",
        session.id(),
        oauth_data
    );

    let oauth_data = match oauth_data {
        Some(data) => data,
        None => {
            eprintln!("No oauth_data found in session");

            // Fallback: check for direct PKCE and CSRF keys
            let pkce_verifier = session
                .get::<String>("pkce_verifier_secret")
                .await
                .ok()
                .flatten();
            let csrf_token = session
                .get::<String>("csrf_token_secret")
                .await
                .ok()
                .flatten();

            if pkce_verifier.is_some() && csrf_token.is_some() {
                eprintln!("Found direct PKCE and CSRF keys instead");
                OAuthSessionData {
                    pkce_verifier_secret: pkce_verifier,
                    csrf_token_secret: csrf_token,
                }
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        message: None,
                        error: Some(
                            "No session data found. Please try logging in again.".to_string(),
                        ),
                    }),
                )
                    .into_response();
            }
        }
    };

    // Clean up session data
    session.remove::<OAuthSessionData>("oauth_data").await.ok();

    // Validate PKCE verifier
    let pkce_verifier_secret = match oauth_data.pkce_verifier_secret {
        Some(secret) => secret,
        None => {
            eprintln!("No PKCE verifier found in session");
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    message: None,
                    error: Some(
                        "No PKCE verifier found in session. Please try logging in again."
                            .to_string(),
                    ),
                }),
            )
                .into_response();
        }
    };

    // Validate CSRF token
    let original_csrf_secret = match oauth_data.csrf_token_secret {
        Some(token) => token,
        None => {
            eprintln!("No CSRF token found in session");
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    message: None,
                    error: Some(
                        "No CSRF token found in session. Please try logging in again.".to_string(),
                    ),
                }),
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
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                message: None,
                error: Some("CSRF token mismatch. Please try logging in again.".to_string()),
            }),
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
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiResponse {
                    message: None,
                    error: Some("Failed to communicate with authentication server".to_string()),
                }),
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

        let error_status = match status.as_u16() {
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_GATEWAY,
        };

        return (
            error_status,
            Json(ApiResponse {
                message: None,
                error: Some("Failed to exchange authorization code for access token".to_string()),
            }),
        )
            .into_response();
    }

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        refresh_token: Option<String>,
    }

    let token_data = match response.json::<TokenResponse>().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse token response: {:?}", e);
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiResponse {
                    message: None,
                    error: Some("Failed to parse authentication server response".to_string()),
                }),
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
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                message: None,
                error: Some("Failed to store authentication data".to_string()),
            }),
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

    (
        StatusCode::OK,
        Json(ApiResponse {
            message: Some("Authentication successful".to_string()),
            error: None,
        }),
    )
        .into_response()
}
