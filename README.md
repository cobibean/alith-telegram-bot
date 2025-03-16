# Vic

A sophisticated AI-powered Telegram bot that integrates the Alith AI system with the Telegram messaging platform. This bot serves as an intelligent interface between users and the Alith AI system, providing advanced natural language processing capabilities through a familiar messaging interface.

## 🌟 Features

- **AI Interaction**: Advanced natural language processing powered by Alith AI
- **Context Awareness**: Maintains conversation context and memory
- **Native Performance**: Uses Rust-based Alith SDK for optimal speed
- **Extensible**: Modular design for easy feature additions
- **Production Ready**: Full deployment and monitoring support
- **Secure**: Built-in security features and input validation

## 🚀 Quick Start

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

2. **Installation**
   ```bash
   # Clone repositories
   git clone https://github.com/cobibean/alith.git
   git clone https://github.com/cobibean/alith-telegram-bot.git

   # Build Alith SDK
   cd alith
   cargo build --release
   cd sdks/node
   npm install
   npm run build
   cd ../../..

   # Setup Vic
   cd alith-telegram-bot
   npm install
   ```

3. **Configuration**
   ```bash
   # Create and edit .env file
   cp .env.example .env
   nano .env

   # Required environment variables:
   # TELEGRAM_BOT_TOKEN=your_telegram_bot_token
   # OPENAI_API_KEY=your_openai_api_key
   ```

4. **Running**
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

## 🚀 Future Development

See our [AI Documentation](./AI_DOCUMENTATION.md#future-enhancements) for planned features:
- Multi-modal input processing
- Voice message support
- Image generation
- Blockchain integration
- Enhanced AI capabilities

## 📄 License

MIT

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

For detailed technical information about extending the bot, see our [AI Documentation](./AI_DOCUMENTATION.md#extensibility-points). 