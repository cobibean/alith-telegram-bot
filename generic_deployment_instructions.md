# Generic Deployment Instructions for Vic

This guide provides instructions for deploying Vic (AI-powered Telegram bot) on any server. While the examples use Ubuntu/Debian, the steps can be adapted for other Linux distributions.

## Prerequisites

- A server with SSH access (cloud provider like Digital Ocean, AWS, GCP, or your own server)
- A Telegram bot token (obtained from @BotFather on Telegram)
- An OpenAI API key

## 1. Connect to Your Server

Using SSH:

```bash
ssh username@your_server_ip_address
```

## 2. Update System Packages

For Ubuntu/Debian:
```bash
sudo apt update && sudo apt upgrade -y
```

For CentOS/RHEL:
```bash
sudo yum update -y
```

## 3. Install Required System Software

### Node.js

For Ubuntu/Debian:
```bash
# Install Node.js repository
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -

# Install Node.js and npm
sudo apt-get install -y nodejs
```

For CentOS/RHEL:
```bash
# Install Node.js repository
curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -

# Install Node.js and npm
sudo yum install -y nodejs
```

### Git

For Ubuntu/Debian:
```bash
sudo apt install git -y
```

For CentOS/RHEL:
```bash
sudo yum install git -y
```

### Build Essentials and Rust

For Ubuntu/Debian:
```bash
sudo apt install -y build-essential
```

For CentOS/RHEL:
```bash
sudo yum group install "Development Tools" -y
```

Install Rust (universal):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## 4. Install PM2 (Process Manager)

```bash
# Install PM2 globally
sudo npm install -g pm2
```

## 5. Clone and Build Required Repositories

```bash
# Clone the Alith SDK repository
git clone https://github.com/cobibean/alith.git
cd alith

# Build the Alith SDK
cargo build --release
cd sdks/node
npm install
npm run build

# Clone the Vic repository
cd ~/
git clone https://github.com/cobibean/alith-telegram-bot.git
cd alith-telegram-bot
```

## 6. Install and Configure Dependencies

```bash
# Install project dependencies
npm install

# Create the dist directory in the Alith module
mkdir -p node_modules/alith/dist

# Copy the native module to the correct location (adjust path if needed)
cp ~/alith/sdks/node/alith.linux-x64-gnu.node node_modules/alith/dist/
```

## 7. Create Environment File

```bash
cp .env.example .env
nano .env
```

Add the following content (replace with your actual values):

```
TELEGRAM_BOT_TOKEN=your_telegram_bot_token
OPENAI_API_KEY=your_openai_api_key
# Add other needed environment variables
```

Save and exit: `CTRL+X`, then `Y`, then `Enter`

## 8. Start the Bot with PM2

```bash
# Start the bot with PM2 in production mode
NODE_ENV=production pm2 start index.js --name "vic"

# Save the PM2 process list
pm2 save

# Set up PM2 to start on boot
pm2 startup
```

Follow the instructions output by the last command to make PM2 start automatically on system boot.

## 9. Monitor Your Bot

```bash
# Check status
pm2 status

# View logs
pm2 logs vic
```

## 10. Troubleshooting

If you encounter module loading issues:

1. Ensure the native module is in the correct location: `node_modules/alith/dist/alith.linux-x64-gnu.node`
2. Check file permissions: `chmod +x node_modules/alith/dist/alith.linux-x64-gnu.node`
3. For different architectures, you may need to rebuild the native module for your platform
4. Restart the PM2 process: `pm2 restart vic --update-env`

## 11. Security Recommendations

1. Create a non-root user for running the application
2. Set up a firewall (ufw for Ubuntu/Debian, firewalld for CentOS/RHEL)
3. Keep your system updated
4. Consider using SSH key authentication instead of passwords
5. Use HTTPS if exposing any web interfaces

Your bot should now be running and will automatically restart if it crashes or if the server reboots. 