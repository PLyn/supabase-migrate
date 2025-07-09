mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Changed Box to Box<dyn std::error::Error> for better error handling
    use crate::handlers::auth::{signout_handler, status_handler};
    use axum::{
        Router,
        routing::{get, post},
    };
    use handlers::migrate::{migrate_handler, preview_handler, project_list_handler};
    use handlers::oauth::{callback_handler, login_handler};
    use handlers::test_handler;
    use models::{AppConfig, AppState};
    use reqwest::Method;
    use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    use time::Duration;
    use tower_http::cors::CorsLayer; // Any for methods/headers is fine
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

    let app_config = AppConfig::from_env()?;
    let app_state = AppState {
        config: app_config.clone(),
    };
    let server_addr = app_state.config.server_addr.to_owned();

    let session_store = MemoryStore::default();
    let session_expiry = Expiry::OnInactivity(Duration::hours(6));
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true if deploying with HTTPS (Render handles HTTPS usually)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(session_expiry);

    // Configure CORS for production
    let cors = CorsLayer::new()
        .allow_origin([
            "https://supabase-migrate.onrender.com".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST]) // Allows all HTTP methods
        .allow_headers([
            AUTHORIZATION, // For sending tokens
            ACCEPT,        // Standard header
            CONTENT_TYPE,  // Because you send JSON
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(test_handler))
        .route("/auth", get(status_handler))
        .route("/signout", post(signout_handler))
        .route("/projects", get(project_list_handler))
        .route("/preview", get(preview_handler))
        .route("/migrate", get(migrate_handler))
        .route("/connect-supabase/login", get(login_handler))
        .route("/connect-supabase/oauth2/callback", get(callback_handler))
        .layer(cors) // Add CORS layer
        .layer(session_layer)
        .with_state(app_state);

    eprintln!("listening on {}", server_addr);
    let listener = tokio::net::TcpListener::bind(server_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
