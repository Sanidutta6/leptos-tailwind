use anyhow::{Context, Result};
use gloo_net::http::Request;
use serde::{Serialize, de::DeserializeOwned};

pub(super) async fn api_request<T>(
    method: &str,
    url: &str,
    body: Option<impl Serialize>,
) -> Result<T>
where
    T: DeserializeOwned + 'static,
{
    // 1. Initialize the RequestBuilder
    let mut builder = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PUT" => Request::put(url),
        "DELETE" => Request::delete(url),
        "PATCH" => Request::patch(url),
        other => anyhow::bail!("Unsupported HTTP method: {other}"),
    };

    // 2. Add common headers (Returns RequestBuilder)
    builder = builder.header("Content-Type", "application/json");

    // 3. Finalize into a Request object
    // check if body is required to add -
    let add_req_body = match method {
        "POST" => true,
        "PUT" => true,
        "PATCH" => true,
        _ => false,
    };
    let req = if add_req_body == true && let Some(body) = body {
        // .json() consumes builder and returns Result<Request, Error>
        builder.json(&body)?
    } else {
        // .build() consumes builder and returns Result<Request, Error>
        builder.build()?
    };

    // 4. Send finalized request
    let resp = req.send().await.context("network request failed")?;

    let status = resp.status();
    if status >= 400 {
        let status_text = resp.status_text().to_string();
        anyhow::bail!("HTTP {status} {status_text}");
    }

    resp.json::<T>()
        .await
        .context("failed to parse JSON response")
}
