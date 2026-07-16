use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::resources::Attributes;

/// API-initiated transfers. Creating a transfer draft routes it into the
/// workspace's in-app approval flow — the API never executes a transfer
/// itself. Poll [`get`](TransferDrafts::get) (status: `requested` →
/// `processing` → `completed` / `failed`) or subscribe to the
/// `transfer.executed` webhook.
pub struct TransferDrafts<'a> {
    pub(crate) client: &'a Client,
}

impl TransferDrafts<'_> {
    /// Calls `POST /api/transfer_drafts`.
    ///
    /// Required: `account_id`, `amount`, and exactly one of `beneficiary_id`
    /// (external transfer) or `destination_account_id` (own-account move).
    ///
    /// The created draft awaits approval in the workspace's in-app approval
    /// flow — the API never executes a transfer itself.
    pub fn create(&self, attributes: &Attributes) -> Result<Response, Error> {
        self.client.post("api/transfer_drafts", Some(attributes))
    }

    /// Calls `GET /api/transfer_drafts/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client.get(
            &format!("api/transfer_drafts/{}", encode_component(id)),
            &[],
        )
    }
}
