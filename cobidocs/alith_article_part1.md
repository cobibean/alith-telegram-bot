# Becoming a Based AI Agent Dev: Introduction to Alith for Web3

![Alith Logo](imgs/logo.png)

## Introduction: The AI Agent Revolution (That Actually Works On-Chain)

While normies are still struggling to explain AI to their parents, degens are building AI agents that trade, arbitrage, and shitpost for us 24/7. 

Welcome to the future, anon. Bienvenido, amigo

If you've been in crypto for more than a hot minute, you've probably noticed that everyone and their step-sister's dog is talking about AI agents. But here's the thing: **most of these so-called "agents" are just glorified chatbots** with a wallet address slapped on. They're about as useful for on-chain activities as a gas fee estimator during a Yuga Labs NFT drop in 2020...useless when you actually need them.

Enter **Alith** -- the missing link between AI and Web3 that actually f*****g works.

I first stumbled across Alith at ETH Denver this year. While most people were busy aping into whatever token was being shilled at the booths or loading up their bags with as much free swag as they could fit, I was talking to the LazAI team about their new framework. What caught my attention wasn't just another AI buzzword salad (for real), it was the fact that they were building something that could actually interact with blockchains in a meaningful way.

The problem with most existing AI agent frameworks is threefold:
1. They're complex as f**k (unless you have a PhD in ML)
2. They're not Web3 native (good luck getting them to understand what a mempool is)
3. They have a barrier to entry higher than getting into [YC](https://www.ycombinator.com/)

**Here's the kicker though** - I'm a rookie dev who spends way too much time on-chain, not some AI researcher with three PhDs. And even I managed to build something useful with Alith in a weekend. That's the whole point.

## What the F*ck is Alith Anyway?

At its core, Alith is an AI agent framework designed specifically for Web3. It's like the lovechild of OpenAI and MetaMask – combining the power of large language models with the ability to interact on-chain. (or not btw...you can build agents for non-crypto things too)

Here's what makes Alith stand out from the sea of AI bullshit:

### Multiple Model Support
Unlike other frameworks that lock you into a single model, Alith supports everything from Llama and Grok to OpenAI and Anthropic. This means you can use the best model for the job, or even switch between them depending on your needs.

```javascript
// Using GPT-4
const agent = new Agent({
  name: 'GPT Agent',
  model: 'gpt-4',
  preamble: 'You are a helpful assistant.' //you can put whatever "preamble" you want here
});

// Using Claude
const claudeAgent = new Agent({
  name: 'Claude Agent',
  model: 'claude-3-opus',
  preamble: 'You are a helpful assistant.'
});
```

### Cross-Language Support
If you're a Rust chad, a Python enj0yer, or a JavaScript bufficorn like me, Alith has you covered. The framework provides SDKs for all three languages, making it accessible to developers of all backgrounds.

```python
# Python example
from alith import Agent

agent = Agent(
    name="Python Agent",
    model="gpt-4",
    preamble="You are a helpful assistant."
)
```

```javascript
// JavaScript example
const { Agent } = require('alith');

const agent = new Agent({
  name: 'JS Agent',
  model: 'gpt-4',
  preamble: 'You are a helpful assistant.'
});
```

### Web3 Integration Out of the Box
This is where Alith really shines. The framework comes with built-in tools for interacting with blockchains, making it trivial to build agents that can check balances, monitor transactions, and even execute trades.

```javascript
const agent = new Agent({
  name: 'Web3 Agent',
  model: 'gpt-4',
  preamble: 'You are a Web3 assistant.',
  tools: [
    {
      name: 'getEthBalance',
      description: 'Get the ETH balance of an address',
      parameters: {
        type: 'object',
        properties: {
          address: {
            type: 'string',
            description: 'The Ethereum address'
          }
        },
        required: ['address']
      },
      handler: async (params) => {
        // Web3 integration code here
        return balance;
      }
    }
  ]
});
```

### High-Performance Inference
Built on Rust, Alith leverages performance advantages like graph optimization, model compression, and JIT/AOT compilation with GPU coprocessors. In plain English: it's fast as f**k, which matters when milliseconds can be the difference between catching a celebrity token pump or watching from the sidelines as your broke friends somehow make 20x. wen $HAWK2AH ?

### How Does Alith Compare to Other Frameworks?

Let's be real...there are other AI agent frameworks out there. Eliza, AutoGPT, LangChain, they all claim to do similar things. So why should you give a shit about Alith?

Here's my spicy take:

**Eliza**: A solid framework for building chatbots that can talk about blockchain. And guess what? Alith actually integrates Eliza right out of the box in its `/integrations` folder. So you get all the good parts without having to deal with the bloat.

**AutoGPT**: More focused on autonomous agents that can perform tasks, but lacks the Web3-specific tools and optimizations that make Alith shine.

**LangChain**: A solid framework, but it's like trying to use a Swiss Army knife when what you really need is a chainsaw. It's not built specifically for Web3, and it shows. Also out of the box included in its '/integrations' folder. 

Alith, on the other hand, was built from the ground up with Web3 in mind.

## Getting Started: Your First Alith Agent

Enough theory – let's build something. I'm going to walk you through creating your first Alith agent, and I promise it's easier than explaining to my grandma that jpegs of monkeys are worth so much money.

### Setting Up Your Environment

First, you'll need to install the Alith SDK. I'll use JavaScript for this example since it's the most accessible, but remember that you can use Python or Rust if that's more your vibe.

```bash
# Create a new directory for your project
mkdir my-first-agent
cd my-first-agent

# Initialize a new Node.js project
npm init -y

# Install the Alith SDK
npm install alith
```

### Creating Your First Agent

Now, let's create a simple agent that can respond to prompts:

```javascript
// index.js
const { Agent } = require('alith');

// Initialize your agent
const agent = new Agent({
  name: 'Degen Assistant',
  model: 'gpt-4', // You can use other models too
  preamble: 'You are a degen crypto assistant. You speak in crypto slang, use emojis liberally, and always end your messages with "WAGMI" or "NGMI" depending on the context. You are knowledgeable about DeFi, NFTs, and trading.'
});

// Test your agent
async function main() {
  const response = await agent.prompt('What do you think about the current state of DeFi?');
  console.log(response);
}

main().catch(console.error);
```

Run this with `node index.js`, and you'll get a response from your very own degen AI assistant.

### Adding Tools to Your Agent

What makes Alith powerful is the ability to add custom tools that your agent can use. Let's add a simple tool that can check the price of ETH:

```javascript
const { Agent } = require('alith');
const axios = require('axios');

// Initialize your agent with tools
const agent = new Agent({
  name: 'Crypto Price Checker',
  model: 'gpt-4',
  preamble: 'You are a helpful assistant that can check cryptocurrency prices.',
  tools: [
    {
      name: 'getEthPrice',
      description: 'Get the current price of Ethereum in USD',
      parameters: {
        type: 'object',
        properties: {},
        required: []
      },
      handler: async () => {
        try {
          const response = await axios.get('https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd');
          return `The current price of ETH is $${response.data.ethereum.usd}`;
        } catch (error) {
          return `Error fetching ETH price: ${error.message}`;
        }
      }
    }
  ]
});

async function main() {
  const response = await agent.prompt('What is the current price of Ethereum?');
  console.log(response);
}

main().catch(console.error);
```

This agent can now check the price of ETH in real-time, which is a simple example but shows how you can extend your agent's capabilities.

### Creating a Telegram Bot

Let's take it a step further and create a Telegram bot using Alith, similar to what I did with the testBot, I made in 1 night (i'm not even that gud, you can use cursor w/ claude 3.7 and you'll be fine):

```javascript
require('dotenv').config();
const { Agent } = require('alith');
const { Telegraf } = require('telegraf');

// Initialize your Alith Agent
const agent = new Agent({
  name: 'Telegram Bot Agent',
  model: 'gpt-4',
  preamble: 'You are a helpful crypto assistant. You provide information about cryptocurrencies, DeFi, and blockchain technology.'
});

// Initialize Telegram Bot
const bot = new Telegraf(process.env.TELEGRAM_BOT_TOKEN);

// Handle /start command
bot.start((ctx) => {
  ctx.reply('Welcome to the Crypto Assistant Bot! Ask me anything about crypto.');
});

// Handle messages
bot.on('text', async (ctx) => {
  try {
    const userMessage = ctx.message.text;
    
    // Use the agent to generate a response
    const response = await agent.prompt(userMessage);
    
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
```

Save this to a file, create a `.env` file with your Telegram bot token (use Bot Father to get one), install the required packages (`npm install alith telegraf dotenv`), and run it. Boom – you've got a crypto assistant bot on Telegram.

Even if you think coding is just pressing keys randomly until something works, you can build an Alith agent. The framework is designed to be accessible to developers of all skill levels...complete beginners to 10x pro dev chads.

## Conclusion

In this first part of my series, we've covered what Alith is, why it could be a tool for Web3 developers, and how to build your first agent. We've seen how easy it is to get started, even if you're not a coding wizard.

### "But I'm Scared of Coding..."

Look, I get it. Not everyone wants to dive into code, even if it's relatively simple. The good news? There are already "AAAS" (Agents as a Service) products out there that let you use pre-built agents without touching a line of code. And I'd bet my last .1 METIS that someone is already working on a no-code UI for Alith that'll let you build custom agents with just a few clicks.

So if you're not ready to code yet, just keep your eyes peeled. In the meantime, encourage the builders around you. And hey, maybe one day you'll get curious enough to step out of your comfort zone and download Cursor or VS Code... they're free, after all. The barrier to entry has never been lower.

### What's Next?

In the next part, we'll dive into more advanced use cases, focusing on building agents that can actually make you money. We'll explore MEV bounty hunters, NFT/token alpha detectors, and yield optimizers – the kind of agents that can turn your passive crypto holdings into active income streams.

Until then, keep building, stay degen, and remember: your AI agents are only as smart as the tools you give them.

---

*This article is part 1 of a 3-part series on building AI agents with Alith. Stay tuned for parts 2 and 3, where we'll explore profitable use cases and the future of AI agents on Metis Hyperion.*

### Useful Links

- Follow me on Twitter: [@YourTwitterHandle](https://twitter.com/YourTwitterHandle)
- Alith Twitter: [@0xalith](https://x.com/0xalith)
- LazAI Twitter: [@LazAINetwork](https://twitter.com/LazAINetwork)
- Alith GitHub Repository: [github.com/0xLazAI/alith](https://github.com/0xLazAI/alith) 