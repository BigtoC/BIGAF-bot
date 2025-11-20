# Change: Add GitHub Workflow

## Why
The project requires automated execution on a schedule (cron) as specified in `project.md`. Currently, the bot can only be run locally.

## What Changes
- Add a GitHub Actions workflow file `.github/workflows/bot.yml`.
- Configure the workflow to run on a schedule (e.g., every hour).
- Configure the workflow to use the `PRIVATE_KEY` secret.
- Ensure the workflow commits the updated `last_action_rate.txt` back to the repository.

## Impact
- Affected specs: `bot`
- Affected code: `.github/workflows/`
