use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct ChatQuery {
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LLMRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct LLMResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}

async fn home_handler() -> Result<Html<String>, StatusCode> {
    // Read HTML file from static directory
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Ok(Html(content)),
        Err(_) => {
            // Fallback HTML if file doesn't exist
            let fallback_html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>AI Chat - File Not Found</title>
</head>
<body>
    <h1>Welcome to AI Chat</h1>
    <p>Please ensure static/index.html exists in your project directory.</p>
    <p>The chat API is available at <code>/api/chat</code></p>
</body>
</html>
            "#;
            Ok(Html(fallback_html.to_string()))
        }
    }
}

async fn chat_handler(
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<HashMap<String, String>>, StatusCode> {
    let message = payload.get("message").unwrap_or(&"Hello".to_string()).clone();
    
    // Call LLM API
    let response = call_llm_api(&message).await.unwrap_or_else(|_| {
        "I'm currently experiencing some technical difficulties. Please try again later.".to_string()
    });
    
    let mut result = HashMap::new();
    result.insert("response".to_string(), response);
    
    Ok(Json(result))
}

async fn call_llm_api(message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "your-api-key-here".to_string());
    
    if api_key == "your-api-key-here" {
        return Ok(format!(
            "I received your message: '{}'. This is a demo response. To enable real AI responses, configure your OPENAI_API_KEY environment variable.",
            message
        ));
    }
    
    let client = reqwest::Client::new();
    
    let request_body = LLMRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful AI assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: message.to_string(),
            },
        ],
        max_tokens: 150,
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    
    if response.status().is_success() {
        let llm_response: LLMResponse = response.json().await?;
        Ok(llm_response.choices[0].message.content.clone())
    } else {
        Err(format!("API request failed with status: {}", response.status()).into())
    }
}

#[tokio::main]
async fn main() {
    // Get the port from the environment, defaulting to 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/api/chat", post(chat_handler))
        // Serve static files (CSS, JS, images, etc.)
        .nest_service("/static", ServeDir::new("static"));

    // Run the server
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    
    println!("🚀 Server listening on {}", addr);
    println!("💡 Visit http://localhost:{} to start chatting!", port);
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}