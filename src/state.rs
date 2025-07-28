use crate::config::Config;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub keycloak_admin: Arc<RwLock<KeycloakAdmin>>,
    pub reqwest_client: reqwest::Client,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let reqwest_client = reqwest::Client::new();
        
        let admin_token = KeycloakAdminToken::acquire_custom_realm(
            &config.keycloak_url,
            &config.keycloak_username,
            &config.keycloak_password,
            &config.keycloak_realm,
            &config.keycloak_client_id,
            "password",
            &reqwest_client,
        )
        .await?;

        let keycloak_admin = KeycloakAdmin::new(
            &config.keycloak_url,
            admin_token,
            reqwest_client.clone(),
        );

        Ok(Self {
            config: config.clone(),
            keycloak_admin: Arc::new(RwLock::new(keycloak_admin)),
            reqwest_client,
        })
    }

    pub async fn refresh_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let admin_token = KeycloakAdminToken::acquire_custom_realm(
            &self.config.keycloak_url,
            &self.config.keycloak_username,
            &self.config.keycloak_password,
            &self.config.keycloak_realm,
            &self.config.keycloak_client_id,
            "password",
            &self.reqwest_client,
        )
        .await?;

        let new_admin = KeycloakAdmin::new(
            &self.config.keycloak_url,
            admin_token,
            self.reqwest_client.clone(),
        );

        *self.keycloak_admin.write().await = new_admin;

        Ok(())
    }
}