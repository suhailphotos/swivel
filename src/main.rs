use anyhow::{bail, Context, Result};
use std::env;

#[derive(Debug)]
struct ApiResponse {
    data: String, // pretty JSON if possible; raw text otherwise
}

fn api_request(page_id: &str) -> Result<ApiResponse> {
    // 1) Read API key from the environment
    let api_key = env::var("NOTION_API_KEY")
        .context("NOTION_API_KEY is not set in the environment")?;

    // 2) Build and send the request
    let url = format!("https://api.notion.com/v1/pages/{page_id}");
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Notion-Version", "2025-09-03")
        .send()
        .context("failed to send HTTP request to Notion")?;

    // 3) Map HTTP status codes into rich errors
    let status = resp.status();
    if !status.is_success() {
        if status.as_u16() == 401 || status.as_u16() == 403 {
            bail!("unauthorized: HTTP {status}");
        } else if status.as_u16() == 404 {
            bail!("not found: HTTP {status}");
        } else if status.is_server_error() {
            bail!("server error: HTTP {status}");
        } else {
            bail!("request failed: HTTP {status}");
        }
    }

    // 4) Read body and try to pretty-print JSON
    let text = resp.text().context("failed to read response body")?;
    let pretty = serde_json::from_str::<serde_json::Value>(&text)
        .map(|v| serde_json::to_string_pretty(&v).unwrap_or_else(|_| text.clone()))
        .unwrap_or(text);

    Ok(ApiResponse { data: pretty })
}

fn main() -> Result<()> {
    // Optional CLI arg for page id; otherwise use your sample
    let default_id = "275a1865-b187-807a-adea-ebaf36fb49b0".to_string();
    let page_id = env::args().nth(1).unwrap_or(default_id);

    let res = api_request(&page_id).context("Notion API call failed")?;
    println!("Data received:\n{}", res.data);

    Ok(())
}
