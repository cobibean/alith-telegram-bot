use alith::{Agent, LLM, WindowBufferMemory};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

// Type to store user conversation memories
type UserId = String;
type UserMemories = Arc<Mutex<HashMap<UserId, Agent>>>;

// Request and response models
#[derive(Deserialize)]
struct PromptRequest {
    user_id: String,
    message: String,
}

#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv().ok();
    
    // Create a shared state to store user conversation memories
    let user_memories = Arc::new(Mutex::new(HashMap::new()));
    
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Alith Memory Service is running!" }))
        .route("/prompt", post(handle_prompt))
        .layer(cors)
        .with_state(user_memories);
    
    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_prompt(
    State(user_memories): State<UserMemories>, 
    Json(request): Json<PromptRequest>
) -> Result<Json<PromptResponse>, StatusCode> {
    let user_id = request.user_id;
    let message = request.message;
    
    info!("Received message from user {}: {}", user_id, message);
    
    let mut memories = user_memories.lock().await;
    
    // Get or create an agent for this user
    let agent = if let Some(agent) = memories.get(&user_id) {
        agent.clone()
    } else {
        // Create a new agent with memory for this user
        info!("Creating new agent for user {}", user_id);
        
        // Use environment variable or default to gpt-4
        let model_name = env::var("ALITH_MODEL").unwrap_or_else(|_| "gpt-4".to_string());
        let model = match LLM::from_model_name(&model_name) {
            Ok(model) => model,
            Err(e) => {
                tracing::error!("Failed to create model: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        
        // Get the preamble from env var or use default
        let preamble = env::var("ALITH_PREAMBLE").unwrap_or_else(|_| 
            "You are Vic, a no-nonsense, street-smart sports predictions expert. Born in the rough streets of Philly and forged in the gritty underground of gambling, I bring you predictions with a side of attitude. I may call you a few names along the way—think of it as tough love from a true New Yorker.".to_string()
        );
        
        // Create the agent with memory
        let agent = Agent::new("Telegram Bot Agent", model, vec![])
            .preamble(&preamble)
            .memory(WindowBufferMemory::new(10)); // Remember last 10 messages
            
        memories.insert(user_id.clone(), agent.clone());
        agent
    };
    
    // Generate response using the agent
    let response = match agent.prompt(&message).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Error generating response: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    info!("Generated response for user {}: {}", user_id, response);
    
    Ok(Json(PromptResponse { response }))
}
