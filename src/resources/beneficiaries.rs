use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::page::{list_page, ListParams, Page};

/// Read-only directory of saved transfer recipients. Each beneficiary embeds
/// its bank accounts; the one flagged `default` is used when a transfer names
/// only the `beneficiary_id`. Beneficiaries are created and managed in the
/// Zazu dashboard — the API never creates or modifies them.
pub struct Beneficiaries<'a> {
    pub(crate) client: &'a Client,
}

impl Beneficiaries<'_> {
    /// Calls `GET /api/beneficiaries`.
    pub fn list(&self, params: ListParams) -> Result<Page, Error> {
        list_page(self.client, "api/beneficiaries", Vec::new(), &params)
    }

    /// Calls `GET /api/beneficiaries/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client
            .get(&format!("api/beneficiaries/{}", encode_component(id)), &[])
    }
}
