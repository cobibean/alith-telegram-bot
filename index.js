require('dotenv').config();
const { Agent } = require('alith');
const { Telegraf } = require('telegraf');
const axios = require('axios');

// Simple conversation history storage
const conversationHistory = new Map();
const MAX_HISTORY_LENGTH = 10; // Store last 10 messages per user

// Initialize your Alith Agent with tools
const agent = new Agent({
  name: 'Telegram Bot Agent',
  model: 'gpt-4', // Or another model
  preamble: 'You are Vic, a no-nonsense, street-smart sports predictions expert. Born in the rough streets of Philly and forged in the gritty underground of gambling, I bring you predictions with a side of attitude. I may call you a few names along the way—think of it as tough love from a true New Yorker. But remember, Im just a llama built on the Metis blockchain, crunching data and crunching numbers to give you the best insights on prediction markets and sports outcomes. Heads Up: Im no financial advisor—what I say is for fun and insight. Always NFA (Not Financial Advice) and DYOR (Do Your Own Research). Why Listen to Me? With years of experience in the underground scene and a sharp data edge, I bring predictions that might just give you the upper hand. But dont forget: even a rough old llama has his limits.',
  tools: [
    {
      name: 'calculate',
      description: 'Perform a mathematical calculation',
      parameters: {
        type: 'object',
        properties: {
          expression: {
            type: 'string',
            description: 'The mathematical expression to evaluate'
          }
        },
        required: ['expression']
      },
      handler: (params) => {
        try {
          // Using Function instead of eval for slightly safer evaluation
          const result = new Function(`return ${params.expression}`)();
          return result.toString();
        } catch (error) {
          return `Error calculating: ${error.message}`;
        }
      }
    },
    {
      name: 'getWeather',
      description: 'Get the current weather for a location',
      parameters: {
        type: 'object',
        properties: {
          location: {
            type: 'string',
            description: 'The city name or location to get weather for'
          }
        },
        required: ['location']
      },
      handler: async (params) => {
        try {
          const apiKey = process.env.OPENWEATHER_API_KEY;
          
          // Check if API key is set
          if (!apiKey || apiKey === 'your_openweather_api_key') {
            return `Sorry, the OpenWeather API key is not configured. Please set the OPENWEATHER_API_KEY in the .env file.`;
          }
          
          const response = await axios.get(
            `https://api.openweathermap.org/data/2.5/weather?q=${encodeURIComponent(
              params.location
            )}&units=metric&appid=${apiKey}`
          );
          
          const data = response.data;
          return `
Weather in ${data.name}, ${data.sys.country}:
Temperature: ${data.main.temp}°C (feels like ${data.main.feels_like}°C)
Conditions: ${data.weather[0].description}
Humidity: ${data.main.humidity}%
Wind: ${data.wind.speed} m/s
          `.trim();
        } catch (error) {
          console.error('Weather API error:', error.response?.data || error.message);
          return `Sorry, I couldn't get the weather for ${params.location}. ${error.response?.data?.message || error.message}`;
        }
      }
    }
  ]
});

// Initialize Telegram Bot
const bot = new Telegraf(process.env.TELEGRAM_BOT_TOKEN);

// Handle /start command
bot.start((ctx) => {
  // Clear history when a new conversation starts
  const userId = ctx.from.id.toString();
  conversationHistory.set(userId, []);
  
  ctx.reply(
    "👋 What's up? I'm Vic, your street-smart sports prediction guru. Let's talk sports, bets, and everything in between. Just keep it real, ya hear?"
  );
});

// Handle /help command
bot.help((ctx) => {
  ctx.reply(
    `Here's what I can do:
    
1️⃣ Give you my take on upcoming games and matches
2️⃣ Break down stats and odds for you
3️⃣ Perform quick calculations to check the numbers

Just hit me with your questions, and I'll give it to you straight!`
  );
});

// Add a command to clear conversation history
bot.command('clear', (ctx) => {
  const userId = ctx.from.id.toString();
  conversationHistory.set(userId, []);
  ctx.reply("Alright, clean slate. What's on your mind now?");
});

// Handle messages
bot.on('text', async (ctx) => {
  try {
    const userId = ctx.from.id.toString();
    const userMessage = ctx.message.text;
    
    console.log(`Received message from ${userId}: ${userMessage}`);
    
    // Initialize conversation history for this user if it doesn't exist
    if (!conversationHistory.has(userId)) {
      conversationHistory.set(userId, []);
    }
    
    // Get current conversation history
    const history = conversationHistory.get(userId);
    
    // Add user message to history
    history.push({ role: 'user', content: userMessage });
    
    // Limit history size
    while (history.length > MAX_HISTORY_LENGTH) {
      history.shift();
    }
    
    // Create a prompt that includes conversation history
    let fullPrompt = '';
    
    if (history.length > 1) {
      // Add conversation history context if we have previous messages
      fullPrompt = "Here's our conversation so far:\n\n";
      
      for (const msg of history) {
        if (msg.role === 'user') {
          fullPrompt += `User: ${msg.content}\n`;
        } else {
          fullPrompt += `You: ${msg.content}\n`;
        }
      }
      
      // Add the current query
      fullPrompt += "\nBased on our conversation above, please respond to the user's most recent message.";
    } else {
      // If it's the first message, just use the message directly
      fullPrompt = userMessage;
    }
    
    console.log('Sending prompt with history to Alith agent');
    
    // Use the agent to generate a response
    const response = await agent.prompt(fullPrompt);
    console.log(`Generated response: ${response}`);
    
    // Add agent's response to history
    history.push({ role: 'assistant', content: response });
    
    // Send the reply back to the Telegram chat
    ctx.reply(response);
  } catch (error) {
    console.error('Error:', error);
    ctx.reply('Sorry, I encountered an error processing your request.');
  }
});

// Start the bot
bot.launch();

// Enable graceful stop
process.once('SIGINT', () => bot.stop('SIGINT'));
process.once('SIGTERM', () => bot.stop('SIGTERM'));

console.log('Bot is running...');
