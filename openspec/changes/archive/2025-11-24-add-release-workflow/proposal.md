# Change: Add Release Workflow

## Why
To separate the build process from execution and ensure stable versioning, the user requested a workflow that builds the binary on tag creation and uploads it to GitHub Releases. The bot execution workflow should then download this pre-built binary.

## What Changes
- Create `.github/workflows/release.yml`:
    - Triggers on tag creation (e.g., `v*`).
    - Builds the Rust binary (`release` profile).
    - Creates a GitHub Release with the tag name.
    - Uploads the binary as an asset to the release.
- Update `.github/workflows/bot.yml`:
    - Remove build steps.
    - Add a step to download the binary from the latest GitHub Release.
    - Execute the downloaded binary.

## Impact
- Affected specs: `bot`
- Affected code: `.github/workflows/`
