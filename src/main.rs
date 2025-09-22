use std::env;

#[derive(Debug)]
enum ApiError {
    MissingApiKey,
    ConnectionFailed,
    Unauthorized,
    NotFound,
    ServerError,
    InvalidResponse,
}

#[derive(Debug)]
struct ApiResponse {
    data: String, // raw JSON (pretty-printed if possible)
}

fn api_request(page_id: &str) -> Result<ApiResponse, ApiError> {
    // 1) Read API key from the environment
    let api_key = env::var("NOTION_API_KEY").map_err(|_| ApiError::MissingApiKey)?;

    // 2) Build the request
    let url = format!("https://api.notion.com/v1/pages/{page_id}");
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Notion-Version", "2025-09-03")
        .send()
        .map_err(|_| ApiError::ConnectionFailed)?;

    // 3) Map HTTP status codes into our enum
    let status = resp.status();
    if status.is_success() {
        let text = resp.text().map_err(|_| ApiError::InvalidResponse)?;

        // Try to pretty-print JSON; fall back to raw text if it isn’t JSON
        let pretty = serde_json::from_str::<serde_json::Value>(&text)
            .map(|v| serde_json::to_string_pretty(&v).unwrap_or(text.clone()))
            .unwrap_or(text);

        Ok(ApiResponse { data: pretty })
    } else if status.as_u16() == 401 || status.as_u16() == 403 {
        Err(ApiError::Unauthorized)
    } else if status.as_u16() == 404 {
        Err(ApiError::NotFound)
    } else if status.is_server_error() {
        Err(ApiError::ServerError)
    } else {
        Err(ApiError::InvalidResponse)
    }
}

fn main() {
    // Optional: take page id from the first CLI arg; otherwise use your example
    let default_id = "273a1865-b187-805e-88d3-cabf5f3cdf04".to_string();
    let page_id = env::args().nth(1).unwrap_or(default_id);

    let api_result: Result<ApiResponse, ApiError> = api_request(&page_id);

    // Handle the result
    match api_result {
        Ok(response) => {
            println!("Data received:\n{}", response.data);
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }
}
