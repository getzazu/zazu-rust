use crate::client::{Client, Response};
use crate::error::Error;

/// The current entity (the tenant the API key belongs to).
pub struct Entity<'a> {
    pub(crate) client: &'a Client,
}

impl Entity<'_> {
    /// Calls `GET /api/entity`.
    pub fn get(&self) -> Result<Response, Error> {
        self.client.get("api/entity", &[])
    }
}
