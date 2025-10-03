# Branch Protection Rules

This document describes the branch protection rules that should be configured for the Chimera repository to ensure code quality and prevent breaking changes from being merged into the main branch.

## Required Configuration

To enable GitOps workflow, configure the following branch protection rules for the `main` branch:

### Navigate to Branch Protection Settings

1. Go to the repository on GitHub: https://github.com/ArrEssJay/chimera
2. Click on **Settings** → **Branches** → **Add branch protection rule**
3. Set **Branch name pattern** to `main`

### Required Status Checks

Enable **Require status checks to pass before merging** and select the following checks:

- ✅ **Run Tests** (`test`)
- ✅ **Check Formatting** (`fmt`)
- ✅ **Run Clippy** (`clippy`)
- ✅ **Build Web Dashboard** (`build-web`)
- ✅ **Test Deploy and Verify** (`test-deployment`)

These checks are defined in:
- `.github/workflows/ci.yml` - Runs tests, formatting, clippy, and web build
- `.github/workflows/test-deploy.yml` - Performs test deployment and verification

### Additional Recommended Settings

- ✅ **Require branches to be up to date before merging**
  - Ensures feature branches include the latest changes from main
  
- ✅ **Require linear history**
  - Maintains a clean git history
  
- ✅ **Include administrators**
  - Applies rules to repository administrators as well

### Optional Settings (Recommended for Team Workflows)

- **Require pull request reviews before merging**
  - Set required number of approving reviews: 1
  - Dismiss stale pull request approvals when new commits are pushed
  
- **Require conversation resolution before merging**
  - Ensures all review comments are addressed

## Workflow Overview

### For Feature Branches

1. Developer creates a feature branch from `main`
2. Developer makes changes and pushes to the feature branch
3. CI workflow runs automatically on push:
   - Runs all tests
   - Checks code formatting
   - Runs clippy lints
   - Builds web dashboard
4. Developer creates a pull request to `main`
5. Test deployment workflow runs:
   - Builds and verifies the web dashboard
   - Creates deployment artifacts
   - Posts verification status to PR
6. All status checks must pass before merge is allowed
7. Once approved and checks pass, PR can be merged to `main`

### For Main Branch

1. When changes are merged to `main`:
   - CI workflow runs for verification
   - Deploy workflow builds and deploys to GitHub Pages
   - Production site is updated automatically

## CI Workflows

### CI Workflow (`.github/workflows/ci.yml`)

Runs on:
- All pushes to any branch
- All pull requests to `main`

Jobs:
- **test**: Runs `cargo test --workspace --all-features`
- **fmt**: Checks formatting with `cargo fmt --all -- --check`
- **clippy**: Runs `cargo clippy --workspace --all-targets --all-features`
- **build-web**: Builds the web dashboard with trunk

### Test Deployment Workflow (`.github/workflows/test-deploy.yml`)

Runs on:
- Pull requests to `main`

Jobs:
- **test-deployment**: 
  - Builds web dashboard
  - Verifies build artifacts exist
  - Checks WASM files are generated
  - Validates HTML structure
  - Creates deployment summary
  - Posts status comment on PR

### Deploy Workflow (`.github/workflows/deploy.yml`)

Runs on:
- Pushes to `main`
- Manual workflow dispatch

Jobs:
- **check-ci**: Verifies CI has passed
- **build**: Builds the web dashboard
- **deploy**: Deploys to GitHub Pages

## Local Development

Before pushing changes, run the following checks locally:

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --workspace --all-targets --all-features

# Run tests
cargo test --workspace --all-features

# Build web dashboard
cd chimera-web
trunk build --release
```

## Troubleshooting

### Status Checks Not Appearing

If the status checks don't appear in the branch protection settings:
1. Ensure the workflows have run at least once on a pull request
2. Wait a few minutes for GitHub to register the checks
3. Refresh the branch protection settings page

### Build Failures

If the web dashboard build fails:
1. Check that the `wasm32-unknown-unknown` target is installed locally
2. Verify trunk is installed: `cargo install trunk --locked`
3. Try building locally to reproduce the issue
4. Check the GitHub Actions logs for detailed error messages

### Test Failures

If tests fail in CI but pass locally:
1. Ensure you're testing with the same feature flags: `--all-features`
2. Check for race conditions in tests
3. Verify all dependencies are properly specified in `Cargo.toml`

## References

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [GitHub Actions Status Checks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/about-status-checks)
