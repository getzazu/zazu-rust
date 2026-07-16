use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::page::{list_page, ListParams, Page};
use crate::resources::Attributes;

/// Webhook endpoint management.
pub struct WebhookEndpoints<'a> {
    pub(crate) client: &'a Client,
}

impl WebhookEndpoints<'_> {
    /// Calls `GET /api/webhook_endpoints`.
    pub fn list(&self, params: ListParams) -> Result<Page, Error> {
        list_page(self.client, "api/webhook_endpoints", Vec::new(), &params)
    }

    /// Calls `GET /api/webhook_endpoints/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client.get(
            &format!("api/webhook_endpoints/{}", encode_component(id)),
            &[],
        )
    }

    /// Calls `POST /api/webhook_endpoints`.
    pub fn create(&self, attributes: &Attributes) -> Result<Response, Error> {
        self.client.post("api/webhook_endpoints", Some(attributes))
    }

    /// Calls `PATCH /api/webhook_endpoints/:id`.
    pub fn update(&self, id: &str, attributes: &Attributes) -> Result<Response, Error> {
        self.client.patch(
            &format!("api/webhook_endpoints/{}", encode_component(id)),
            Some(attributes),
        )
    }

    /// Calls `DELETE /api/webhook_endpoints/:id`.
    pub fn delete(&self, id: &str) -> Result<Response, Error> {
        self.client
            .delete(&format!("api/webhook_endpoints/{}", encode_component(id)))
    }

    /// Calls `POST /api/webhook_endpoints/:id/test`.
    pub fn test(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/webhook_endpoints/{}/test", encode_component(id)),
            None,
        )
    }

    /// Calls `POST /api/webhook_endpoints/:id/regenerate_secret`.
    pub fn regenerate_secret(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!(
                "api/webhook_endpoints/{}/regenerate_secret",
                encode_component(id)
            ),
            None,
        )
    }

    /// Calls `POST /api/webhook_endpoints/:id/enable`.
    pub fn enable(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/webhook_endpoints/{}/enable", encode_component(id)),
            None,
        )
    }

    /// Calls `POST /api/webhook_endpoints/:id/disable`.
    pub fn disable(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/webhook_endpoints/{}/disable", encode_component(id)),
            None,
        )
    }
}
