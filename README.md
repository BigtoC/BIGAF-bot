# BIGAF-bot
Have fun!

[![Release](https://github.com/BigtoC/BIGAF-bot/actions/workflows/release.yaml/badge.svg)](https://github.com/BigtoC/BIGAF-bot/actions/workflows/release.yaml)
[![GAF Vault Bot](https://github.com/BigtoC/BIGAF-bot/actions/workflows/bot.yaml/badge.svg)](https://github.com/BigtoC/BIGAF-bot/actions/workflows/bot.yaml)

## GitHub Actions Setup

To run this bot via GitHub Actions, you must configure the following secrets in your repository settings:

- `PRIVATE_KEY`: The private key of the wallet that will execute the transactions. 
- `MAX_DEPOSIT_RATE`: The maximum exchange rate threshold to trigger deposits (e.g., 2.2). 
- `MIN_WITHDRAW_RATE`: The minimum exchange rate threshold to trigger withdrawals (e.g., 1.8).

Optional secrets/variables:
- `RPC_URL`: Custom RPC URL (defaults to Mantra Chain mainnet).
- `ACTION_AMOUNT_CONTROL`: Control the transaction amount (1 = full, <1 = percentage, >1 = fixed amount).

Project Initialized with [OpenSpec](https://openspec.dev).
```shell
openspec init  

 ████   █████   ██████  ██  ██   █████  █████   ██████   █████
██  ██  ██  ██  ██      ███ ██  ██      ██  ██  ██      ██
██  ██  █████   █████   ██ ███   ████   █████   █████   ██
██  ██  ██      ██      ██  ██      ██  ██      ██      ██
 ████   ██      ██████  ██  ██  █████   ██      ██████   █████

Welcome to OpenSpec!

Step 3/3

Review selections
Press Enter to confirm or Backspace to adjust.

▌ GitHub Copilot
▌ OpenSpec structure created
▌ AI tools configured

✔ OpenSpec initialized successfully!

Tool summary:
▌ Root AGENTS.md stub created for other assistants
▌ Created: GitHub Copilot
▌ Skipped: Auggie, Claude Code, Cline, RooCode, CodeBuddy Code, CoStrict, Crush, Cursor, Factory Droid, Gemini CLI, OpenCode, Kilo Code, Qoder, Windsurf, Codex, Amazon Q Developer, and Qwen Code

Use `openspec update` to refresh shared OpenSpec instructions in the future.

Next steps - Copy these prompts to GitHub Copilot:
────────────────────────────────────────────────────────────
1. Populate your project context:
   "Please read openspec/project.md and help me fill it out
    with details about my project, tech stack, and conventions"

2. Create your first change proposal:
   "I want to add [YOUR FEATURE HERE]. Please create an
    OpenSpec change proposal for this feature"

3. Learn the OpenSpec workflow:
   "Please explain the OpenSpec workflow from openspec/AGENTS.md
    and how I should work with you on this project"
────────────────────────────────────────────────────────────
```
