# Chromatic Visual Regression Testing Setup

## Overview

Chromatic is integrated into the Chimera project to provide automated visual regression testing for React UI components. This ensures that UI changes are caught before they reach production.

## What is Chromatic?

Chromatic is a visual testing and review tool for Storybook. It:
- Captures pixel-perfect screenshots of all Storybook stories
- Compares new screenshots against baselines to detect visual changes
- Provides a web UI for reviewing and approving/rejecting changes
- Integrates with GitHub to block PRs with unapproved visual changes

## Setup Instructions

### 1. Initial Setup (Already Complete)

The following has been configured:

- ✅ Chromatic package installed (`chromatic` in `devDependencies`)
- ✅ GitHub Actions workflow created (`.github/workflows/chromatic.yml`)
- ✅ Storybook stories exist for UI components
- ✅ NPM script added (`npm run chromatic`)

### 2. GitHub Repository Configuration

**Required:** Add the `CHROMATIC_PROJECT_TOKEN` secret to the GitHub repository:

1. Go to https://www.chromatic.com/
2. Sign in with your GitHub account
3. Create a new project or select the existing Chimera project
4. Copy the project token
5. In GitHub repository settings:
   - Navigate to `Settings` → `Secrets and variables` → `Actions`
   - Click `New repository secret`
   - Name: `CHROMATIC_PROJECT_TOKEN`
   - Value: Paste the token from Chromatic
   - Click `Add secret`

### 3. Capturing Baseline Screenshots

Once the secret is configured, baselines will be captured automatically:

1. The first run on the `main` branch will establish baselines
2. Subsequent runs will compare against these baselines
3. Baselines on `main` are auto-accepted (configured in workflow)

## Workflow

### For Developers

When you create a PR with UI changes:

1. **Automated Testing:** 
   - Chromatic workflow runs automatically
   - Screenshots are captured and compared to baselines
   - Build link is posted in the PR checks

2. **Review Changes:**
   - Click the Chromatic build link in the PR checks
   - Review any detected visual changes
   - Changes are categorized as:
     - **New stories:** Components or stories that didn't exist before
     - **Changes:** Visual differences from the baseline
     - **Unchanged:** Stories that match the baseline

3. **Accept or Reject:**
   - If changes are intentional → Accept them in Chromatic UI
   - If changes are unintentional → Fix the code and push again
   - Once all changes are accepted → CI check passes

4. **Merge:**
   - After Chromatic check passes, PR can be merged
   - The new baselines become the reference for future PRs

### For Reviewers

1. Review code changes as normal
2. Check the Chromatic build link for visual changes
3. Verify that visual changes align with the PR intent
4. Approve changes in Chromatic if they look correct
5. Leave comments on the PR if visual changes are unexpected

## CI/CD Integration

### Workflow Configuration

File: `.github/workflows/chromatic.yml`

**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch

**Key Settings:**
- `exitZeroOnChanges: true` - Doesn't fail on visual changes (requires manual review)
- `exitOnceUploaded: true` - Exits after upload (faster feedback)
- `autoAcceptChanges: main` - Auto-accepts changes on main branch
- `fetch-depth: 0` - Full git history for accurate change detection

### Local Testing

Run Chromatic locally (requires project token):

```bash
cd chimera-web
export CHROMATIC_PROJECT_TOKEN=your_token_here
npm run chromatic
```

This will:
- Build Storybook
- Upload to Chromatic
- Compare against baselines
- Provide a URL to review changes

## Best Practices

### Writing Storybook Stories

For effective visual regression testing:

1. **Cover all component states:**
   ```tsx
   export const Default: Story = { args: { ... } };
   export const Loading: Story = { args: { loading: true } };
   export const Error: Story = { args: { error: true } };
   export const Disabled: Story = { args: { disabled: true } };
   ```

2. **Use consistent layouts:**
   ```tsx
   parameters: {
     layout: 'centered', // or 'padded', 'fullscreen'
   }
   ```

3. **Test responsive states:**
   ```tsx
   parameters: {
     viewport: {
       defaultViewport: 'mobile1',
     },
   }
   ```

4. **Avoid animations in stories:**
   - Animations can cause flaky tests
   - Use `prefers-reduced-motion` or disable animations

### Handling Changes

**When you see visual changes:**

1. **Expected changes** (new feature, intentional redesign):
   - Review the changes in Chromatic
   - Accept the changes
   - Document the change in PR description

2. **Unexpected changes** (regression, unintended side effect):
   - Investigate why the change occurred
   - Fix the issue
   - Push new code
   - Chromatic will re-test automatically

3. **False positives** (anti-aliasing, font rendering):
   - These are rare with Chromatic
   - If persistent, can adjust threshold or ignore specific elements

## Coverage and Metrics

### Current Visual Test Coverage

Chromatic automatically tests all Storybook stories:
- Button component: 8 stories (variants, sizes, states)
- Select component: Multiple stories
- Badge component: Multiple stories
- Panel component: Multiple stories
- Tooltip component: Multiple stories

### Coverage Goals

- ✅ All UI components have Storybook stories
- ✅ All component variants covered
- ✅ All interactive states covered
- ✅ Responsive breakpoints tested (future enhancement)
- ✅ Theme variations tested (future enhancement)

## Troubleshooting

### Build Fails in CI

**Issue:** Chromatic workflow fails
**Solution:** 
1. Check that `CHROMATIC_PROJECT_TOKEN` is set
2. Verify Storybook builds locally: `npm run build-storybook`
3. Check workflow logs for specific errors

### Visual Changes Not Detected

**Issue:** Expected visual changes don't show up in Chromatic
**Solution:**
1. Ensure the story is properly exported
2. Check that the story is in the correct directory (`src-react/**/*.stories.tsx`)
3. Verify Storybook configuration (`.storybook/main.ts`)

### Slow Build Times

**Issue:** Chromatic builds take too long
**Solution:**
- Currently set to `exitOnceUploaded: true` for faster feedback
- Chromatic processes screenshots asynchronously after upload
- Build times typically 2-5 minutes

## Resources

- **Chromatic Documentation:** https://www.chromatic.com/docs/
- **Storybook Documentation:** https://storybook.js.org/docs/
- **Chromatic GitHub Action:** https://github.com/chromaui/action

## Support

For issues with Chromatic integration:
1. Check this documentation first
2. Review Chromatic build logs in the PR
3. Check Chromatic support docs: https://www.chromatic.com/docs/
4. Open an issue in the repository with details
