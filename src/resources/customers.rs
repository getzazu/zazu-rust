use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::page::{list_page, ListParams, Page};
use crate::resources::{push_if_present, Attributes};

/// Individuals or businesses the entity invoices.
pub struct Customers<'a> {
    pub(crate) client: &'a Client,
}

/// Filters for `GET /api/customers`.
#[derive(Debug, Clone, Default)]
pub struct CustomerListParams {
    /// Shared pagination inputs.
    pub list: ListParams,
    /// Free-text query matching company name, person name, email.
    pub q: Option<String>,
}

impl Customers<'_> {
    /// Calls `GET /api/customers`.
    pub fn list(&self, params: CustomerListParams) -> Result<Page, Error> {
        let mut query = Vec::new();
        push_if_present(&mut query, "q", params.q.as_ref());
        list_page(self.client, "api/customers", query, &params.list)
    }

    /// Calls `GET /api/customers/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client
            .get(&format!("api/customers/{}", encode_component(id)), &[])
    }

    /// Calls `POST /api/customers`.
    pub fn create(&self, attributes: &Attributes) -> Result<Response, Error> {
        self.client.post("api/customers", Some(attributes))
    }

    /// Calls `PATCH /api/customers/:id`.
    pub fn update(&self, id: &str, attributes: &Attributes) -> Result<Response, Error> {
        self.client.patch(
            &format!("api/customers/{}", encode_component(id)),
            Some(attributes),
        )
    }

    /// Calls `DELETE /api/customers/:id`.
    pub fn delete(&self, id: &str) -> Result<Response, Error> {
        self.client
            .delete(&format!("api/customers/{}", encode_component(id)))
    }
}
