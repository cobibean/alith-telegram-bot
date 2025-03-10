use alith::{Agent, LLM, WindowBufferMemory};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::BotCommand;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;
use tracing::{info, error};

// Define a wrapper type for our HashMap that uses u64 as the key
// This is important because UserId from teloxide is u64
type AgentStorage = Arc<Mutex<HashMap<u64, Agent<LLM>>>>;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv().ok();
    
    // Create a shared state to store user agents
    let agent_storage = Arc::new(Mutex::new(HashMap::<u64, Agent<LLM>>::new()));
    
    // Initialize the bot
    let bot = Bot::from_env();
    
    // Register commands
    let commands = vec![
        BotCommand::new("start", "Start a new conversation"),
        BotCommand::new("help", "Show help information"),
        BotCommand::new("clear", "Clear conversation history"),
    ];
    
    bot.set_my_commands(commands).await.unwrap();
    
    // Start the bot with message handlers
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(handle_command)
        )
        .branch(
            Update::filter_message()
                .filter(|msg: Message| msg.text().is_some())
                .endpoint(|bot: Bot, msg: Message, agent_storage: AgentStorage| async move {
                    handle_message(bot, msg, agent_storage).await
                })
        );
    
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![agent_storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

// Bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Start a new conversation")]
    Start,
    #[command(description = "Show help information")]
    Help,
    #[command(description = "Clear conversation history")]
    Clear,
}

// Handle bot commands
async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    agent_storage: AgentStorage,
) -> ResponseResult<()> {
    // Convert UserId to u64 for HashMap key
    let user_id = msg.from
        .map(|user| user.id.0)
        .unwrap_or(0);
    
    match cmd {
        Command::Start => {
            // Send welcome message
            bot.send_message(
                msg.chat.id,
                "👋 What's up? I'm Vic, your street-smart sports prediction guru. Let's talk sports, bets, and everything in between. Just keep it real, ya hear?"
            ).await?;
        }
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                "Here's what I can do:\n\n1️⃣ Give you my take on upcoming games and matches\n2️⃣ Break down stats and odds for you\n3️⃣ Perform quick calculations to check the numbers\n\nJust hit me with your questions, and I'll give it to you straight!"
            ).await?;
        }
        Command::Clear => {
            // Clear the agent's memory by creating a new one
            let mut agents = agent_storage.lock().await;
            if agents.contains_key(&user_id) {
                // Remove the agent to clear memory
                agents.remove(&user_id);
                
                bot.send_message(
                    msg.chat.id,
                    "Alright, clean slate. What's on your mind now?"
                ).await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    "No conversation to clear. Let's start fresh!"
                ).await?;
            }
        }
    }
    
    Ok(())
}

// Handle regular messages
async fn handle_message(
    bot: Bot,
    msg: Message,
    agent_storage: AgentStorage,
) -> ResponseResult<()> {
    // Clone the message early to avoid borrow issues
    let text = msg.text().unwrap().to_string();
    let user_id = msg.from
        .map(|user| user.id.0)
        .unwrap_or(0);
    let chat_id = msg.chat.id;
    
    info!("Received message from user {}: {}", user_id, text);
    
    let mut agents = agent_storage.lock().await;
    
    // Check if we already have an agent for this user
    if agents.contains_key(&user_id) {
        // Use the existing agent
        if let Some(agent) = agents.get_mut(&user_id) {
            match agent.prompt(&text).await {
                Ok(response_text) => {
                    info!("Generated response for user {}: {}", user_id, response_text);
                    bot.send_message(chat_id, response_text).await?;
                }
                Err(e) => {
                    error!("Error generating response: {}", e);
                    bot.send_message(
                        chat_id,
                        "Sorry, I encountered an error processing your request."
                    ).await?;
                }
            }
        }
    } else {
        // Create a new agent with memory for this user
        info!("Creating new agent for user {}", user_id);
        
        // Use environment variable or default to gpt-4
        let model_name = env::var("ALITH_MODEL").unwrap_or_else(|_| "gpt-4".to_string());
        let model = match LLM::from_model_name(&model_name) {
            Ok(model) => model,
            Err(e) => {
                error!("Failed to create model: {}", e);
                bot.send_message(
                    chat_id,
                    "Sorry, I encountered an error setting up. Please try again later."
                ).await?;
                return Ok(());
            }
        };
        
        // Get the preamble from env var or use default
        let preamble = env::var("ALITH_PREAMBLE").unwrap_or_else(|_| 
            "You are Vic, a no-nonsense, street-smart sports predictions expert...".to_string()
        );
        
        // Create the agent with memory
        let agent = Agent::new("Telegram Bot Agent", model, vec![])
            .preamble(&preamble)
            .memory(WindowBufferMemory::new(10)); // Remember last 10 messages
        
        // Generate the first response
        match agent.prompt(&text).await {
            Ok(response_text) => {
                // Insert the agent into the map after using it
                agents.insert(user_id, agent);
                
                info!("Generated response for user {}: {}", user_id, response_text);
                bot.send_message(chat_id, response_text).await?;
            }
            Err(e) => {
                error!("Error generating response: {}", e);
                bot.send_message(
                    chat_id,
                    "Sorry, I encountered an error processing your request."
                ).await?;
            }
        }
    }
    
    Ok(())
}
