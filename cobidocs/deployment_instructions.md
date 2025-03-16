# Deployment Instructions for Vic on Digital Ocean

Follow these steps to deploy Vic (AI-powered Telegram bot) on your Digital Ocean droplet.

## 1. Connect to Your Droplet

After creating your droplet with SSH keys, connect to it:

```bash
ssh root@your_droplet_ip_address
```

## 2. Update System Packages

Use the apt package manager (Ubuntu/Debian system package manager) to update your system:

```bash
apt update && apt upgrade -y
```

## 3. Install Required System Software

Use apt to install system requirements:

```bash
# Install Node.js repository
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -

# Install Node.js and npm using apt (system package manager)
apt-get install -y nodejs

# Install Git using apt
apt install git -y

# Install build essentials and Rust (required for Alith SDK)
apt install -y build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y
source $HOME/.cargo/env
```

## 4. Install PM2 (Process Manager) using npm

Now that Node.js and npm are installed, use npm to install the PM2 process manager:

```bash
# Install PM2 globally using npm (Node.js package manager)
npm install -g pm2
```

## 5. Clone Required Repositories

```bash
# Clone the Alith SDK repository (required for native module)
git clone https://github.com/cobibean/alith.git
cd alith

# Build the Alith SDK
cargo build --release
cd sdks/node
npm install
npm run build

# Clone the Vic repository
cd /root
git clone https://github.com/cobibean/alith-telegram-bot.git
cd alith-telegram-bot
```

## 6. Install and Configure Dependencies

Use npm to install the project dependencies and configure the Alith SDK:

```bash
# Install project dependencies using npm
npm install

# Create the dist directory in the Alith module
mkdir -p node_modules/alith/dist

# Copy the native module to the correct location
cp /root/alith/sdks/node/alith.linux-x64-gnu.node node_modules/alith/dist/
```

## 7. Create Environment File

Create a `.env` file with your configuration:

```bash
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

## 10. Verify Native Module Loading

Check the logs to ensure the Alith native module is loading correctly:

```bash
# The logs should show successful message processing
# If you see module loading errors, verify:
ls -l node_modules/alith/dist/alith.linux-x64-gnu.node
```

## 11. Set Up a Firewall (Optional but Recommended)

Use Ubuntu's uncomplicated firewall (ufw) to secure your server:

```bash
# Allow SSH, HTTP, and HTTPS
ufw allow ssh
ufw allow http
ufw allow https

# Enable the firewall
ufw enable
```

## Troubleshooting

If you encounter module loading issues:
1. Ensure the native module is in the correct location: `node_modules/alith/dist/alith.linux-x64-gnu.node`
2. Verify the system is using GNU libc (not musl): `ldd --version`
3. Check file permissions: `chmod +x node_modules/alith/dist/alith.linux-x64-gnu.node`
4. Restart the PM2 process: `pm2 restart vic --update-env`

Your bot should now be running and will automatically restart if it crashes or if the server reboots. 