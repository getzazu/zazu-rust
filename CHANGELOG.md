# Changelog

All notable changes to `zazu-sdk` (zazu-rust) are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1]

Version alignment: the whole SDK family now releases in lockstep with zazu-ruby. No functional changes since [0.1.0].

## [0.1.0]

Initial release.

### Added

- `zazu_sdk::Client` built on `ureq` (sync, dependency-light — no async runtime)
- Resources: `accounts`, `beneficiaries`, `checkout_sessions`, `customers`, `entity`, `invoices`, `payment_links`, `transfer_drafts`, `webhook_endpoints`
- Cursor-based `Page` with `next()` (max 100 records per page)
- `zazu_sdk::Error` mirroring the shared SDK error taxonomy
- Cassette-replay test harness driven by the Ruby SDK's release tarball
