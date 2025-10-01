# Behaviour-Driven Development Practices

To keep the Rust port aligned with the reference Python behaviour, we describe
new features using **Given/When/Then** phrasing and capture them in executable
tests before implementing the code. Each new capability should follow this
loop:

1. **Document the scenario** – outline the preconditions, action, and expected
   outcome in this guide or in feature-specific notes.
2. **Write an executable test** – encode the scenario with expressive test names
   (e.g. `given_small_payload_when_build_frames_then_single_frame_is_emitted`).
3. **Implement the feature** – only as much code as needed to satisfy the new
   test.
4. **Refactor confidently** – once the test passes, tidy the implementation
   while keeping the scenario tests green.

## Current Focus: Frame Construction

- *Given* a payload that fits in a single frame,
  *When* we build the Chimera frame stream,
  *Then* we emit exactly one frame containing the sync header, target id,
  command word with correct total frame count, and the encoded payload/ECC bits.

- *Given* a payload that spans multiple frames,
  *When* we build the frame stream,
  *Then* each frame carries its frame index in the command word and the overall
  frame count remains consistent.

These scenarios are tracked by integration tests in
`chimera-core/tests/encoder_acceptance.rs` and should remain green as we evolve
`encoder::build_frame_stream`.
