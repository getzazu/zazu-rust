//! Mirror of zazu-ruby's spec/zazu/resources/*_spec.rb — same cassettes,
//! same assertions, per the cross-language SDK contract.

mod common;

use common::{fixture_id, ReplayServer};
use serde_json::json;
use zazu_sdk::Client;

fn replay_client(server: &ReplayServer) -> Client {
    Client::builder()
        .api_key("test-api-key-for-replay")
        .base_url(&server.url)
        .build()
        .expect("build client")
}

#[test]
fn entity_get() {
    let server = ReplayServer::start(&["entity/get"]);
    let client = replay_client(&server);

    let resp = client.entity().get().expect("entity get");
    assert!(
        resp.body["id"].is_string(),
        "expected string id, got {:?}",
        resp.body["id"]
    );
}

#[test]
fn accounts() {
    let server = ReplayServer::start(&[
        "accounts/list",
        "accounts/get",
        "accounts/list_transactions",
        "accounts/get_transaction",
    ]);
    let client = replay_client(&server);

    let page = client
        .accounts()
        .list(Default::default())
        .expect("accounts list");
    assert!(!page.data.is_empty(), "expected data rows");

    let account_id = fixture_id("ZAZU_FIXTURE_ACCOUNT_ID");
    client.accounts().get(account_id).expect("accounts get");

    client
        .accounts()
        .list_transactions(account_id, Default::default())
        .expect("list transactions");

    let tx_id = fixture_id("ZAZU_FIXTURE_TRANSACTION_ID");
    client
        .accounts()
        .get_transaction(account_id, tx_id)
        .expect("get transaction");
}

#[test]
fn customers() {
    let server = ReplayServer::start(&["customers/list", "customers/get"]);
    let client = replay_client(&server);

    client
        .customers()
        .list(Default::default())
        .expect("customers list");

    let customer_id = fixture_id("ZAZU_FIXTURE_CUSTOMER_ID");
    let resp = client.customers().get(customer_id).expect("customers get");
    assert!(
        resp.body["id"].is_string(),
        "expected string id, got {:?}",
        resp.body["id"]
    );
}

#[test]
fn invoices() {
    let server = ReplayServer::start(&["invoices/list", "invoices/get"]);
    let client = replay_client(&server);

    let page = client
        .invoices()
        .list(Default::default())
        .expect("invoices list");
    assert!(!page.data.is_empty(), "expected data rows");

    let invoice_id = fixture_id("ZAZU_FIXTURE_INVOICE_ID");
    client.invoices().get(invoice_id).expect("invoices get");
}

#[test]
fn payment_links() {
    let server = ReplayServer::start(&[
        "payment_links/list",
        "payment_links/get",
        "payment_links/create",
        "payment_links/cancel",
    ]);
    let client = replay_client(&server);

    client
        .payment_links()
        .list(Default::default())
        .expect("payment links list");

    let resp = client
        .payment_links()
        .create(&json!({
            "account_id": fixture_id("ZAZU_FIXTURE_ACCOUNT_ID"),
            "amount": "100.00",
            "title": "SDK fixture",
            "description": "Created by zazu-ruby fixture spec",
            "link_type": "single",
        }))
        .expect("payment links create");
    assert_eq!(resp.status, 201, "expected 201, got {}", resp.status);

    client
        .payment_links()
        .cancel(fixture_id("ZAZU_FIXTURE_CANCELLABLE_PAYMENT_LINK_ID"))
        .expect("payment links cancel");
}

#[test]
fn checkout_sessions() {
    let server = ReplayServer::start(&["checkout_sessions/get"]);
    let client = replay_client(&server);

    let resp = client
        .checkout_sessions()
        .get(fixture_id("ZAZU_FIXTURE_CHECKOUT_SESSION_ID"))
        .expect("checkout sessions get");
    assert!(
        resp.body["id"].is_string(),
        "expected string id, got {:?}",
        resp.body["id"]
    );
}

#[test]
fn webhook_endpoints() {
    let server = ReplayServer::start(&["webhook_endpoints/list", "webhook_endpoints/get"]);
    let client = replay_client(&server);

    client
        .webhook_endpoints()
        .list(Default::default())
        .expect("webhook endpoints list");
    client
        .webhook_endpoints()
        .get(fixture_id("ZAZU_FIXTURE_WEBHOOK_ID"))
        .expect("webhook endpoints get");
}

#[test]
fn transfer_drafts() {
    let server = ReplayServer::start(&["transfer_drafts/create", "transfer_drafts/get"]);
    let client = replay_client(&server);

    let resp = client
        .transfer_drafts()
        .create(&json!({
            "account_id": fixture_id("ZAZU_FIXTURE_ACCOUNT_ID"),
            "beneficiary_id": fixture_id("ZAZU_FIXTURE_BENEFICIARY_ID"),
            "amount": "150.00",
            "payment_reference": "SDK fixture",
        }))
        .expect("transfer drafts create");
    assert_eq!(resp.status, 201, "expected 201, got {}", resp.status);
    assert_eq!(
        resp.body["status"].as_str(),
        Some("requested"),
        "expected requested status (awaiting in-app approval), got {:?}",
        resp.body["status"]
    );
    assert!(
        resp.body["transfer"].is_null(),
        "expected null transfer before approval, got {:?}",
        resp.body["transfer"]
    );

    let got = client
        .transfer_drafts()
        .get(fixture_id("ZAZU_FIXTURE_TRANSFER_DRAFT_ID"))
        .expect("transfer drafts get");
    assert!(
        got.body["status"].is_string(),
        "expected string status, got {:?}",
        got.body["status"]
    );
}

#[test]
fn beneficiaries() {
    let server = ReplayServer::start(&["beneficiaries/list", "beneficiaries/get"]);
    let client = replay_client(&server);

    let page = client
        .beneficiaries()
        .list(Default::default())
        .expect("beneficiaries list");
    assert!(!page.data.is_empty(), "expected at least one beneficiary");
    assert!(
        page.data[0]["external_accounts"].is_array(),
        "expected embedded external_accounts, got {:?}",
        page.data[0].get("external_accounts")
    );

    let resp = client
        .beneficiaries()
        .get(fixture_id("ZAZU_FIXTURE_BENEFICIARY_ID"))
        .expect("beneficiaries get");
    assert!(
        resp.body["id"].is_string(),
        "expected string id, got {:?}",
        resp.body["id"]
    );
}
