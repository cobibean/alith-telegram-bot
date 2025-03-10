# Deployment Instructions for Alith Telegram Bot on Digital Ocean

Follow these steps to deploy the Alith Telegram Bot on your Digital Ocean droplet.

## 1. Connect to Your Droplet

After creating your droplet with SSH keys, connect to it:

```bash
ssh root@YOUR_DROPLET_IP
```

## 2. Update System Packages

```bash
apt update && apt upgrade -y
```

## 3. Install Required Software

Install Node.js, Git, and PM2:

```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
apt-get install -y nodejs

# Install Git
apt install git -y

# Install PM2 (process manager)
npm install -g pm2
```

## 4. Clone the Repository

```bash
# Clone the repository
git clone https://github.com/cobibean/alith-telegram-bot.git
cd alith-telegram-bot
```

## 5. Install Dependencies

```bash
npm install
```

## 6. Create Environment File

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

## 7. Start the Bot with PM2

```bash
# Start the bot
pm2 start index.js --name "alith-telegram-bot"

# Save the PM2 process list
pm2 save

# Set up PM2 to start on boot
pm2 startup
```

Follow the instructions output by the last command to make PM2 start automatically on system boot.

## 8. Monitor Your Bot

```bash
# Check status
pm2 status

# View logs
pm2 logs alith-telegram-bot
```

## 9. Set Up a Firewall (Optional but Recommended)

```bash
# Allow SSH, HTTP, and HTTPS
ufw allow ssh
ufw allow http
ufw allow https

# Enable the firewall
ufw enable
```

Your bot should now be running and will automatically restart if it crashes or if the server reboots. 