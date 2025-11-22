# Change: Add Frontend Dashboard

## Why
Currently, the bot operates in the background, and its status is only visible through logs or checking the `last_action_rate.txt` file. A frontend dashboard is needed to provide a user-friendly interface for monitoring the bot's activity, viewing exchange rates, and potentially interacting with the contracts via a wallet connection.

## What Changes
- Initialize a new Single Page Application (SPA) in the `frontend/` directory.
- Use a modern tech stack: Vite v7, React, shadcn/ui, TanStack Query, Apache ECharts, TailwindCSS, Storybook, Wagmi, and Viem.
- Implement a basic dashboard to visualize data.
- Add a GitHub Actions workflow to automatically build and deploy the SPA to GitHub Pages whenever changes are pushed to the `frontend/` directory on the `main` branch.

## Impact
- **Affected specs**: Adds a new `frontend` capability.
- **Affected code**:
    - New `frontend/` directory containing the SPA source code.
    - New `.github/workflows/deploy-frontend.yml` workflow file.
