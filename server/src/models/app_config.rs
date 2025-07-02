#[derive(Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub client_addr: String,
    pub server_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let client_id = env::var("SUPA_CONNECT_CLIENT_ID")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_ID not found: {}", e))?;
        let client_secret = env::var("SUPA_CONNECT_CLIENT_SECRET")
            .map_err(|e| format!("SUPA_CONNECT_CLIENT_SECRET not found: {}", e))?;
        let redirect_url =
            env::var("REDIRECT_URL").map_err(|e| format!("REDIRECT_URL not found: {}", e))?;
        let client_addr =
            env::var("CLIENT_ADDR").map_err(|e| format!("CLIENT_ADDR not found: {}", e))?;
        let server_addr =
            env::var("SERVER_ADDR").map_err(|e| format!("SERVER_ADDR not found: {}", e))?;
        Ok(Self {
            client_id,
            client_secret,
            redirect_url,
            client_addr,
            server_addr,
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}
