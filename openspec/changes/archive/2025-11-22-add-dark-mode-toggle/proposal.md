# Change: Add Dark Mode Toggle

## Why
Users may prefer a dark interface in low-light environments. Providing a toggle allows users to customize their viewing experience.

## What Changes
- Implement a `ThemeProvider` context to manage the application's theme state (light, dark, system).
- Create a `ModeToggle` component (likely a dropdown or button) to switch themes.
- Update the `Dashboard` to include the `ModeToggle`.
- Ensure TailwindCSS is configured correctly for class-based dark mode (already present, but verified).

## Impact
- **Affected specs**: `frontend` capability.
- **Affected code**:
    - `frontend/src/components/theme-provider.tsx` (New)
    - `frontend/src/components/mode-toggle.tsx` (New)
    - `frontend/src/App.tsx`
    - `frontend/src/components/Dashboard.tsx`
