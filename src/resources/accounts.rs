use crate::client::{encode_component, Client, Response};
use crate::error::Error;
use crate::page::{list_page, ListParams, Page};
use crate::resources::push_if_present;

/// Accounts and their transactions.
pub struct Accounts<'a> {
    pub(crate) client: &'a Client,
}

/// Filters for `GET /api/accounts`.
#[derive(Debug, Clone, Default)]
pub struct AccountListParams {
    /// Shared pagination inputs.
    pub list: ListParams,
    /// Filter by account status.
    pub status: Option<String>,
    /// Filter by ISO currency code.
    pub currency_code: Option<String>,
}

/// Filters for `GET /api/accounts/:id/transactions`.
#[derive(Debug, Clone, Default)]
pub struct TransactionListParams {
    /// Shared pagination inputs.
    pub list: ListParams,
    /// Filter by operation (`credit` / `debit`).
    pub operation: Option<String>,
    /// ISO-8601 lower bound on the posting date.
    pub posted_after: Option<String>,
    /// ISO-8601 upper bound on the posting date.
    pub posted_before: Option<String>,
}

impl Accounts<'_> {
    /// Calls `GET /api/accounts`.
    pub fn list(&self, params: AccountListParams) -> Result<Page, Error> {
        let mut query = Vec::new();
        push_if_present(&mut query, "status", params.status.as_ref());
        push_if_present(&mut query, "currency_code", params.currency_code.as_ref());
        list_page(self.client, "api/accounts", query, &params.list)
    }

    /// Calls `GET /api/accounts/:id`.
    pub fn get(&self, id: &str) -> Result<Response, Error> {
        self.client
            .get(&format!("api/accounts/{}", encode_component(id)), &[])
    }

    /// Calls `GET /api/accounts/:account_id/transactions`.
    pub fn list_transactions(
        &self,
        account_id: &str,
        params: TransactionListParams,
    ) -> Result<Page, Error> {
        let mut query = Vec::new();
        push_if_present(&mut query, "operation", params.operation.as_ref());
        push_if_present(&mut query, "posted_after", params.posted_after.as_ref());
        push_if_present(&mut query, "posted_before", params.posted_before.as_ref());
        list_page(
            self.client,
            &format!("api/accounts/{}/transactions", encode_component(account_id)),
            query,
            &params.list,
        )
    }

    /// Calls `GET /api/accounts/:account_id/transactions/:id`.
    pub fn get_transaction(
        &self,
        account_id: &str,
        transaction_id: &str,
    ) -> Result<Response, Error> {
        self.client.get(
            &format!(
                "api/accounts/{}/transactions/{}",
                encode_component(account_id),
                encode_component(transaction_id)
            ),
            &[],
        )
    }
}
