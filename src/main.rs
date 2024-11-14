use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use native_tls::TlsConnector;
use serde::Serialize;
use tokio_tungstenite::{connect_async, connect_async_tls_with_config};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use url::Url;

// Define response structure that can be serialized to JSON
#[derive(Serialize)]
struct ProbeResult {
    status: String,      // Success/failure status
    error: Option<String>, // Optional error message
}

// Define HTTP GET endpoint at /probe-ws
// web::Query extracts query parameters into a HashMap
#[get("/probe-ws")]
async fn probe_ws(web::Query(params): web::Query<std::collections::HashMap<String, String>>) -> Result<HttpResponse> {
    println!("Received probe request: {:?}", params);

    // Extract url parameter from query string
    let url = match params.get("url") {
        Some(url) => url,
        None => {
            return Ok(HttpResponse::BadRequest().json(ProbeResult {
                status: "failed".to_string(),
                error: Some("url parameter is required".to_string()),
            }));
        }
    };

    // Parse string URL into Url struct for validation
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(ProbeResult {
                status: "failed".to_string(),
                error: Some(format!("Invalid URL: {}", e)),
            }));
        }
    };

    // Attempt WebSocket connection and return appropriate response
    match try_connect(url).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ProbeResult {
            status: "success".to_string(),
            error: None,
        })),
        Err(e) => Ok(HttpResponse::NotFound().json(ProbeResult {
            status: "failed".to_string(),
            error: Some(format!("Connection failed: {}", e)),
        })),
    }
}

// Helper function to handle different WebSocket connection types
async fn try_connect(url: Url) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match url.scheme() {
        "ws" => {
            // Plain WebSocket connection
            connect_async(url).await?;
        }
        "wss" => {
            // Secure WebSocket connection
            let request = url.into_client_request()?;
            // Create TLS connector for secure connection
            let connector = TlsConnector::builder()
                .build()?;
            let connector = tokio_tungstenite::Connector::NativeTls(connector);

            // Attempt secure connection with TLS
            connect_async_tls_with_config(
                request,
                None,        // No custom config
                true,        // Enable compression
                Some(connector),
            ).await?;
        },
        // Return error for invalid schemes (not ws:// or wss://)
        _ => return Err(format!("Invalid scheme: {}", url.scheme()).into()),
    };

    Ok(())
}

// Entry point with async runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 9555;
    println!("Starting server on port {}", port);

    // Create and start HTTP server
    HttpServer::new(|| {
        App::new()
            .service(probe_ws)  // Register our endpoint
    })
        .bind(("0.0.0.0", port))?  // Listen on all interfaces
        .run()
        .await
}