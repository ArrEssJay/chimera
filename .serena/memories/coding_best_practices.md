# Coding Workflow Principles
- Always gather context before editing: review relevant modules, configs, styles and docs via Serena tools (symbol overview, search, list_dir) to avoid blind refactors.
- Prefer incremental, well-scoped edits. Use symbolic tools to touch only the symbols that need changing and minimise unrelated diffs.
- After modifying runnable code, run the appropriate checks immediately (`cargo test`, `cargo clippy`, `trunk build`) so regressions surface early.
- Commit frequently with descriptive messages summarising the technical intent and affected areas.
- Document key assumptions, trade-offs, and follow-ups in code comments, PR descriptions, or `docs/todo.md`.

# BDD & Verification Practices
- Capture new behaviour first in a user-facing scenario (Gherkin-style Given/When/Then) within the `docs/bdd-guidelines.md` canon; ensure acceptance tests mirror these flows.
- Keep scenarios focused on one behaviour; factor shared Given steps into helpers to prevent duplication.
- Align unit/integration tests with the BDD narratives: scenario -> integration test in `chimera-core/tests` or `chimera-web/tests`, step -> helper functions.
- Treat acceptance tests as living documentation: update them whenever protocol rules, UI flows, or diagnostics change.
- Automate the BDD suite inside CI so red scenarios block merges.

# Modern Development Habits
- Enforce formatting (`cargo fmt`), linting (`cargo clippy --all-targets --all-features`), and security/static checks in CI before merging.
- Gate risky features behind compile-time flags or runtime toggles so we can ship safely and roll out gradually.
- Optimise based on measurement: profile (`cargo bench`, `criterion`, `cargo flamegraph`) before attempting performance fixes.
- Prioritise observability: extend structured logging (`tracing`, `log`) and diagnostics surfaces in both CLI and web UI when adding features.
- Maintain backward-compatible public APIs; if a breaking change is necessary, document migration steps and bump versions as per semver.
