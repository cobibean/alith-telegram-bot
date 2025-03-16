# Update Instructions for Vic

This guide explains how to update Vic on the Digital Ocean production server.

## Prerequisites

- SSH access to the server (see [droplet_access.md](./droplet_access.md))
- Git repository access
- Understanding of Node.js and PM2

## Update Process

### 1. Connect to the Server

```bash
ssh root@146.190.63.86
```

### 2. Update the Alith SDK (if needed)

If there are changes to the Alith SDK:

```bash
# Navigate to Alith SDK directory
cd /root/alith

# Pull latest changes
git pull origin main

# Rebuild the SDK
cargo build --release
cd sdks/node
npm install
npm run build

# Return to root
cd /root
```

### 3. Update the Telegram Bot

```bash
# Navigate to bot directory
cd /root/alith-telegram-bot

# Backup the .env file
cp .env .env.backup

# Pull latest changes
git pull origin main

# Restore the .env file
cp .env.backup .env

# Install any new dependencies
npm install

# If the Alith SDK was updated:
mkdir -p node_modules/alith/dist
cp /root/alith/sdks/node/alith.linux-x64-gnu.node node_modules/alith/dist/
```

### 4. Restart the Service

```bash
# Restart the PM2 process with updated environment
NODE_ENV=production pm2 restart vic --update-env

# Save the PM2 process list
pm2 save
```

### 5. Verify the Update

```bash
# Check the process status
pm2 status

# Monitor the logs for any errors
pm2 logs vic
```

## Rollback Process

If the update causes issues:

1. Stop the current process:
```bash
pm2 stop vic
```

2. Restore from backup:
```bash
cd /root/alith-telegram-bot
git reset --hard HEAD@{1}  # Go back to previous commit
cp .env.backup .env
npm install
```

3. Restart the service:
```bash
NODE_ENV=production pm2 restart vic --update-env
pm2 save
```

## Common Issues and Solutions

### Native Module Issues

If you see errors related to the native module:

1. Verify the module location:
```bash
ls -l node_modules/alith/dist/alith.linux-x64-gnu.node
```

2. Check file permissions:
```bash
chmod +x node_modules/alith/dist/alith.linux-x64-gnu.node
```

3. Rebuild and recopy if needed:
```bash
cd /root/alith/sdks/node
npm run build
cp alith.linux-x64-gnu.node /root/alith-telegram-bot/node_modules/alith/dist/
```

### PM2 Process Issues

If the PM2 process isn't starting correctly:

1. Check the logs:
```bash
pm2 logs vic
```

2. Try deleting and recreating the process:
```bash
pm2 delete vic
NODE_ENV=production pm2 start index.js --name vic
pm2 save
```

### Environment Issues

If environment variables aren't being picked up:

1. Verify the .env file:
```bash
cat .env
```

2. Restart with explicit environment update:
```bash
NODE_ENV=production pm2 restart vic --update-env
```

## Monitoring After Update

Always monitor the bot for a few minutes after an update to ensure everything is working correctly:

```bash
# Watch the logs
pm2 logs vic

# Check memory usage
pm2 monit
```

If you notice any issues, check the logs and consider rolling back if necessary.

## Handling Alith Repository Updates

### Major Updates and New Features

When the main Alith repository receives significant updates or new features:

1. First, backup the current working state:
```bash
# Backup Alith SDK
cd /root
cp -r alith alith.backup

# Backup Telegram Bot
cp -r alith-telegram-bot alith-telegram-bot.backup
```

2. Update and rebuild the Alith SDK:
```bash
# Navigate to Alith directory
cd /root/alith

# Stash any local changes if needed
git stash

# Switch to main branch
git checkout main

# Pull latest changes
git pull origin main

# Update Rust dependencies
cargo update

# Rebuild the entire project
cargo clean
cargo build --release

# Update and rebuild the Node.js SDK
cd sdks/node
npm install
npm run build

# Return to root
cd /root
```

3. Test the new build:
```bash
# Create a test directory
mkdir -p /root/alith-test
cd /root/alith-test

# Initialize a new Node.js project
npm init -y

# Install the local Alith SDK
npm install ../alith/sdks/node

# Create a test file
cat > test.js << 'EOL'
const { DelegateAgent } = require('alith');
const agent = new DelegateAgent();
console.log('Alith SDK loaded successfully');
EOL

# Run the test
node test.js
```

4. If the test succeeds, update the Telegram Bot:
```bash
cd /root/alith-telegram-bot

# Backup current node_modules
mv node_modules node_modules.backup

# Clean npm cache
npm cache clean --force

# Reinstall dependencies
npm install

# Copy new native module
mkdir -p node_modules/alith/dist
cp /root/alith/sdks/node/alith.linux-x64-gnu.node node_modules/alith/dist/

# Restart the service
NODE_ENV=production pm2 restart vic --update-env
```

### Breaking Changes

If the Alith SDK introduces breaking changes:

1. Check the changelog and migration guide:
```bash
cd /root/alith
git log --oneline
# Look for breaking changes in commit messages
```

2. Review API changes:
```bash
# Check the Node.js SDK documentation
cat /root/alith/sdks/node/README.md

# Review TypeScript definitions
cat /root/alith/sdks/node/index.d.ts
```

3. Update the Telegram Bot code:
```bash
cd /root/alith-telegram-bot

# Create a new branch for the update
git checkout -b feature/alith-update

# Make necessary code changes to accommodate breaking changes
# Edit files as needed...

# Test changes locally
NODE_ENV=development node index.js
```

4. Once changes are verified:
```bash
# Commit changes
git add .
git commit -m "Update: Adapt to Alith SDK breaking changes"

# Push to repository
git push origin feature/alith-update

# Switch back to main branch
git checkout main
```

### Rollback After Alith Update

If issues occur after updating the Alith SDK:

1. Stop the current process:
```bash
pm2 stop vic
```

2. Restore from backups:
```bash
# Restore Alith SDK
cd /root
rm -rf alith
mv alith.backup alith

# Restore Telegram Bot
rm -rf alith-telegram-bot
mv alith-telegram-bot.backup alith-telegram-bot

# Navigate to bot directory
cd alith-telegram-bot
```

3. Restart the service:
```bash
NODE_ENV=production pm2 restart vic --update-env
pm2 save
```

### Version Management

Keep track of working Alith SDK versions:

1. Create a version file:
```bash
# In the Telegram bot directory
cd /root/alith-telegram-bot
echo "ALITH_SDK_VERSION=$(cd /root/alith && git rev-parse HEAD)" >> .env
```

2. Document the version:
```bash
# Add to your deployment notes
echo "Last working Alith SDK commit: $(cat .env | grep ALITH_SDK_VERSION)" >> deployment.log
``` 