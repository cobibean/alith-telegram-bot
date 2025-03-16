# Vic - Your Street-Smart Sports Prediction Guru

![Vic Banner](https://i.imgur.com/placeholder-for-vic-banner.png)

Meet Vic, a no-nonsense, street-smart sports predictions expert born in the rough streets of Philly and forged in the gritty underground of sports gambling. Vic brings predictions with attitude, crunching numbers to give you the edge on prediction markets and sports outcomes. He's not your typical polite AI assistant - he's a character with personality who tells it like it is.

## 🤔 What Problem Does Vic Solve?

In a world full of bland, corporate AI assistants that give generic sports advice, Vic stands out by:

- **Cutting Through the Noise**: Get straight-talking sports analysis without the fluff
- **Entertainment + Insight**: Enjoy a personality-driven assistant who makes predictions fun
- **Accessible Expertise**: Access sports betting insights in a casual, conversational way
- **Data-Driven Predictions**: Receive insights backed by Alith AI's powerful analytical capabilities
- **Blockchain Integration**: Built on the Metis blockchain to eventually offer prediction market integration

## 🌟 Key Features

- **Authentic Personality**: Interact with a character who has attitude and street smarts
- **Sports Predictions**: Get insights across major sports leagues and events
- **Betting Intelligence**: Access odds analysis and betting trends
- **Contextual Memory**: Vic remembers your conversation and preferences
- **High-Performance Backend**: Powered by the Rust-based Alith SDK for lightning-fast responses
- **Telegram Integration**: Available wherever you are through Telegram messaging

## 🚀 Quick Start

### Repository Setup

1. **Fork Required Repositories**
   - First, fork the main [Alith repository](https://github.com/cobibean/alith)
   - Then, fork this [Vic repository](https://github.com/cobibean/alith-telegram-bot)
   - The Vic repository should be placed in the root directory of the Alith repository for proper dependency resolution

2. **Clone Both Repositories**
   ```bash
   # Clone main Alith repository
   git clone https://github.com/YOUR_USERNAME/alith.git
   cd alith
   
   # Clone Vic repository inside the Alith repository root
   git clone https://github.com/YOUR_USERNAME/alith-telegram-bot.git VIC
   ```

### Setup and Installation

1. **Prerequisites**
   ```bash
   # Install Node.js 18.x
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs

   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

   # Install build essentials
   sudo apt install -y build-essential
   ```

2. **Build the Alith SDK**
   ```bash
   # Navigate to Alith root directory (if not already there)
   cd alith
   
   # Build Alith SDK
   cargo build --release
   cd sdks/node
   npm install
   npm run build
   cd ../../
   ```

3. **Setup Vic**
   ```bash
   # Navigate to Vic directory
   cd VIC
   npm install
   ```

4. **Configuration**
   ```bash
   # Create and edit .env file
   cp .env.example .env
   nano .env

   # Required environment variables:
   # TELEGRAM_BOT_TOKEN=your_telegram_bot_token
   # OPENAI_API_KEY=your_openai_api_key
   ```

5. **Running**
   ```bash
   # Development
   npm run dev

   # Production
   NODE_ENV=production pm2 start index.js --name vic
   ```

## 📚 Documentation

- [**AI Documentation**](./AI_DOCUMENTATION.md) - Comprehensive technical documentation for AI training and analysis
- [**Deployment Guide**](./deployment_instructions.md) - Detailed server deployment instructions
- [**Update Guide**](./update_instructions.md) - Instructions for updating the bot and SDK
- [Telegram Bot API](https://core.telegram.org/bots/api) - Official Telegram Bot API documentation

## 🛠 Development

### Architecture
```
[User] <-> [Telegram] <-> [Vic] <-> [Alith SDK] <-> [AI Models]
```

### Key Components
- **Alith SDK**: Rust-based AI processing engine
- **Node.js Application**: Main bot application
- **Telegram Integration**: Message handling and user interaction
- **PM2**: Process management and monitoring

### Development Commands
```bash
# Run tests
npm test

# Lint code
npm run lint

# Format code
npm run format

# Development mode
npm run dev
```

## 🔧 Troubleshooting

Common issues and solutions:
- **Native Module Errors**: Check `node_modules/alith/dist/alith.linux-x64-gnu.node` exists
- **Memory Issues**: Monitor with `pm2 monit`
- **Startup Errors**: Check logs with `pm2 logs vic`

For detailed troubleshooting, see the [AI Documentation](./AI_DOCUMENTATION.md#troubleshooting-guide).

## 🔐 Security

- Environment variables for sensitive data
- Input validation and sanitization
- Rate limiting and access control
- Regular security updates

## 🎯 Future Roadmap for Vic

Vic's journey is just beginning. Here's what's coming up:

- **Multi-Sport Expertise**: Expanding knowledge base to cover more niche sports and leagues
- **Real-Time Game Updates**: Live updates during games with probability shifts
- **Personalized Prediction Profiles**: Customizing advice based on your risk tolerance and interests
- **Voice Messages**: Talking trash and giving predictions in Vic's signature style
- **Prediction Market Integration**: Direct interaction with blockchain-based prediction markets
- **Public Performance Tracking**: Transparent record of Vic's prediction accuracy
- **Community Features**: Group chats where Vic can engage with multiple users discussing the same game
- **Historical Analysis**: Compare current situations to similar historical scenarios
- **Prop Bet Specialty**: Advanced analysis for proposition betting opportunities

## 📄 License

MIT

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

For detailed technical information about extending the bot, see our [AI Documentation](./AI_DOCUMENTATION.md#extensibility-points). 