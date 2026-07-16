use serde_json::{Map, Value};

use crate::client::{Client, Response};
use crate::error::Error;

/// The API's hard page-size cap.
pub const MAX_PER_PAGE: u32 = 100;

/// Shared cursor-pagination inputs. A `limit` of `None` means
/// [`MAX_PER_PAGE`].
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    /// Page size, 1..=100.
    pub limit: Option<u32>,
    /// Opaque cursor from a previous page's `next_cursor`.
    pub cursor: Option<String>,
}

/// One page of a cursor-paginated list endpoint:
/// `{ "data": [...], "has_more": bool, "next_cursor": string|null }`.
#[derive(Debug, Clone)]
pub struct Page {
    /// The rows on this page — untyped `snake_case` objects, as-is from
    /// the API.
    pub data: Vec<Map<String, Value>>,
    /// Whether another page follows this one.
    pub has_more: bool,
    /// Cursor for the following page.
    pub next_cursor: Option<String>,
    /// The underlying API response.
    pub response: Response,

    client: Client,
    path: String,
    base_query: Vec<(String, String)>,
    limit: u32,
}

impl Page {
    /// Fetches the following page, or returns `None` when this is the last
    /// one.
    pub fn next(&self) -> Result<Option<Page>, Error> {
        match (&self.next_cursor, self.has_more) {
            (Some(cursor), true) => Some(fetch_page(
                &self.client,
                &self.path,
                &self.base_query,
                self.limit,
                Some(cursor),
            ))
            .transpose(),
            _ => Ok(None),
        }
    }
}

pub(crate) fn list_page(
    client: &Client,
    path: &str,
    base_query: Vec<(String, String)>,
    params: &ListParams,
) -> Result<Page, Error> {
    let limit = params.limit.unwrap_or(MAX_PER_PAGE);
    if limit == 0 || limit > MAX_PER_PAGE {
        return Err(Error::Configuration(format!(
            "limit must be between 1 and {MAX_PER_PAGE} (got {limit})"
        )));
    }

    fetch_page(client, path, &base_query, limit, params.cursor.as_deref())
}

fn fetch_page(
    client: &Client,
    path: &str,
    base_query: &[(String, String)],
    limit: u32,
    cursor: Option<&str>,
) -> Result<Page, Error> {
    let mut query = base_query.to_vec();
    query.push(("limit".to_owned(), limit.to_string()));
    if let Some(cursor) = cursor {
        query.push(("cursor".to_owned(), cursor.to_owned()));
    }

    let response = client.get(path, &query)?;

    let data = response
        .body
        .get("data")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter_map(|row| row.as_object().cloned())
                .collect()
        })
        .unwrap_or_default();
    let has_more = response
        .body
        .get("has_more")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let next_cursor = response
        .body
        .get("next_cursor")
        .and_then(Value::as_str)
        .map(str::to_owned);

    Ok(Page {
        data,
        has_more,
        next_cursor,
        response,
        client: client.clone(),
        path: path.to_owned(),
        base_query: base_query.to_vec(),
        limit,
    })
}
