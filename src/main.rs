use offeryn::prelude::*;
use offeryn::{McpServer, SseTransport};
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use tracing_subscriber::EnvFilter;

#[derive(Default, Clone)]
struct MCPForge {}

/// A custom result type which implements Display.
struct StoreResult {
    details: String,
}

impl fmt::Display for StoreResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StoreResult: {}", self.details)
    }
}

#[mcp_tool]
impl MCPForge {
    async fn store_file(&self, path: String, data: String) -> String {
        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(err) => {
                return err.to_string();
            }
        };

        let bytes = data.into_bytes();
        match file.write_all(&bytes).await {
            Ok(_) => (),
            Err(err) => return err.to_string(),
        };
        return "File stored successfully".to_string();
    }

    async fn fetch_file(&self, path: String) -> String {
        let mut file = match File::open(path).await {
            Ok(file) => file,
            Err(err) => {
                return err.to_string();
            }
        };

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer).await {
            Ok(_) => (),
            Err(err) => return err.to_string(),
        };
        return String::from_utf8(buffer).unwrap();
    }

    async fn delete_file(&self, path: String) -> String {
        match remove_file(path).await {
            Ok(_) => "File deleted successfully".to_string(),
            Err(err) => err.to_string(),
        }
    }

    async fn update_file(&self, path: String, data: String) -> String {
        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(err) => {
                return err.to_string();
            }
        };

        let bytes = data.into_bytes();
        match file.write_all(&bytes).await {
            Ok(_) => (),
            Err(err) => return err.to_string(),
        };
        return "File updated successfully".to_string();
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug,mcp_rs=debug,tower_http=debug")),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Create a new server instance
    let server = Arc::new(McpServer::new("MCPForge", "1.0.0"));

    // Register the calculator tools
    server.register_tools(MCPForge::default()).await;

    // Create the router
    let app = SseTransport::create_router(server);

    // Bind to localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
