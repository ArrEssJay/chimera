# Copilot Instructions for Chimera DSP Development

## 1. Understand the System Before Editing
- Skim the relevant modules (`chimera-core/src`) and docs (`docs/chimera_technical_overview.md`) before making changes.
- Use Serena tools (`list_dir`, `find_symbol`, `think_about_task_adherence`) to gather context and avoid broad edits.
- Record important decisions, assumptions, and TODOs in code comments or the docs folder.

## 2. Coding Style & Structure
- Follow idiomatic Rust patterns: prefer declarative code, limit `unsafe`, and leverage iterators where possible.
- Keep modules cohesive: encoder/decoder logic goes in `chimera-core`, UI logic in `chimera-web`, tooling in `chimera-cli`.
- Use descriptive names for DSP-specific constructs (e.g., `carrier_freq_hz`, `ldpc_matrices`).
- Organize configuration via `config.rs` structures and prefer builder-like helpers for complex setups.

## 3. Performance-Critical Practices
- Minimize heap allocations inside inner loops; reuse buffers and prefer stack arrays where sizes are known.
- Exploit data-oriented design: operate on slices and iterators, avoid per-sample dynamic dispatch.
- Consider SIMD (`packed_simd`, `std::arch`) or `rayon` for parallelism when profiling shows hot paths.
- Avoid panics in DSP code; return `Result`/`Option` and handle errors at the boundary.
- Use `#[inline]` sparingly—only for tight loops confirmed via profiling.

## 4. Numerical Stability & DSP Accuracy
- Document assumptions about sample rate, bit depth, and scaling at every stage.
- Clamp or normalize intermediate values to avoid overflow/underflow.
- Keep deterministic RNG seeds for reproducible simulations; expose seed overrides for experiments.
- Validate new algorithms against reference data or MATLAB/Python prototypes when possible.

## 5. Testing & Verification
- Add unit tests for math helpers and LDPC routines (`cargo test -p chimera-core`).
- Write integration tests that run full simulations (see `chimera-web/tests/pipeline.rs`).
- For new DSP blocks, create property-based tests (`proptest`) to stress edge cases.
- Before merging, run:
  ```bash
  cargo fmt
  cargo clippy --workspace --all-targets --all-features
  cargo test --workspace --all-features
  ```

## 6. Profiling & Benchmarking
- Use `cargo bench` or `criterion` benchmarks for hot paths (modulation, decoding, LDPC).
- Capture flamegraphs (e.g., `cargo flamegraph`) when optimizing: document findings in PR summaries.
- For WASM/Yew front-end, profile with browser performance tools and keep UI rendering lightweight.

## 7. Front-End (Yew/WebAssembly) Considerations
- Keep Yew components pure and derive `Properties` with `Clone + PartialEq` to reduce re-renders.
- Store large buffers in `Rc`/`Arc` to prevent cloning; pass slices to canvas drawing routines.
- Use `spawn_local` for async work; ensure error handling logs to `console` instead of panicking.
- **MANDATORY: All CSS colors MUST use LCH color space** - see SIGINT_THEME.md for details
  - ✅ Use: `lch(75% 65 140)`, `lch(92% 8 140 / 0.5)`
  - ❌ Never use: `#00ff00`, `rgb(200, 200, 200)`, `rgba(10, 10, 15, 0.9)`
  - Exception: `transparent` keyword is allowed
  - Reference CSS variables from `:root` whenever possible
- For Plotters charts in Rust: RGB colors are acceptable as they approximate LCH tactical colors
  - Document LCH → RGB mapping in comments when adding new colors
  - Example: `RGBColor(120, 220, 150) // Tactical green ≈ lch(75% 65 140)`

## 8. Diagnostics & Logging
- Extend `DiagnosticsBundle` for new insights; prefer structured logs (`tracing`, `log`) over `println!`.
- When adding logs, keep them concise and rate-limited to avoid overwhelming the UI/CLI outputs.
- Surface critical metrics (BER, frame errors, timing offsets) through both CLI and web UI.

## 9. Documentation & Knowledge Sharing
- Update README or docs with protocol or API changes immediately.
- Provide diagrams or markdown tables for new DSP flows, including signal path and configuration dependencies.
- Note open questions or future work in `docs/todo.md` so the team can track them.

## 10. Workflow Tips
- Commit frequently with context-rich messages (“Optimize LDPC parity check by reusing syndrome buffer”).
- Keep PRs focused; if refactors are required, land them separately from feature work.
- Consult the `coding_best_practices` memory for quick reminders on the expected workflow.
