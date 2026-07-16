use serde_json::json;

use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::page::{list_page, ListParams, Page};
use crate::resources::{push_if_present, Attributes};

/// Invoices and their lifecycle actions.
pub struct Invoices<'a> {
    pub(crate) client: &'a Client,
}

/// Filters for `GET /api/invoices`.
#[derive(Debug, Clone, Default)]
pub struct InvoiceListParams {
    /// Shared pagination inputs.
    pub list: ListParams,
    /// Filter by invoice status.
    pub status: Option<String>,
    /// Filter by customer id.
    pub customer_id: Option<String>,
}

impl Invoices<'_> {
    /// Calls `GET /api/invoices`.
    pub fn list(&self, params: InvoiceListParams) -> Result<Page, Error> {
        let mut query = Vec::new();
        push_if_present(&mut query, "status", params.status.as_ref());
        push_if_present(&mut query, "customer_id", params.customer_id.as_ref());
        list_page(self.client, "api/invoices", query, &params.list)
    }

    /// Calls `GET /api/invoices/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client
            .get(&format!("api/invoices/{}", encode_component(id)), &[])
    }

    /// Calls `POST /api/invoices`.
    pub fn create(&self, attributes: &Attributes) -> Result<Response, Error> {
        self.client.post("api/invoices", Some(attributes))
    }

    /// Calls `PATCH /api/invoices/:id`.
    pub fn update(&self, id: &str, attributes: &Attributes) -> Result<Response, Error> {
        self.client.patch(
            &format!("api/invoices/{}", encode_component(id)),
            Some(attributes),
        )
    }

    /// Calls `POST /api/invoices/:id/send`.
    pub fn send(&self, id: &str) -> Result<Response, Error> {
        self.client
            .post(&format!("api/invoices/{}/send", encode_component(id)), None)
    }

    /// Calls `POST /api/invoices/:id/mark_as_paid`.
    pub fn mark_as_paid(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/invoices/{}/mark_as_paid", encode_component(id)),
            None,
        )
    }

    /// Calls `POST /api/invoices/:id/cancel`.
    pub fn cancel(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/invoices/{}/cancel", encode_component(id)),
            None,
        )
    }

    /// Calls `POST /api/invoices/:id/credit_note`.
    pub fn credit_note(&self, id: &str) -> Result<Response, Error> {
        self.client.post(
            &format!("api/invoices/{}/credit_note", encode_component(id)),
            None,
        )
    }

    /// Calls `DELETE /api/invoices/:id`.
    pub fn delete(&self, id: &str) -> Result<Response, Error> {
        self.client
            .delete(&format!("api/invoices/{}", encode_component(id)))
    }

    /// Calls `POST /api/invoices/:invoice_id/payment_link`.
    pub fn create_payment_link(
        &self,
        invoice_id: &str,
        account_id: &str,
    ) -> Result<Response, Error> {
        self.client.post(
            &format!("api/invoices/{}/payment_link", encode_component(invoice_id)),
            Some(&json!({ "account_id": account_id })),
        )
    }
}
