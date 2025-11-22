## Context
The project requires a web interface to present record data and potentially interact with the blockchain. The user has specified a modern React stack.

## Goals / Non-Goals
- **Goals**:
    - Create a performant SPA using Vite v7.
    - Provide data visualization using Apache ECharts.
    - Enable blockchain interaction using Wagmi and Viem.
    - Automate deployment to GitHub Pages.
- **Non-Goals**:
    - Backend development (the app will rely on static data or direct blockchain queries).

## Decisions
- **Decision**: Use Vite v7 with React.
    - **Rationale**: Requested by user, offers fast build times and modern features.
- **Decision**: Use GitHub Pages for hosting.
    - **Rationale**: Free, integrated with GitHub, suitable for SPAs (with hash routing or 404 hack).
- **Decision**: Use `shadcn/ui` for components.
    - **Rationale**: Provides accessible, customizable components with TailwindCSS.

## Risks / Trade-offs
- **Risk**: GitHub Pages client-side routing issues.
    - **Mitigation**: Use HashRouter or a 404 redirect script if using BrowserRouter.

## Migration Plan
- N/A (New feature)
