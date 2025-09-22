use anyhow::Result;
use scylla::client::session::Session;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    session: Arc<Session>,
}

impl Database {
    pub fn new(session: Session) -> Self {
        Self {
            session: Arc::new(session),
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub async fn health_check(&self) -> Result<bool> {
        let result = self
            .session
            .query_unpaged("SELECT now() FROM system.local", &[])
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}